//! Unix-specific items: load-time `dlsym` symbol resolution and the platform
//! `SysIOVec`.
//!
//! The `enif_*` symbols are not linked; [`init`] resolves each one via
//! `dlsym(RTLD_DEFAULT, ...)` and stores the `Api` table.

use std::ffi::{c_char, c_void};

use crate::ffi::{Api, API};

/// Resolve every `enif_*` pointer via `dlsym(RTLD_DEFAULT, ...)` and store the
/// table globally. Idempotent.
///
/// Call exactly once from the NIF load entry point, before any other function
/// in this crate. On success returns `Ok(())`; on the first unresolved symbol
/// returns `Err(name)` and leaves the table uninitialized, so the caller must
/// propagate the failure to the BEAM (fail the load) rather than proceed.
///
/// # Safety
///
/// Must be called from the BEAM's NIF loading context.
pub unsafe fn init() -> Result<(), &'static str> {
    if API.get().is_some() {
        return Ok(());
    }

    unsafe fn load<T>(name: &[u8]) -> Result<T, &'static str> {
        assert!(
            std::mem::size_of::<T>() == std::mem::size_of::<*mut c_void>(),
            "load<T>: T must be a function pointer"
        );
        let sym = unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr() as *const c_char) };
        if sym.is_null() {
            // Trailing NUL stripped; the byte literals are 'static.
            let s = std::str::from_utf8(&name[..name.len() - 1]).unwrap_or("<invalid utf8>");
            return Err(unsafe { &*(s as *const str) });
        }
        Ok(unsafe { std::mem::transmute_copy(&sym) })
    }

    // On a 64-bit target `long` is 64-bit, so the header aliases the int64
    // functions onto the `long` variants; load those names instead.
    #[cfg(target_pointer_width = "64")]
    let (s_get_int64, s_get_uint64, s_make_int64, s_make_uint64) = (
        b"enif_get_long\0".as_ref(),
        b"enif_get_ulong\0".as_ref(),
        b"enif_make_long\0".as_ref(),
        b"enif_make_ulong\0".as_ref(),
    );
    #[cfg(not(target_pointer_width = "64"))]
    let (s_get_int64, s_get_uint64, s_make_int64, s_make_uint64) = (
        b"enif_get_int64\0".as_ref(),
        b"enif_get_uint64\0".as_ref(),
        b"enif_make_int64\0".as_ref(),
        b"enif_make_uint64\0".as_ref(),
    );

    let api = unsafe {
        Api {
            // 0.1 / 1.0 core
            priv_data: load(b"enif_priv_data\0")?,
            alloc: load(b"enif_alloc\0")?,
            free: load(b"enif_free\0")?,
            is_atom: load(b"enif_is_atom\0")?,
            is_binary: load(b"enif_is_binary\0")?,
            is_ref: load(b"enif_is_ref\0")?,
            inspect_binary: load(b"enif_inspect_binary\0")?,
            alloc_binary: load(b"enif_alloc_binary\0")?,
            realloc_binary: load(b"enif_realloc_binary\0")?,
            release_binary: load(b"enif_release_binary\0")?,
            get_int: load(b"enif_get_int\0")?,
            get_ulong: load(b"enif_get_ulong\0")?,
            get_double: load(b"enif_get_double\0")?,
            get_list_cell: load(b"enif_get_list_cell\0")?,
            get_tuple: load(b"enif_get_tuple\0")?,
            is_identical: load(b"enif_is_identical\0")?,
            compare: load(b"enif_compare\0")?,
            make_binary: load(b"enif_make_binary\0")?,
            make_badarg: load(b"enif_make_badarg\0")?,
            make_int: load(b"enif_make_int\0")?,
            make_ulong: load(b"enif_make_ulong\0")?,
            make_double: load(b"enif_make_double\0")?,
            make_atom: load(b"enif_make_atom\0")?,
            make_existing_atom: load(b"enif_make_existing_atom\0")?,
            make_tuple: load(b"enif_make_tuple\0")?,
            make_list: load(b"enif_make_list\0")?,
            make_list_cell: load(b"enif_make_list_cell\0")?,
            make_string: load(b"enif_make_string\0")?,
            make_ref: load(b"enif_make_ref\0")?,

            // 1.0 thread primitives
            mutex_create: load(b"enif_mutex_create\0")?,
            mutex_destroy: load(b"enif_mutex_destroy\0")?,
            mutex_trylock: load(b"enif_mutex_trylock\0")?,
            mutex_lock: load(b"enif_mutex_lock\0")?,
            mutex_unlock: load(b"enif_mutex_unlock\0")?,
            cond_create: load(b"enif_cond_create\0")?,
            cond_destroy: load(b"enif_cond_destroy\0")?,
            cond_signal: load(b"enif_cond_signal\0")?,
            cond_broadcast: load(b"enif_cond_broadcast\0")?,
            cond_wait: load(b"enif_cond_wait\0")?,
            rwlock_create: load(b"enif_rwlock_create\0")?,
            rwlock_destroy: load(b"enif_rwlock_destroy\0")?,
            rwlock_tryrlock: load(b"enif_rwlock_tryrlock\0")?,
            rwlock_rlock: load(b"enif_rwlock_rlock\0")?,
            rwlock_runlock: load(b"enif_rwlock_runlock\0")?,
            rwlock_tryrwlock: load(b"enif_rwlock_tryrwlock\0")?,
            rwlock_rwlock: load(b"enif_rwlock_rwlock\0")?,
            rwlock_rwunlock: load(b"enif_rwlock_rwunlock\0")?,
            tsd_key_create: load(b"enif_tsd_key_create\0")?,
            tsd_key_destroy: load(b"enif_tsd_key_destroy\0")?,
            tsd_set: load(b"enif_tsd_set\0")?,
            tsd_get: load(b"enif_tsd_get\0")?,
            thread_opts_create: load(b"enif_thread_opts_create\0")?,
            thread_opts_destroy: load(b"enif_thread_opts_destroy\0")?,
            thread_create: load(b"enif_thread_create\0")?,
            thread_self: load(b"enif_thread_self\0")?,
            equal_tids: load(b"enif_equal_tids\0")?,
            thread_exit: load(b"enif_thread_exit\0")?,
            thread_join: load(b"enif_thread_join\0")?,

            // 1.0 / 2.0 core, resources, env, send
            realloc: load(b"enif_realloc\0")?,
            system_info: load(b"enif_system_info\0")?,
            fprintf: load(b"enif_fprintf\0")?,
            inspect_iolist_as_binary: load(b"enif_inspect_iolist_as_binary\0")?,
            make_sub_binary: load(b"enif_make_sub_binary\0")?,
            get_string: load(b"enif_get_string\0")?,
            get_atom: load(b"enif_get_atom\0")?,
            is_fun: load(b"enif_is_fun\0")?,
            is_pid: load(b"enif_is_pid\0")?,
            is_port: load(b"enif_is_port\0")?,
            get_uint: load(b"enif_get_uint\0")?,
            get_long: load(b"enif_get_long\0")?,
            make_uint: load(b"enif_make_uint\0")?,
            make_long: load(b"enif_make_long\0")?,
            make_tuple_from_array: load(b"enif_make_tuple_from_array\0")?,
            make_list_from_array: load(b"enif_make_list_from_array\0")?,
            is_empty_list: load(b"enif_is_empty_list\0")?,
            open_resource_type: load(b"enif_open_resource_type\0")?,
            alloc_resource: load(b"enif_alloc_resource\0")?,
            release_resource: load(b"enif_release_resource\0")?,
            make_resource: load(b"enif_make_resource\0")?,
            get_resource: load(b"enif_get_resource\0")?,
            sizeof_resource: load(b"enif_sizeof_resource\0")?,
            make_new_binary: load(b"enif_make_new_binary\0")?,
            is_list: load(b"enif_is_list\0")?,
            is_tuple: load(b"enif_is_tuple\0")?,
            get_atom_length: load(b"enif_get_atom_length\0")?,
            get_list_length: load(b"enif_get_list_length\0")?,
            make_atom_len: load(b"enif_make_atom_len\0")?,
            make_existing_atom_len: load(b"enif_make_existing_atom_len\0")?,
            make_string_len: load(b"enif_make_string_len\0")?,
            alloc_env: load(b"enif_alloc_env\0")?,
            free_env: load(b"enif_free_env\0")?,
            clear_env: load(b"enif_clear_env\0")?,
            send: load(b"enif_send\0")?,
            make_copy: load(b"enif_make_copy\0")?,
            self_: load(b"enif_self\0")?,
            get_local_pid: load(b"enif_get_local_pid\0")?,
            keep_resource: load(b"enif_keep_resource\0")?,
            make_resource_binary: load(b"enif_make_resource_binary\0")?,

            // 2.0 int64 (aliased to long on 64-bit, see above)
            get_int64: load(s_get_int64)?,
            get_uint64: load(s_get_uint64)?,
            make_int64: load(s_make_int64)?,
            make_uint64: load(s_make_uint64)?,

            // 2.2 – 2.4
            is_exception: load(b"enif_is_exception\0")?,
            make_reverse_list: load(b"enif_make_reverse_list\0")?,
            is_number: load(b"enif_is_number\0")?,
            dlopen: load(b"enif_dlopen\0")?,
            dlsym: load(b"enif_dlsym\0")?,
            consume_timeslice: load(b"enif_consume_timeslice\0")?,

            // 2.6 maps
            is_map: load(b"enif_is_map\0")?,
            get_map_size: load(b"enif_get_map_size\0")?,
            make_new_map: load(b"enif_make_new_map\0")?,
            make_map_put: load(b"enif_make_map_put\0")?,
            get_map_value: load(b"enif_get_map_value\0")?,
            make_map_update: load(b"enif_make_map_update\0")?,
            make_map_remove: load(b"enif_make_map_remove\0")?,
            map_iterator_create: load(b"enif_map_iterator_create\0")?,
            map_iterator_destroy: load(b"enif_map_iterator_destroy\0")?,
            map_iterator_is_head: load(b"enif_map_iterator_is_head\0")?,
            map_iterator_is_tail: load(b"enif_map_iterator_is_tail\0")?,
            map_iterator_next: load(b"enif_map_iterator_next\0")?,
            map_iterator_prev: load(b"enif_map_iterator_prev\0")?,
            map_iterator_get_pair: load(b"enif_map_iterator_get_pair\0")?,

            // 2.7 – 2.9
            schedule_nif: load(b"enif_schedule_nif\0")?,
            has_pending_exception: load(b"enif_has_pending_exception\0")?,
            raise_exception: load(b"enif_raise_exception\0")?,
            getenv: load(b"enif_getenv\0")?,

            // 2.10 time
            monotonic_time: load(b"enif_monotonic_time\0")?,
            time_offset: load(b"enif_time_offset\0")?,
            convert_time_unit: load(b"enif_convert_time_unit\0")?,

            // 2.11
            now_time: load(b"enif_now_time\0")?,
            cpu_time: load(b"enif_cpu_time\0")?,
            make_unique_integer: load(b"enif_make_unique_integer\0")?,
            is_current_process_alive: load(b"enif_is_current_process_alive\0")?,
            is_process_alive: load(b"enif_is_process_alive\0")?,
            is_port_alive: load(b"enif_is_port_alive\0")?,
            get_local_port: load(b"enif_get_local_port\0")?,
            term_to_binary: load(b"enif_term_to_binary\0")?,
            binary_to_term: load(b"enif_binary_to_term\0")?,
            port_command: load(b"enif_port_command\0")?,
            thread_type: load(b"enif_thread_type\0")?,
            snprintf: load(b"enif_snprintf\0")?,

            // 2.12
            select: load(b"enif_select\0")?,
            open_resource_type_x: load(b"enif_open_resource_type_x\0")?,
            monitor_process: load(b"enif_monitor_process\0")?,
            demonitor_process: load(b"enif_demonitor_process\0")?,
            compare_monitors: load(b"enif_compare_monitors\0")?,
            hash: load(b"enif_hash\0")?,
            whereis_pid: load(b"enif_whereis_pid\0")?,
            whereis_port: load(b"enif_whereis_port\0")?,

            // 2.13 I/O queue
            ioq_create: load(b"enif_ioq_create\0")?,
            ioq_destroy: load(b"enif_ioq_destroy\0")?,
            ioq_enq_binary: load(b"enif_ioq_enq_binary\0")?,
            ioq_enqv: load(b"enif_ioq_enqv\0")?,
            ioq_size: load(b"enif_ioq_size\0")?,
            ioq_deq: load(b"enif_ioq_deq\0")?,
            ioq_peek: load(b"enif_ioq_peek\0")?,
            inspect_iovec: load(b"enif_inspect_iovec\0")?,
            free_iovec: load(b"enif_free_iovec\0")?,

            // 2.14
            ioq_peek_head: load(b"enif_ioq_peek_head\0")?,
            mutex_name: load(b"enif_mutex_name\0")?,
            cond_name: load(b"enif_cond_name\0")?,
            rwlock_name: load(b"enif_rwlock_name\0")?,
            thread_name: load(b"enif_thread_name\0")?,
            vfprintf: load(b"enif_vfprintf\0")?,
            vsnprintf: load(b"enif_vsnprintf\0")?,
            make_map_from_arrays: load(b"enif_make_map_from_arrays\0")?,

            // 2.15
            select_x: load(b"enif_select_x\0")?,
            make_monitor_term: load(b"enif_make_monitor_term\0")?,
            set_pid_undefined: load(b"enif_set_pid_undefined\0")?,
            is_pid_undefined: load(b"enif_is_pid_undefined\0")?,
            term_type: load(b"enif_term_type\0")?,

            // 2.16
            #[cfg(feature = "nif_2_16")]
            init_resource_type: load(b"enif_init_resource_type\0")?,
            #[cfg(feature = "nif_2_16")]
            dynamic_resource_call: load(b"enif_dynamic_resource_call\0")?,

            // 2.17
            #[cfg(feature = "nif_2_17")]
            get_string_length: load(b"enif_get_string_length\0")?,
            #[cfg(feature = "nif_2_17")]
            make_new_atom: load(b"enif_make_new_atom\0")?,
            #[cfg(feature = "nif_2_17")]
            make_new_atom_len: load(b"enif_make_new_atom_len\0")?,
            #[cfg(feature = "nif_2_17")]
            set_option: load(b"enif_set_option\0")?,

            // 2.18
            #[cfg(feature = "nif_2_18")]
            term_size: load(b"enif_term_size\0")?,
            #[cfg(feature = "nif_2_18")]
            get_atom_cache_index: load(b"enif_get_atom_cache_index\0")?,
            #[cfg(feature = "nif_2_18")]
            max_atom_cache_index: load(b"enif_max_atom_cache_index\0")?,
        }
    };

    // Another thread may have raced us; either way the table is now set.
    let _ = API.set(api);
    Ok(())
}

/// `SysIOVec` — iovec; on Unix this is `struct iovec`. NIF 2.13 (OTP 20.1).
#[repr(C)]
pub struct SysIOVec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}
