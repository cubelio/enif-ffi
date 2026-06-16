//! The `enif_*` function-pointer table and the load-time symbol resolver.
//!
//! The BEAM does not let a NIF library link against `enif_*` directly; the
//! symbols are resolved at load time. [`Api`] holds one pointer per
//! `enif_*` C function, and [`init`] fills it via `dlsym`.
//!
//! Field order is the canonical `erl_nif_api_funcs.h` declaration order — the
//! same order as the Windows `TWinDynNifCallbacks` struct — so this table can
//! double as that struct when Windows support lands. Each field is marked with
//! the NIF version that introduced it; because the C list is append-only, the
//! versions form contiguous bands, and the 2.16+ bands are feature-gated.

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::sync::OnceLock;

use crate::types::*;

// ---------------------------------------------------------------------------
// Function-pointer table
// ---------------------------------------------------------------------------

/// One pointer per `enif_*` function, in canonical declaration order.
///
/// All fields are plain `extern "C"` function pointers, so the table is `Sync`
/// and can live behind a `OnceLock`.
pub(crate) struct Api {
    // ── NIF 0.1 / 1.0 — initial term, binary, integer, atom, list/tuple core ──
    pub priv_data: unsafe extern "C" fn(*mut Env) -> *mut c_void, // 1.0
    pub alloc: unsafe extern "C" fn(usize) -> *mut c_void,        // 1.0
    pub free: unsafe extern "C" fn(*mut c_void),                  // 1.0
    pub is_atom: unsafe extern "C" fn(*mut Env, Term) -> c_int,   // 0.1
    pub is_binary: unsafe extern "C" fn(*mut Env, Term) -> c_int, // 0.1
    pub is_ref: unsafe extern "C" fn(*mut Env, Term) -> c_int,    // 0.1
    pub inspect_binary: unsafe extern "C" fn(*mut Env, Term, *mut Binary) -> c_int, // 0.1
    pub alloc_binary: unsafe extern "C" fn(usize, *mut Binary) -> c_int, // 0.1
    pub realloc_binary: unsafe extern "C" fn(*mut Binary, usize) -> c_int, // 1.0
    pub release_binary: unsafe extern "C" fn(*mut Binary),        // 2.0
    pub get_int: unsafe extern "C" fn(*mut Env, Term, *mut c_int) -> c_int, // 0.1
    pub get_ulong: unsafe extern "C" fn(*mut Env, Term, *mut c_ulong) -> c_int, // 0.1
    pub get_double: unsafe extern "C" fn(*mut Env, Term, *mut f64) -> c_int, // 0.1
    pub get_list_cell: unsafe extern "C" fn(*mut Env, Term, *mut Term, *mut Term) -> c_int, // 0.1
    pub get_tuple: unsafe extern "C" fn(*mut Env, Term, *mut c_int, *mut *const Term) -> c_int, // 0.1
    pub is_identical: unsafe extern "C" fn(Term, Term) -> c_int, // 0.1
    pub compare: unsafe extern "C" fn(Term, Term) -> c_int,      // 0.1
    pub make_binary: unsafe extern "C" fn(*mut Env, *mut Binary) -> Term, // 0.1
    pub make_badarg: unsafe extern "C" fn(*mut Env) -> Term,     // 0.1
    pub make_int: unsafe extern "C" fn(*mut Env, c_int) -> Term, // 0.1
    pub make_ulong: unsafe extern "C" fn(*mut Env, c_ulong) -> Term, // 0.1
    pub make_double: unsafe extern "C" fn(*mut Env, f64) -> Term, // 0.1
    pub make_atom: unsafe extern "C" fn(*mut Env, *const c_char) -> Term, // 0.1
    pub make_existing_atom:
        unsafe extern "C" fn(*mut Env, *const c_char, *mut Term, CharEncoding) -> c_int, // 0.1
    /// Variadic; the `make_tupleN` wrappers call this with N args. 0.1
    pub make_tuple: unsafe extern "C" fn(*mut Env, c_uint, ...) -> Term, // 0.1
    /// Variadic; the `make_listN` wrappers call this with N args. 0.1
    pub make_list: unsafe extern "C" fn(*mut Env, c_uint, ...) -> Term, // 0.1
    pub make_list_cell: unsafe extern "C" fn(*mut Env, Term, Term) -> Term, // 0.1
    pub make_string: unsafe extern "C" fn(*mut Env, *const c_char, CharEncoding) -> Term, // 0.1
    pub make_ref: unsafe extern "C" fn(*mut Env) -> Term,        // 0.1

    // ── NIF 1.0 — thread primitives (mutex, cond, rwlock, tsd, thread) ──
    pub mutex_create: unsafe extern "C" fn(*mut c_char) -> *mut Mutex,
    pub mutex_destroy: unsafe extern "C" fn(*mut Mutex),
    pub mutex_trylock: unsafe extern "C" fn(*mut Mutex) -> c_int,
    pub mutex_lock: unsafe extern "C" fn(*mut Mutex),
    pub mutex_unlock: unsafe extern "C" fn(*mut Mutex),
    pub cond_create: unsafe extern "C" fn(*mut c_char) -> *mut Cond,
    pub cond_destroy: unsafe extern "C" fn(*mut Cond),
    pub cond_signal: unsafe extern "C" fn(*mut Cond),
    pub cond_broadcast: unsafe extern "C" fn(*mut Cond),
    pub cond_wait: unsafe extern "C" fn(*mut Cond, *mut Mutex),
    pub rwlock_create: unsafe extern "C" fn(*mut c_char) -> *mut RWLock,
    pub rwlock_destroy: unsafe extern "C" fn(*mut RWLock),
    pub rwlock_tryrlock: unsafe extern "C" fn(*mut RWLock) -> c_int,
    pub rwlock_rlock: unsafe extern "C" fn(*mut RWLock),
    pub rwlock_runlock: unsafe extern "C" fn(*mut RWLock),
    pub rwlock_tryrwlock: unsafe extern "C" fn(*mut RWLock) -> c_int,
    pub rwlock_rwlock: unsafe extern "C" fn(*mut RWLock),
    pub rwlock_rwunlock: unsafe extern "C" fn(*mut RWLock),
    pub tsd_key_create: unsafe extern "C" fn(*mut c_char, *mut TSDKey) -> c_int,
    pub tsd_key_destroy: unsafe extern "C" fn(TSDKey),
    pub tsd_set: unsafe extern "C" fn(TSDKey, *mut c_void),
    pub tsd_get: unsafe extern "C" fn(TSDKey) -> *mut c_void,
    pub thread_opts_create: unsafe extern "C" fn(*mut c_char) -> *mut ThreadOpts,
    pub thread_opts_destroy: unsafe extern "C" fn(*mut ThreadOpts),
    pub thread_create: unsafe extern "C" fn(
        *mut c_char,
        *mut Tid,
        Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        *mut c_void,
        *mut ThreadOpts,
    ) -> c_int,
    pub thread_self: unsafe extern "C" fn() -> Tid,
    pub equal_tids: unsafe extern "C" fn(Tid, Tid) -> c_int,
    pub thread_exit: unsafe extern "C" fn(*mut c_void),
    pub thread_join: unsafe extern "C" fn(Tid, *mut *mut c_void) -> c_int,

    // ── NIF 1.0 / 2.0 — more core, resources, strings, env, send ──
    pub realloc: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void, // 1.0
    pub system_info: unsafe extern "C" fn(*mut SysInfo, usize),           // 1.0
    /// Variadic (`FILE*`, fmt, ...). 1.0
    #[allow(dead_code)] // unwrapped (varargs/va_list); slot kept for table order
    pub fprintf: unsafe extern "C" fn(*mut c_void, *const c_char, ...) -> c_int, // 1.0
    pub inspect_iolist_as_binary: unsafe extern "C" fn(*mut Env, Term, *mut Binary) -> c_int, // 1.0
    pub make_sub_binary: unsafe extern "C" fn(*mut Env, Term, usize, usize) -> Term, // 1.0
    pub get_string:
        unsafe extern "C" fn(*mut Env, Term, *mut c_char, c_uint, CharEncoding) -> c_int, // 1.0
    pub get_atom: unsafe extern "C" fn(*mut Env, Term, *mut c_char, c_uint, CharEncoding) -> c_int, // 1.0
    pub is_fun: unsafe extern "C" fn(*mut Env, Term) -> c_int, // 1.0
    pub is_pid: unsafe extern "C" fn(*mut Env, Term) -> c_int, // 1.0
    pub is_port: unsafe extern "C" fn(*mut Env, Term) -> c_int, // 1.0
    pub get_uint: unsafe extern "C" fn(*mut Env, Term, *mut c_uint) -> c_int, // 1.0
    pub get_long: unsafe extern "C" fn(*mut Env, Term, *mut c_long) -> c_int, // 1.0
    pub make_uint: unsafe extern "C" fn(*mut Env, c_uint) -> Term, // 1.0
    pub make_long: unsafe extern "C" fn(*mut Env, c_long) -> Term, // 1.0
    pub make_tuple_from_array: unsafe extern "C" fn(*mut Env, *const Term, c_uint) -> Term, // 1.0
    pub make_list_from_array: unsafe extern "C" fn(*mut Env, *const Term, c_uint) -> Term, // 1.0
    pub is_empty_list: unsafe extern "C" fn(*mut Env, Term) -> c_int, // 1.0
    pub open_resource_type: unsafe extern "C" fn(
        *mut Env,
        *const c_char,
        *const c_char,
        Option<unsafe extern "C" fn(*mut Env, *mut c_void)>,
        ResourceFlags,
        *mut ResourceFlags,
    ) -> *mut ResourceType, // 1.0
    pub alloc_resource: unsafe extern "C" fn(*mut ResourceType, usize) -> *mut c_void, // 1.0
    pub release_resource: unsafe extern "C" fn(*mut c_void),   // 1.0
    pub make_resource: unsafe extern "C" fn(*mut Env, *mut c_void) -> Term, // 1.0
    pub get_resource:
        unsafe extern "C" fn(*mut Env, Term, *mut ResourceType, *mut *mut c_void) -> c_int, // 1.0
    pub sizeof_resource: unsafe extern "C" fn(*mut c_void) -> usize, // 1.0
    pub make_new_binary: unsafe extern "C" fn(*mut Env, usize, *mut Term) -> *mut u8, // 1.0
    pub is_list: unsafe extern "C" fn(*mut Env, Term) -> c_int, // 2.0
    pub is_tuple: unsafe extern "C" fn(*mut Env, Term) -> c_int, // 2.0
    pub get_atom_length: unsafe extern "C" fn(*mut Env, Term, *mut c_uint, CharEncoding) -> c_int, // 2.0
    pub get_list_length: unsafe extern "C" fn(*mut Env, Term, *mut c_uint) -> c_int, // 2.0
    pub make_atom_len: unsafe extern "C" fn(*mut Env, *const c_char, usize) -> Term, // 2.0
    pub make_existing_atom_len:
        unsafe extern "C" fn(*mut Env, *const c_char, usize, *mut Term, CharEncoding) -> c_int, // 2.0
    pub make_string_len: unsafe extern "C" fn(*mut Env, *const c_char, usize, CharEncoding) -> Term, // 2.0
    pub alloc_env: unsafe extern "C" fn() -> *mut Env, // 2.0
    pub free_env: unsafe extern "C" fn(*mut Env),      // 2.0
    pub clear_env: unsafe extern "C" fn(*mut Env),     // 2.0
    pub send: unsafe extern "C" fn(*mut Env, *const Pid, *mut Env, Term) -> c_int, // 2.0
    pub make_copy: unsafe extern "C" fn(*mut Env, Term) -> Term, // 2.0
    pub self_: unsafe extern "C" fn(*mut Env, *mut Pid) -> *mut Pid, // 2.0
    pub get_local_pid: unsafe extern "C" fn(*mut Env, Term, *mut Pid) -> c_int, // 2.0
    pub keep_resource: unsafe extern "C" fn(*mut c_void), // 2.0
    pub make_resource_binary:
        unsafe extern "C" fn(*mut Env, *mut c_void, *const c_void, usize) -> Term, // 2.0

    // ── NIF 2.0 — 64-bit integers (header gates these on SIZEOF_LONG != 8; on
    //    a 64-bit target they alias the `long` variants, see `init`) ──
    pub get_int64: unsafe extern "C" fn(*mut Env, Term, *mut i64) -> c_int,
    pub get_uint64: unsafe extern "C" fn(*mut Env, Term, *mut u64) -> c_int,
    pub make_int64: unsafe extern "C" fn(*mut Env, i64) -> Term,
    pub make_uint64: unsafe extern "C" fn(*mut Env, u64) -> Term,

    // ── NIF 2.2 – 2.4 ──
    pub is_exception: unsafe extern "C" fn(*mut Env, Term) -> c_int, // 2.2
    pub make_reverse_list: unsafe extern "C" fn(*mut Env, Term, *mut Term) -> c_int, // 2.3
    pub is_number: unsafe extern "C" fn(*mut Env, Term) -> c_int,    // 2.3
    pub dlopen: unsafe extern "C" fn(
        *const c_char,
        Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
        *mut c_void,
    ) -> *mut c_void, // 2.4
    pub dlsym: unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
        *mut c_void,
    ) -> *mut c_void, // 2.4
    pub consume_timeslice: unsafe extern "C" fn(*mut Env, c_int) -> c_int, // 2.4

    // ── NIF 2.6 — maps ──
    pub is_map: unsafe extern "C" fn(*mut Env, Term) -> c_int,
    pub get_map_size: unsafe extern "C" fn(*mut Env, Term, *mut usize) -> c_int,
    pub make_new_map: unsafe extern "C" fn(*mut Env) -> Term,
    pub make_map_put: unsafe extern "C" fn(*mut Env, Term, Term, Term, *mut Term) -> c_int,
    pub get_map_value: unsafe extern "C" fn(*mut Env, Term, Term, *mut Term) -> c_int,
    pub make_map_update: unsafe extern "C" fn(*mut Env, Term, Term, Term, *mut Term) -> c_int,
    pub make_map_remove: unsafe extern "C" fn(*mut Env, Term, Term, *mut Term) -> c_int,
    pub map_iterator_create:
        unsafe extern "C" fn(*mut Env, Term, *mut MapIterator, MapIteratorEntry) -> c_int,
    pub map_iterator_destroy: unsafe extern "C" fn(*mut Env, *mut MapIterator),
    pub map_iterator_is_head: unsafe extern "C" fn(*mut Env, *mut MapIterator) -> c_int,
    pub map_iterator_is_tail: unsafe extern "C" fn(*mut Env, *mut MapIterator) -> c_int,
    pub map_iterator_next: unsafe extern "C" fn(*mut Env, *mut MapIterator) -> c_int,
    pub map_iterator_prev: unsafe extern "C" fn(*mut Env, *mut MapIterator) -> c_int,
    pub map_iterator_get_pair:
        unsafe extern "C" fn(*mut Env, *mut MapIterator, *mut Term, *mut Term) -> c_int,

    // ── NIF 2.7 – 2.9 ──
    pub schedule_nif: unsafe extern "C" fn(
        *mut Env,
        *const c_char,
        c_int,
        unsafe extern "C" fn(*mut Env, c_int, *const Term) -> Term,
        c_int,
        *const Term,
    ) -> Term, // 2.7
    pub has_pending_exception: unsafe extern "C" fn(*mut Env, *mut Term) -> c_int, // 2.8
    pub raise_exception: unsafe extern "C" fn(*mut Env, Term) -> Term,             // 2.8
    pub getenv: unsafe extern "C" fn(*const c_char, *mut c_char, *mut usize) -> c_int, // 2.9

    // ── NIF 2.10 — time ──
    pub monotonic_time: unsafe extern "C" fn(TimeUnit) -> Time,
    pub time_offset: unsafe extern "C" fn(TimeUnit) -> Time,
    pub convert_time_unit: unsafe extern "C" fn(Time, TimeUnit, TimeUnit) -> Time,

    // ── NIF 2.11 — process/port queries, term<->binary, snprintf ──
    pub now_time: unsafe extern "C" fn(*mut Env) -> Term,
    pub cpu_time: unsafe extern "C" fn(*mut Env) -> Term,
    pub make_unique_integer: unsafe extern "C" fn(*mut Env, UniqueInteger) -> Term,
    pub is_current_process_alive: unsafe extern "C" fn(*mut Env) -> c_int,
    pub is_process_alive: unsafe extern "C" fn(*mut Env, *const Pid) -> c_int,
    pub is_port_alive: unsafe extern "C" fn(*mut Env, *const Port) -> c_int,
    pub get_local_port: unsafe extern "C" fn(*mut Env, Term, *mut Port) -> c_int,
    pub term_to_binary: unsafe extern "C" fn(*mut Env, Term, *mut Binary) -> c_int,
    pub binary_to_term:
        unsafe extern "C" fn(*mut Env, *const u8, usize, *mut Term, c_uint) -> usize,
    pub port_command: unsafe extern "C" fn(*mut Env, *const Port, *mut Env, Term) -> c_int,
    pub thread_type: unsafe extern "C" fn() -> c_int,
    /// Variadic (buf, size, fmt, ...). 2.11
    #[allow(dead_code)] // unwrapped (varargs/va_list); slot kept for table order
    pub snprintf: unsafe extern "C" fn(*mut c_char, usize, *const c_char, ...) -> c_int,

    // ── NIF 2.12 — select, monitors, hash, whereis ──
    pub select:
        unsafe extern "C" fn(*mut Env, Event, SelectFlags, *mut c_void, *const Pid, Term) -> c_int,
    pub open_resource_type_x: unsafe extern "C" fn(
        *mut Env,
        *const c_char,
        *const ResourceTypeInit,
        ResourceFlags,
        *mut ResourceFlags,
    ) -> *mut ResourceType,
    pub monitor_process:
        unsafe extern "C" fn(*mut Env, *mut c_void, *const Pid, *mut Monitor) -> c_int,
    pub demonitor_process: unsafe extern "C" fn(*mut Env, *mut c_void, *const Monitor) -> c_int,
    pub compare_monitors: unsafe extern "C" fn(*const Monitor, *const Monitor) -> c_int,
    pub hash: unsafe extern "C" fn(Hash, Term, u64) -> u64,
    pub whereis_pid: unsafe extern "C" fn(*mut Env, Term, *mut Pid) -> c_int,
    pub whereis_port: unsafe extern "C" fn(*mut Env, Term, *mut Port) -> c_int,

    // ── NIF 2.13 — I/O queue ──
    pub ioq_create: unsafe extern "C" fn(IOQueueOpts) -> *mut IOQueue,
    pub ioq_destroy: unsafe extern "C" fn(*mut IOQueue),
    pub ioq_enq_binary: unsafe extern "C" fn(*mut IOQueue, *mut Binary, usize) -> c_int,
    pub ioq_enqv: unsafe extern "C" fn(*mut IOQueue, *mut IOVec, usize) -> c_int,
    pub ioq_size: unsafe extern "C" fn(*mut IOQueue) -> usize,
    pub ioq_deq: unsafe extern "C" fn(*mut IOQueue, usize, *mut usize) -> c_int,
    pub ioq_peek: unsafe extern "C" fn(*mut IOQueue, *mut c_int) -> *mut SysIOVec,
    pub inspect_iovec:
        unsafe extern "C" fn(*mut Env, usize, Term, *mut Term, *mut *mut IOVec) -> c_int,
    pub free_iovec: unsafe extern "C" fn(*mut IOVec),

    // ── NIF 2.14 — ioq_peek_head, *_name, v*printf, make_map_from_arrays ──
    pub ioq_peek_head: unsafe extern "C" fn(*mut Env, *mut IOQueue, *mut usize, *mut Term) -> c_int,
    pub mutex_name: unsafe extern "C" fn(*mut Mutex) -> *mut c_char,
    pub cond_name: unsafe extern "C" fn(*mut Cond) -> *mut c_char,
    pub rwlock_name: unsafe extern "C" fn(*mut RWLock) -> *mut c_char,
    pub thread_name: unsafe extern "C" fn(Tid) -> *mut c_char,
    /// `va_list` argument approximated as a pointer — slot kept for table order;
    /// not wrapped (Rust has no portable `va_list`). 2.14
    #[allow(dead_code)] // unwrapped (varargs/va_list); slot kept for table order
    pub vfprintf: unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void) -> c_int,
    /// See [`Self::vfprintf`]. Not wrapped. 2.14
    #[allow(dead_code)] // unwrapped (varargs/va_list); slot kept for table order
    pub vsnprintf: unsafe extern "C" fn(*mut c_char, usize, *const c_char, *mut c_void) -> c_int,
    pub make_map_from_arrays:
        unsafe extern "C" fn(*mut Env, *const Term, *const Term, usize, *mut Term) -> c_int,

    // ── NIF 2.15 — select_x, monitor term, pid-undefined, term_type ──
    pub select_x: unsafe extern "C" fn(
        *mut Env,
        Event,
        SelectFlags,
        *mut c_void,
        *const Pid,
        Term,
        *mut Env,
    ) -> c_int,
    pub make_monitor_term: unsafe extern "C" fn(*mut Env, *const Monitor) -> Term,
    pub set_pid_undefined: unsafe extern "C" fn(*mut Pid),
    pub is_pid_undefined: unsafe extern "C" fn(*const Pid) -> c_int,
    pub term_type: unsafe extern "C" fn(*mut Env, Term) -> c_int,

    // ── NIF 2.16 (OTP 24) ──
    #[cfg(feature = "nif_2_16")]
    pub init_resource_type: unsafe extern "C" fn(
        *mut Env,
        *const c_char,
        *const ResourceTypeInit,
        ResourceFlags,
        *mut ResourceFlags,
    ) -> *mut ResourceType,
    #[cfg(feature = "nif_2_16")]
    pub dynamic_resource_call:
        unsafe extern "C" fn(*mut Env, Term, Term, Term, *mut c_void) -> c_int,

    // ── NIF 2.17 (OTP 26) ──
    #[cfg(feature = "nif_2_17")]
    pub get_string_length: unsafe extern "C" fn(*mut Env, Term, *mut c_uint, CharEncoding) -> c_int,
    #[cfg(feature = "nif_2_17")]
    pub make_new_atom:
        unsafe extern "C" fn(*mut Env, *const c_char, *mut Term, CharEncoding) -> c_int,
    #[cfg(feature = "nif_2_17")]
    pub make_new_atom_len:
        unsafe extern "C" fn(*mut Env, *const c_char, usize, *mut Term, CharEncoding) -> c_int,
    /// Variadic (env, opt, ...). 2.17
    #[cfg(feature = "nif_2_17")]
    pub set_option: unsafe extern "C" fn(*mut Env, Option_, ...) -> c_int,

    // ── NIF 2.18 (OTP 29) ──
    #[cfg(feature = "nif_2_18")]
    pub term_size: unsafe extern "C" fn(Term) -> usize,
    #[cfg(feature = "nif_2_18")]
    pub get_atom_cache_index: unsafe extern "C" fn(*mut Env, Term, *mut c_uint) -> c_int,
    #[cfg(feature = "nif_2_18")]
    pub max_atom_cache_index: unsafe extern "C" fn() -> c_uint,
}

// ---------------------------------------------------------------------------
// Global storage
// ---------------------------------------------------------------------------

static API: OnceLock<Api> = OnceLock::new();

/// The resolved function table. Panics if [`init`] has not run.
#[inline]
pub(crate) fn api() -> &'static Api {
    API.get()
        .expect("enif_ffi: not initialized — init() was not called")
}

// ---------------------------------------------------------------------------
// Initialization — Unix (dlsym)
// ---------------------------------------------------------------------------

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
#[cfg(unix)]
pub unsafe fn init() -> Result<(), &'static str> {
    if API.get().is_some() {
        return Ok(());
    }

    unsafe fn load<T>(name: &[u8]) -> Result<T, &'static str> {
        assert!(
            size_of::<T>() == size_of::<*mut c_void>(),
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

// ---------------------------------------------------------------------------
// Initialization — non-Unix
// ---------------------------------------------------------------------------

/// Windows binds the API through a callback struct passed at load rather than
/// `dlsym`; that path is not implemented yet.
#[cfg(not(unix))]
pub unsafe fn init() -> Result<(), &'static str> {
    compile_error!("enif-ffi: only Unix is supported at this time");
}
