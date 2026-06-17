//! The `enif_*` function-pointer table and the load-time symbol resolver.
//!
//! The BEAM does not let a NIF library link against `enif_*` directly; the
//! symbols are resolved at load time. [`Api`] holds one pointer per
//! `enif_*` C function; the platform loader fills it — `unix::init` via `dlsym`
//! (Unix) or `windows::init` from the BEAM-supplied callback table (Windows).
//!
//! Field order is the canonical `erl_nif_api_funcs.h` declaration order — the
//! same order as the Windows `TWinDynNifCallbacks` struct, which is read
//! straight into this type on Windows. Each field is marked with
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
/// and can live behind a `OnceLock`. `#[repr(C)]` fixes the field order to the
/// declaration order, which is also the layout of the Windows
/// `TWinDynNifCallbacks` struct — so on Windows the BEAM-supplied table can be
/// read straight into this type (see `windows::init`).
#[rustfmt::skip]
#[repr(C)]
pub(crate) struct Api {
    // ── NIF 0.1 / 1.0 — initial term, binary, integer, atom, list/tuple core ──
    pub priv_data:                unsafe extern "C" fn(*mut Env) -> *mut c_void, // 1.0
    pub alloc:                    unsafe extern "C" fn(usize) -> *mut c_void, // 1.0
    pub free:                     unsafe extern "C" fn(*mut c_void), // 1.0
    pub is_atom:                  unsafe extern "C" fn(*mut Env, Term) -> c_int, // 0.1
    pub is_binary:                unsafe extern "C" fn(*mut Env, Term) -> c_int, // 0.1
    pub is_ref:                   unsafe extern "C" fn(*mut Env, Term) -> c_int, // 0.1
    pub inspect_binary:           unsafe extern "C" fn(*mut Env, Term, *mut Binary) -> c_int, // 0.1
    pub alloc_binary:             unsafe extern "C" fn(usize, *mut Binary) -> c_int, // 0.1
    pub realloc_binary:           unsafe extern "C" fn(*mut Binary, usize) -> c_int, // 1.0
    pub release_binary:           unsafe extern "C" fn(*mut Binary), // 2.0
    pub get_int:                  unsafe extern "C" fn(*mut Env, Term, *mut c_int) -> c_int, // 0.1
    pub get_ulong:                unsafe extern "C" fn(*mut Env, Term, *mut c_ulong) -> c_int, // 0.1
    pub get_double:               unsafe extern "C" fn(*mut Env, Term, *mut f64) -> c_int, // 0.1
    pub get_list_cell:            unsafe extern "C" fn(*mut Env, Term, *mut Term, *mut Term) -> c_int, // 0.1
    pub get_tuple:                unsafe extern "C" fn(*mut Env, Term, *mut c_int, *mut *const Term) -> c_int, // 0.1
    pub is_identical:             unsafe extern "C" fn(Term, Term) -> c_int, // 0.1
    pub compare:                  unsafe extern "C" fn(Term, Term) -> c_int, // 0.1
    pub make_binary:              unsafe extern "C" fn(*mut Env, *mut Binary) -> Term, // 0.1
    pub make_badarg:              unsafe extern "C" fn(*mut Env) -> Term, // 0.1
    pub make_int:                 unsafe extern "C" fn(*mut Env, c_int) -> Term, // 0.1
    pub make_ulong:               unsafe extern "C" fn(*mut Env, c_ulong) -> Term, // 0.1
    pub make_double:              unsafe extern "C" fn(*mut Env, f64) -> Term, // 0.1
    pub make_atom:                unsafe extern "C" fn(*mut Env, *const c_char) -> Term, // 0.1
    pub make_existing_atom:       unsafe extern "C" fn(*mut Env, *const c_char, *mut Term, CharEncoding) -> c_int, // 0.1
    /// Variadic; the `make_tupleN` wrappers call this with N args. 0.1
    pub make_tuple:               unsafe extern "C" fn(*mut Env, c_uint, ...) -> Term, // 0.1
    /// Variadic; the `make_listN` wrappers call this with N args. 0.1
    pub make_list:                unsafe extern "C" fn(*mut Env, c_uint, ...) -> Term, // 0.1
    pub make_list_cell:           unsafe extern "C" fn(*mut Env, Term, Term) -> Term, // 0.1
    pub make_string:              unsafe extern "C" fn(*mut Env, *const c_char, CharEncoding) -> Term, // 0.1
    pub make_ref:                 unsafe extern "C" fn(*mut Env) -> Term, // 0.1

    // ── NIF 1.0 — thread primitives (mutex, cond, rwlock, tsd, thread) ──
    pub mutex_create:             unsafe extern "C" fn(*mut c_char) -> *mut Mutex,
    pub mutex_destroy:            unsafe extern "C" fn(*mut Mutex),
    pub mutex_trylock:            unsafe extern "C" fn(*mut Mutex) -> c_int,
    pub mutex_lock:               unsafe extern "C" fn(*mut Mutex),
    pub mutex_unlock:             unsafe extern "C" fn(*mut Mutex),
    pub cond_create:              unsafe extern "C" fn(*mut c_char) -> *mut Cond,
    pub cond_destroy:             unsafe extern "C" fn(*mut Cond),
    pub cond_signal:              unsafe extern "C" fn(*mut Cond),
    pub cond_broadcast:           unsafe extern "C" fn(*mut Cond),
    pub cond_wait:                unsafe extern "C" fn(*mut Cond, *mut Mutex),
    pub rwlock_create:            unsafe extern "C" fn(*mut c_char) -> *mut RWLock,
    pub rwlock_destroy:           unsafe extern "C" fn(*mut RWLock),
    pub rwlock_tryrlock:          unsafe extern "C" fn(*mut RWLock) -> c_int,
    pub rwlock_rlock:             unsafe extern "C" fn(*mut RWLock),
    pub rwlock_runlock:           unsafe extern "C" fn(*mut RWLock),
    pub rwlock_tryrwlock:         unsafe extern "C" fn(*mut RWLock) -> c_int,
    pub rwlock_rwlock:            unsafe extern "C" fn(*mut RWLock),
    pub rwlock_rwunlock:          unsafe extern "C" fn(*mut RWLock),
    pub tsd_key_create:           unsafe extern "C" fn(*mut c_char, *mut TSDKey) -> c_int,
    pub tsd_key_destroy:          unsafe extern "C" fn(TSDKey),
    pub tsd_set:                  unsafe extern "C" fn(TSDKey, *mut c_void),
    pub tsd_get:                  unsafe extern "C" fn(TSDKey) -> *mut c_void,
    pub thread_opts_create:       unsafe extern "C" fn(*mut c_char) -> *mut ThreadOpts,
    pub thread_opts_destroy:      unsafe extern "C" fn(*mut ThreadOpts),
    pub thread_create:            unsafe extern "C" fn(*mut c_char, *mut Tid, Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>, *mut c_void, *mut ThreadOpts) -> c_int,
    pub thread_self:              unsafe extern "C" fn() -> Tid,
    pub equal_tids:               unsafe extern "C" fn(Tid, Tid) -> c_int,
    pub thread_exit:              unsafe extern "C" fn(*mut c_void),
    pub thread_join:              unsafe extern "C" fn(Tid, *mut *mut c_void) -> c_int,

    // ── NIF 1.0 / 2.0 — more core, resources, strings, env, send ──
    pub realloc:                  unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void, // 1.0
    pub system_info:              unsafe extern "C" fn(*mut SysInfo, usize), // 1.0
    /// Variadic (`FILE*`, fmt, ...). 1.0
    #[allow(dead_code)] // unwrapped (varargs/va_list); slot kept for table order
    pub fprintf:                  unsafe extern "C" fn(*mut c_void, *const c_char, ...) -> c_int, // 1.0
    pub inspect_iolist_as_binary: unsafe extern "C" fn(*mut Env, Term, *mut Binary) -> c_int, // 1.0
    pub make_sub_binary:          unsafe extern "C" fn(*mut Env, Term, usize, usize) -> Term, // 1.0
    pub get_string:               unsafe extern "C" fn(*mut Env, Term, *mut c_char, c_uint, CharEncoding) -> c_int, // 1.0
    pub get_atom:                 unsafe extern "C" fn(*mut Env, Term, *mut c_char, c_uint, CharEncoding) -> c_int, // 1.0
    pub is_fun:                   unsafe extern "C" fn(*mut Env, Term) -> c_int, // 1.0
    pub is_pid:                   unsafe extern "C" fn(*mut Env, Term) -> c_int, // 1.0
    pub is_port:                  unsafe extern "C" fn(*mut Env, Term) -> c_int, // 1.0
    pub get_uint:                 unsafe extern "C" fn(*mut Env, Term, *mut c_uint) -> c_int, // 1.0
    pub get_long:                 unsafe extern "C" fn(*mut Env, Term, *mut c_long) -> c_int, // 1.0
    pub make_uint:                unsafe extern "C" fn(*mut Env, c_uint) -> Term, // 1.0
    pub make_long:                unsafe extern "C" fn(*mut Env, c_long) -> Term, // 1.0
    pub make_tuple_from_array:    unsafe extern "C" fn(*mut Env, *const Term, c_uint) -> Term, // 1.0
    pub make_list_from_array:     unsafe extern "C" fn(*mut Env, *const Term, c_uint) -> Term, // 1.0
    pub is_empty_list:            unsafe extern "C" fn(*mut Env, Term) -> c_int, // 1.0
    pub open_resource_type:       unsafe extern "C" fn(*mut Env, *const c_char, *const c_char, Option<unsafe extern "C" fn(*mut Env, *mut c_void)>, ResourceFlags, *mut ResourceFlags) -> *mut ResourceType, // 1.0
    pub alloc_resource:           unsafe extern "C" fn(*mut ResourceType, usize) -> *mut c_void, // 1.0
    pub release_resource:         unsafe extern "C" fn(*mut c_void), // 1.0
    pub make_resource:            unsafe extern "C" fn(*mut Env, *mut c_void) -> Term, // 1.0
    pub get_resource:             unsafe extern "C" fn(*mut Env, Term, *mut ResourceType, *mut *mut c_void) -> c_int, // 1.0
    pub sizeof_resource:          unsafe extern "C" fn(*mut c_void) -> usize, // 1.0
    pub make_new_binary:          unsafe extern "C" fn(*mut Env, usize, *mut Term) -> *mut u8, // 1.0
    pub is_list:                  unsafe extern "C" fn(*mut Env, Term) -> c_int, // 2.0
    pub is_tuple:                 unsafe extern "C" fn(*mut Env, Term) -> c_int, // 2.0
    pub get_atom_length:          unsafe extern "C" fn(*mut Env, Term, *mut c_uint, CharEncoding) -> c_int, // 2.0
    pub get_list_length:          unsafe extern "C" fn(*mut Env, Term, *mut c_uint) -> c_int, // 2.0
    pub make_atom_len:            unsafe extern "C" fn(*mut Env, *const c_char, usize) -> Term, // 2.0
    pub make_existing_atom_len:   unsafe extern "C" fn(*mut Env, *const c_char, usize, *mut Term, CharEncoding) -> c_int, // 2.0
    pub make_string_len:          unsafe extern "C" fn(*mut Env, *const c_char, usize, CharEncoding) -> Term, // 2.0
    pub alloc_env:                unsafe extern "C" fn() -> *mut Env, // 2.0
    pub free_env:                 unsafe extern "C" fn(*mut Env), // 2.0
    pub clear_env:                unsafe extern "C" fn(*mut Env), // 2.0
    pub send:                     unsafe extern "C" fn(*mut Env, *const Pid, *mut Env, Term) -> c_int, // 2.0
    pub make_copy:                unsafe extern "C" fn(*mut Env, Term) -> Term, // 2.0
    pub self_:                    unsafe extern "C" fn(*mut Env, *mut Pid) -> *mut Pid, // 2.0
    pub get_local_pid:            unsafe extern "C" fn(*mut Env, Term, *mut Pid) -> c_int, // 2.0
    pub keep_resource:            unsafe extern "C" fn(*mut c_void), // 2.0
    pub make_resource_binary:     unsafe extern "C" fn(*mut Env, *mut c_void, *const c_void, usize) -> Term, // 2.0

    // ── NIF 2.0 — 64-bit integers (header gates these on SIZEOF_LONG != 8; on
    //    a 64-bit target they alias the `long` variants, see `unix::init`) ──
    pub get_int64:                unsafe extern "C" fn(*mut Env, Term, *mut i64) -> c_int,
    pub get_uint64:               unsafe extern "C" fn(*mut Env, Term, *mut u64) -> c_int,
    pub make_int64:               unsafe extern "C" fn(*mut Env, i64) -> Term,
    pub make_uint64:              unsafe extern "C" fn(*mut Env, u64) -> Term,

    // ── NIF 2.2 – 2.4 ──
    pub is_exception:             unsafe extern "C" fn(*mut Env, Term) -> c_int, // 2.2
    pub make_reverse_list:        unsafe extern "C" fn(*mut Env, Term, *mut Term) -> c_int, // 2.3
    pub is_number:                unsafe extern "C" fn(*mut Env, Term) -> c_int, // 2.3
    pub dlopen:                   unsafe extern "C" fn(*const c_char, Option<unsafe extern "C" fn(*mut c_void, *const c_char)>, *mut c_void) -> *mut c_void, // 2.4
    pub dlsym:                    unsafe extern "C" fn(*mut c_void, *const c_char, Option<unsafe extern "C" fn(*mut c_void, *const c_char)>, *mut c_void) -> *mut c_void, // 2.4
    pub consume_timeslice:        unsafe extern "C" fn(*mut Env, c_int) -> c_int, // 2.4

    // ── NIF 2.6 — maps ──
    pub is_map:                   unsafe extern "C" fn(*mut Env, Term) -> c_int,
    pub get_map_size:             unsafe extern "C" fn(*mut Env, Term, *mut usize) -> c_int,
    pub make_new_map:             unsafe extern "C" fn(*mut Env) -> Term,
    pub make_map_put:             unsafe extern "C" fn(*mut Env, Term, Term, Term, *mut Term) -> c_int,
    pub get_map_value:            unsafe extern "C" fn(*mut Env, Term, Term, *mut Term) -> c_int,
    pub make_map_update:          unsafe extern "C" fn(*mut Env, Term, Term, Term, *mut Term) -> c_int,
    pub make_map_remove:          unsafe extern "C" fn(*mut Env, Term, Term, *mut Term) -> c_int,
    pub map_iterator_create:      unsafe extern "C" fn(*mut Env, Term, *mut MapIterator, MapIteratorEntry) -> c_int,
    pub map_iterator_destroy:     unsafe extern "C" fn(*mut Env, *mut MapIterator),
    pub map_iterator_is_head:     unsafe extern "C" fn(*mut Env, *mut MapIterator) -> c_int,
    pub map_iterator_is_tail:     unsafe extern "C" fn(*mut Env, *mut MapIterator) -> c_int,
    pub map_iterator_next:        unsafe extern "C" fn(*mut Env, *mut MapIterator) -> c_int,
    pub map_iterator_prev:        unsafe extern "C" fn(*mut Env, *mut MapIterator) -> c_int,
    pub map_iterator_get_pair:    unsafe extern "C" fn(*mut Env, *mut MapIterator, *mut Term, *mut Term) -> c_int,

    // ── NIF 2.7 – 2.9 ──
    pub schedule_nif:             unsafe extern "C" fn(*mut Env, *const c_char, c_int, unsafe extern "C" fn(*mut Env, c_int, *const Term) -> Term, c_int, *const Term) -> Term, // 2.7
    pub has_pending_exception:    unsafe extern "C" fn(*mut Env, *mut Term) -> c_int, // 2.8
    pub raise_exception:          unsafe extern "C" fn(*mut Env, Term) -> Term, // 2.8
    pub getenv:                   unsafe extern "C" fn(*const c_char, *mut c_char, *mut usize) -> c_int, // 2.9

    // ── NIF 2.10 — time ──
    pub monotonic_time:           unsafe extern "C" fn(TimeUnit) -> Time,
    pub time_offset:              unsafe extern "C" fn(TimeUnit) -> Time,
    pub convert_time_unit:        unsafe extern "C" fn(Time, TimeUnit, TimeUnit) -> Time,

    // ── NIF 2.11 — process/port queries, term<->binary, snprintf ──
    pub now_time:                 unsafe extern "C" fn(*mut Env) -> Term,
    pub cpu_time:                 unsafe extern "C" fn(*mut Env) -> Term,
    pub make_unique_integer:      unsafe extern "C" fn(*mut Env, UniqueInteger) -> Term,
    pub is_current_process_alive: unsafe extern "C" fn(*mut Env) -> c_int,
    pub is_process_alive:         unsafe extern "C" fn(*mut Env, *const Pid) -> c_int,
    pub is_port_alive:            unsafe extern "C" fn(*mut Env, *const Port) -> c_int,
    pub get_local_port:           unsafe extern "C" fn(*mut Env, Term, *mut Port) -> c_int,
    pub term_to_binary:           unsafe extern "C" fn(*mut Env, Term, *mut Binary) -> c_int,
    pub binary_to_term:           unsafe extern "C" fn(*mut Env, *const u8, usize, *mut Term, c_uint) -> usize,
    pub port_command:             unsafe extern "C" fn(*mut Env, *const Port, *mut Env, Term) -> c_int,
    pub thread_type:              unsafe extern "C" fn() -> c_int,
    /// Variadic (buf, size, fmt, ...). 2.11
    #[allow(dead_code)] // unwrapped (varargs/va_list); slot kept for table order
    pub snprintf:                 unsafe extern "C" fn(*mut c_char, usize, *const c_char, ...) -> c_int,

    // ── NIF 2.12 — select, monitors, hash, whereis ──
    pub select:                   unsafe extern "C" fn(*mut Env, Event, SelectFlags, *mut c_void, *const Pid, Term) -> c_int,
    pub open_resource_type_x:     unsafe extern "C" fn(*mut Env, *const c_char, *const ResourceTypeInit, ResourceFlags, *mut ResourceFlags) -> *mut ResourceType,
    pub monitor_process:          unsafe extern "C" fn(*mut Env, *mut c_void, *const Pid, *mut Monitor) -> c_int,
    pub demonitor_process:        unsafe extern "C" fn(*mut Env, *mut c_void, *const Monitor) -> c_int,
    pub compare_monitors:         unsafe extern "C" fn(*const Monitor, *const Monitor) -> c_int,
    pub hash:                     unsafe extern "C" fn(Hash, Term, u64) -> u64,
    pub whereis_pid:              unsafe extern "C" fn(*mut Env, Term, *mut Pid) -> c_int,
    pub whereis_port:             unsafe extern "C" fn(*mut Env, Term, *mut Port) -> c_int,

    // ── NIF 2.13 — I/O queue ──
    pub ioq_create:               unsafe extern "C" fn(IOQueueOpts) -> *mut IOQueue,
    pub ioq_destroy:              unsafe extern "C" fn(*mut IOQueue),
    pub ioq_enq_binary:           unsafe extern "C" fn(*mut IOQueue, *mut Binary, usize) -> c_int,
    pub ioq_enqv:                 unsafe extern "C" fn(*mut IOQueue, *mut IOVec, usize) -> c_int,
    pub ioq_size:                 unsafe extern "C" fn(*mut IOQueue) -> usize,
    pub ioq_deq:                  unsafe extern "C" fn(*mut IOQueue, usize, *mut usize) -> c_int,
    pub ioq_peek:                 unsafe extern "C" fn(*mut IOQueue, *mut c_int) -> *mut SysIOVec,
    pub inspect_iovec:            unsafe extern "C" fn(*mut Env, usize, Term, *mut Term, *mut *mut IOVec) -> c_int,
    pub free_iovec:               unsafe extern "C" fn(*mut IOVec),

    // ── NIF 2.14 — ioq_peek_head, *_name, v*printf, make_map_from_arrays ──
    pub ioq_peek_head:            unsafe extern "C" fn(*mut Env, *mut IOQueue, *mut usize, *mut Term) -> c_int,
    pub mutex_name:               unsafe extern "C" fn(*mut Mutex) -> *mut c_char,
    pub cond_name:                unsafe extern "C" fn(*mut Cond) -> *mut c_char,
    pub rwlock_name:              unsafe extern "C" fn(*mut RWLock) -> *mut c_char,
    pub thread_name:              unsafe extern "C" fn(Tid) -> *mut c_char,
    /// `va_list` argument approximated as a pointer — slot kept for table order;
    /// not wrapped (Rust has no portable `va_list`). 2.14
    #[allow(dead_code)] // unwrapped (varargs/va_list); slot kept for table order
    pub vfprintf:                 unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void) -> c_int,
    /// See [`Self::vfprintf`]. Not wrapped. 2.14
    #[allow(dead_code)] // unwrapped (varargs/va_list); slot kept for table order
    pub vsnprintf:                unsafe extern "C" fn(*mut c_char, usize, *const c_char, *mut c_void) -> c_int,
    pub make_map_from_arrays:     unsafe extern "C" fn(*mut Env, *const Term, *const Term, usize, *mut Term) -> c_int,

    // ── NIF 2.15 — select_x, monitor term, pid-undefined, term_type ──
    pub select_x:                 unsafe extern "C" fn(*mut Env, Event, SelectFlags, *mut c_void, *const Pid, Term, *mut Env) -> c_int,
    pub make_monitor_term:        unsafe extern "C" fn(*mut Env, *const Monitor) -> Term,
    pub set_pid_undefined:        unsafe extern "C" fn(*mut Pid),
    pub is_pid_undefined:         unsafe extern "C" fn(*const Pid) -> c_int,
    pub term_type:                unsafe extern "C" fn(*mut Env, Term) -> c_int,

    // ── NIF 2.16 (OTP 24) ──
    #[cfg(feature = "nif_2_16")]
    pub init_resource_type:       unsafe extern "C" fn(*mut Env, *const c_char, *const ResourceTypeInit, ResourceFlags, *mut ResourceFlags) -> *mut ResourceType,
    #[cfg(feature = "nif_2_16")]
    pub dynamic_resource_call:    unsafe extern "C" fn(*mut Env, Term, Term, Term, *mut c_void) -> c_int,

    // ── NIF 2.17 (OTP 26) ──
    #[cfg(feature = "nif_2_17")]
    pub get_string_length:        unsafe extern "C" fn(*mut Env, Term, *mut c_uint, CharEncoding) -> c_int,
    #[cfg(feature = "nif_2_17")]
    pub make_new_atom:            unsafe extern "C" fn(*mut Env, *const c_char, *mut Term, CharEncoding) -> c_int,
    #[cfg(feature = "nif_2_17")]
    pub make_new_atom_len:        unsafe extern "C" fn(*mut Env, *const c_char, usize, *mut Term, CharEncoding) -> c_int,
    /// Variadic (env, opt, ...). 2.17
    #[cfg(feature = "nif_2_17")]
    pub set_option:               unsafe extern "C" fn(*mut Env, Option_, ...) -> c_int,

    // ── NIF 2.18 (OTP 29) ──
    #[cfg(feature = "nif_2_18")]
    pub term_size:                unsafe extern "C" fn(Term) -> usize,
    #[cfg(feature = "nif_2_18")]
    pub get_atom_cache_index:     unsafe extern "C" fn(*mut Env, Term, *mut c_uint) -> c_int,
    #[cfg(feature = "nif_2_18")]
    pub max_atom_cache_index:     unsafe extern "C" fn() -> c_uint,
}

// ---------------------------------------------------------------------------
// Global storage
// ---------------------------------------------------------------------------

pub(crate) static API: OnceLock<Api> = OnceLock::new();

/// The resolved function table. Panics if the loader (`unix::init` /
/// `windows::init`) has not run.
#[inline]
pub(crate) fn api() -> &'static Api {
    API.get()
        .expect("enif_ffi: not initialized — init() was not called")
}
