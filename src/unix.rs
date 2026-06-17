//! Unix-specific items: load-time `dlsym` symbol resolution and the platform
//! `SysIOVec`.
//!
//! The `enif_*` symbols are not linked; [`init`] resolves each one via
//! `dlsym(RTLD_DEFAULT, ...)` and stores the `Api` table.

use std::ffi::{c_void, CStr};

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

    unsafe fn load<T>(name: &'static CStr) -> Result<T, &'static str> {
        assert!(
            std::mem::size_of::<T>() == std::mem::size_of::<*mut c_void>(),
            "load<T>: T must be a function pointer"
        );
        let sym = unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr()) };
        if sym.is_null() {
            return Err(name.to_str().unwrap_or("<invalid utf8>"));
        }
        Ok(unsafe { std::mem::transmute_copy(&sym) })
    }

    // On a 64-bit target `long` is 64-bit, so the header aliases the int64
    // functions onto the `long` variants; load those names instead.
    #[cfg(target_pointer_width = "64")]
    let (s_get_int64, s_get_uint64, s_make_int64, s_make_uint64) = (
        c"enif_get_long",
        c"enif_get_ulong",
        c"enif_make_long",
        c"enif_make_ulong",
    );
    #[cfg(not(target_pointer_width = "64"))]
    let (s_get_int64, s_get_uint64, s_make_int64, s_make_uint64) = (
        c"enif_get_int64",
        c"enif_get_uint64",
        c"enif_make_int64",
        c"enif_make_uint64",
    );

    let api = unsafe {
        Api {
            // 0.1 / 1.0 core
            priv_data: load(c"enif_priv_data")?,
            alloc: load(c"enif_alloc")?,
            free: load(c"enif_free")?,
            is_atom: load(c"enif_is_atom")?,
            is_binary: load(c"enif_is_binary")?,
            is_ref: load(c"enif_is_ref")?,
            inspect_binary: load(c"enif_inspect_binary")?,
            alloc_binary: load(c"enif_alloc_binary")?,
            realloc_binary: load(c"enif_realloc_binary")?,
            release_binary: load(c"enif_release_binary")?,
            get_int: load(c"enif_get_int")?,
            get_ulong: load(c"enif_get_ulong")?,
            get_double: load(c"enif_get_double")?,
            get_list_cell: load(c"enif_get_list_cell")?,
            get_tuple: load(c"enif_get_tuple")?,
            is_identical: load(c"enif_is_identical")?,
            compare: load(c"enif_compare")?,
            make_binary: load(c"enif_make_binary")?,
            make_badarg: load(c"enif_make_badarg")?,
            make_int: load(c"enif_make_int")?,
            make_ulong: load(c"enif_make_ulong")?,
            make_double: load(c"enif_make_double")?,
            make_atom: load(c"enif_make_atom")?,
            make_existing_atom: load(c"enif_make_existing_atom")?,
            make_tuple: load(c"enif_make_tuple")?,
            make_list: load(c"enif_make_list")?,
            make_list_cell: load(c"enif_make_list_cell")?,
            make_string: load(c"enif_make_string")?,
            make_ref: load(c"enif_make_ref")?,

            // 1.0 thread primitives
            mutex_create: load(c"enif_mutex_create")?,
            mutex_destroy: load(c"enif_mutex_destroy")?,
            mutex_trylock: load(c"enif_mutex_trylock")?,
            mutex_lock: load(c"enif_mutex_lock")?,
            mutex_unlock: load(c"enif_mutex_unlock")?,
            cond_create: load(c"enif_cond_create")?,
            cond_destroy: load(c"enif_cond_destroy")?,
            cond_signal: load(c"enif_cond_signal")?,
            cond_broadcast: load(c"enif_cond_broadcast")?,
            cond_wait: load(c"enif_cond_wait")?,
            rwlock_create: load(c"enif_rwlock_create")?,
            rwlock_destroy: load(c"enif_rwlock_destroy")?,
            rwlock_tryrlock: load(c"enif_rwlock_tryrlock")?,
            rwlock_rlock: load(c"enif_rwlock_rlock")?,
            rwlock_runlock: load(c"enif_rwlock_runlock")?,
            rwlock_tryrwlock: load(c"enif_rwlock_tryrwlock")?,
            rwlock_rwlock: load(c"enif_rwlock_rwlock")?,
            rwlock_rwunlock: load(c"enif_rwlock_rwunlock")?,
            tsd_key_create: load(c"enif_tsd_key_create")?,
            tsd_key_destroy: load(c"enif_tsd_key_destroy")?,
            tsd_set: load(c"enif_tsd_set")?,
            tsd_get: load(c"enif_tsd_get")?,
            thread_opts_create: load(c"enif_thread_opts_create")?,
            thread_opts_destroy: load(c"enif_thread_opts_destroy")?,
            thread_create: load(c"enif_thread_create")?,
            thread_self: load(c"enif_thread_self")?,
            equal_tids: load(c"enif_equal_tids")?,
            thread_exit: load(c"enif_thread_exit")?,
            thread_join: load(c"enif_thread_join")?,

            // 1.0 / 2.0 core, resources, env, send
            realloc: load(c"enif_realloc")?,
            system_info: load(c"enif_system_info")?,
            fprintf: load(c"enif_fprintf")?,
            inspect_iolist_as_binary: load(c"enif_inspect_iolist_as_binary")?,
            make_sub_binary: load(c"enif_make_sub_binary")?,
            get_string: load(c"enif_get_string")?,
            get_atom: load(c"enif_get_atom")?,
            is_fun: load(c"enif_is_fun")?,
            is_pid: load(c"enif_is_pid")?,
            is_port: load(c"enif_is_port")?,
            get_uint: load(c"enif_get_uint")?,
            get_long: load(c"enif_get_long")?,
            make_uint: load(c"enif_make_uint")?,
            make_long: load(c"enif_make_long")?,
            make_tuple_from_array: load(c"enif_make_tuple_from_array")?,
            make_list_from_array: load(c"enif_make_list_from_array")?,
            is_empty_list: load(c"enif_is_empty_list")?,
            open_resource_type: load(c"enif_open_resource_type")?,
            alloc_resource: load(c"enif_alloc_resource")?,
            release_resource: load(c"enif_release_resource")?,
            make_resource: load(c"enif_make_resource")?,
            get_resource: load(c"enif_get_resource")?,
            sizeof_resource: load(c"enif_sizeof_resource")?,
            make_new_binary: load(c"enif_make_new_binary")?,
            is_list: load(c"enif_is_list")?,
            is_tuple: load(c"enif_is_tuple")?,
            get_atom_length: load(c"enif_get_atom_length")?,
            get_list_length: load(c"enif_get_list_length")?,
            make_atom_len: load(c"enif_make_atom_len")?,
            make_existing_atom_len: load(c"enif_make_existing_atom_len")?,
            make_string_len: load(c"enif_make_string_len")?,
            alloc_env: load(c"enif_alloc_env")?,
            free_env: load(c"enif_free_env")?,
            clear_env: load(c"enif_clear_env")?,
            send: load(c"enif_send")?,
            make_copy: load(c"enif_make_copy")?,
            self_: load(c"enif_self")?,
            get_local_pid: load(c"enif_get_local_pid")?,
            keep_resource: load(c"enif_keep_resource")?,
            make_resource_binary: load(c"enif_make_resource_binary")?,

            // 2.0 int64 (aliased to long on 64-bit, see above)
            get_int64: load(s_get_int64)?,
            get_uint64: load(s_get_uint64)?,
            make_int64: load(s_make_int64)?,
            make_uint64: load(s_make_uint64)?,

            // 2.2 – 2.4
            is_exception: load(c"enif_is_exception")?,
            make_reverse_list: load(c"enif_make_reverse_list")?,
            is_number: load(c"enif_is_number")?,
            dlopen: load(c"enif_dlopen")?,
            dlsym: load(c"enif_dlsym")?,
            consume_timeslice: load(c"enif_consume_timeslice")?,

            // 2.6 maps
            is_map: load(c"enif_is_map")?,
            get_map_size: load(c"enif_get_map_size")?,
            make_new_map: load(c"enif_make_new_map")?,
            make_map_put: load(c"enif_make_map_put")?,
            get_map_value: load(c"enif_get_map_value")?,
            make_map_update: load(c"enif_make_map_update")?,
            make_map_remove: load(c"enif_make_map_remove")?,
            map_iterator_create: load(c"enif_map_iterator_create")?,
            map_iterator_destroy: load(c"enif_map_iterator_destroy")?,
            map_iterator_is_head: load(c"enif_map_iterator_is_head")?,
            map_iterator_is_tail: load(c"enif_map_iterator_is_tail")?,
            map_iterator_next: load(c"enif_map_iterator_next")?,
            map_iterator_prev: load(c"enif_map_iterator_prev")?,
            map_iterator_get_pair: load(c"enif_map_iterator_get_pair")?,

            // 2.7 – 2.9
            schedule_nif: load(c"enif_schedule_nif")?,
            has_pending_exception: load(c"enif_has_pending_exception")?,
            raise_exception: load(c"enif_raise_exception")?,
            getenv: load(c"enif_getenv")?,

            // 2.10 time
            monotonic_time: load(c"enif_monotonic_time")?,
            time_offset: load(c"enif_time_offset")?,
            convert_time_unit: load(c"enif_convert_time_unit")?,

            // 2.11
            now_time: load(c"enif_now_time")?,
            cpu_time: load(c"enif_cpu_time")?,
            make_unique_integer: load(c"enif_make_unique_integer")?,
            is_current_process_alive: load(c"enif_is_current_process_alive")?,
            is_process_alive: load(c"enif_is_process_alive")?,
            is_port_alive: load(c"enif_is_port_alive")?,
            get_local_port: load(c"enif_get_local_port")?,
            term_to_binary: load(c"enif_term_to_binary")?,
            binary_to_term: load(c"enif_binary_to_term")?,
            port_command: load(c"enif_port_command")?,
            thread_type: load(c"enif_thread_type")?,
            snprintf: load(c"enif_snprintf")?,

            // 2.12
            select: load(c"enif_select")?,
            open_resource_type_x: load(c"enif_open_resource_type_x")?,
            monitor_process: load(c"enif_monitor_process")?,
            demonitor_process: load(c"enif_demonitor_process")?,
            compare_monitors: load(c"enif_compare_monitors")?,
            hash: load(c"enif_hash")?,
            whereis_pid: load(c"enif_whereis_pid")?,
            whereis_port: load(c"enif_whereis_port")?,

            // 2.13 I/O queue
            ioq_create: load(c"enif_ioq_create")?,
            ioq_destroy: load(c"enif_ioq_destroy")?,
            ioq_enq_binary: load(c"enif_ioq_enq_binary")?,
            ioq_enqv: load(c"enif_ioq_enqv")?,
            ioq_size: load(c"enif_ioq_size")?,
            ioq_deq: load(c"enif_ioq_deq")?,
            ioq_peek: load(c"enif_ioq_peek")?,
            inspect_iovec: load(c"enif_inspect_iovec")?,
            free_iovec: load(c"enif_free_iovec")?,

            // 2.14
            ioq_peek_head: load(c"enif_ioq_peek_head")?,
            mutex_name: load(c"enif_mutex_name")?,
            cond_name: load(c"enif_cond_name")?,
            rwlock_name: load(c"enif_rwlock_name")?,
            thread_name: load(c"enif_thread_name")?,
            vfprintf: load(c"enif_vfprintf")?,
            vsnprintf: load(c"enif_vsnprintf")?,
            make_map_from_arrays: load(c"enif_make_map_from_arrays")?,

            // 2.15
            select_x: load(c"enif_select_x")?,
            make_monitor_term: load(c"enif_make_monitor_term")?,
            set_pid_undefined: load(c"enif_set_pid_undefined")?,
            is_pid_undefined: load(c"enif_is_pid_undefined")?,
            term_type: load(c"enif_term_type")?,

            // 2.16
            #[cfg(feature = "nif_2_16")]
            init_resource_type: load(c"enif_init_resource_type")?,
            #[cfg(feature = "nif_2_16")]
            dynamic_resource_call: load(c"enif_dynamic_resource_call")?,

            // 2.17
            #[cfg(feature = "nif_2_17")]
            get_string_length: load(c"enif_get_string_length")?,
            #[cfg(feature = "nif_2_17")]
            make_new_atom: load(c"enif_make_new_atom")?,
            #[cfg(feature = "nif_2_17")]
            make_new_atom_len: load(c"enif_make_new_atom_len")?,
            #[cfg(feature = "nif_2_17")]
            set_option: load(c"enif_set_option")?,

            // 2.18
            #[cfg(feature = "nif_2_18")]
            term_size: load(c"enif_term_size")?,
            #[cfg(feature = "nif_2_18")]
            get_atom_cache_index: load(c"enif_get_atom_cache_index")?,
            #[cfg(feature = "nif_2_18")]
            max_atom_cache_index: load(c"enif_max_atom_cache_index")?,
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
