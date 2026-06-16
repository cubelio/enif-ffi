//! The `enif_*` wrapper functions — the public API of the crate.
//!
//! Each is a thin, `#[inline]`, all-`unsafe` forwarder to the corresponding
//! pointer in the resolved [`api`] table, with the `enif_` prefix dropped.
//! [`init`](crate::init) must have run first.
//!
//! Functions added after the 2.15 floor are gated behind their rung. The C
//! macros that have no exported symbol (`make_tupleN`, `make_listN`,
//! `select_read`/`write`/`error`, `make_pid`, `compare_pids`, the per-option
//! `set_option_*`) are reimplemented here in terms of the real functions they
//! expand to.
//!
//! The variadic `printf` family (`enif_fprintf`/`snprintf`/`vfprintf`/
//! `vsnprintf`) is intentionally not wrapped — Rust cannot forward C varargs or
//! a `va_list` — though their slots are kept in the table for ABI order.

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

use crate::ffi::api;
use crate::types::*;

// ===========================================================================
// NIF 0.1 / 1.0 — core term, binary, integer, atom, list/tuple
// ===========================================================================

/// Pointer to the library's private data. NIF 1.0. Wraps `enif_priv_data`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn priv_data(env: *mut Env) -> *mut c_void {
    unsafe { (api().priv_data)(env) }
}

/// Allocate `size` bytes; `NULL` on failure. NIF 1.0. Wraps `enif_alloc`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn alloc(size: usize) -> *mut c_void {
    unsafe { (api().alloc)(size) }
}

/// Free memory from [`alloc`]. NIF 1.0. Wraps `enif_free`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn free(ptr: *mut c_void) {
    unsafe { (api().free)(ptr) }
}

/// Non-zero if `term` is an atom. NIF 0.1. Wraps `enif_is_atom`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_atom(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_atom)(env, term) }
}

/// Non-zero if `term` is a binary. NIF 0.1. Wraps `enif_is_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_binary(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_binary)(env, term) }
}

/// Non-zero if `term` is a reference. NIF 0.1. Wraps `enif_is_ref`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_ref(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_ref)(env, term) }
}

/// Inspect a binary `term` into `bin`; non-zero on success. NIF 0.1. Wraps `enif_inspect_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn inspect_binary(env: *mut Env, term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().inspect_binary)(env, term, bin) }
}

/// Allocate a binary of `size` bytes; non-zero on success. NIF 0.1. Wraps `enif_alloc_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn alloc_binary(size: usize, bin: *mut Binary) -> c_int {
    unsafe { (api().alloc_binary)(size, bin) }
}

/// Resize `bin`; non-zero on success. NIF 1.0. Wraps `enif_realloc_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn realloc_binary(bin: *mut Binary, size: usize) -> c_int {
    unsafe { (api().realloc_binary)(bin, size) }
}

/// Release an allocated binary. NIF 2.0. Wraps `enif_release_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn release_binary(bin: *mut Binary) {
    unsafe { (api().release_binary)(bin) }
}

/// Extract a C `int`; non-zero on success. NIF 0.1. Wraps `enif_get_int`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_int(env: *mut Env, term: Term, ip: *mut c_int) -> c_int {
    unsafe { (api().get_int)(env, term, ip) }
}

/// Extract a C `unsigned long`; non-zero on success. NIF 0.1. Wraps `enif_get_ulong`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_ulong(env: *mut Env, term: Term, ip: *mut c_ulong) -> c_int {
    unsafe { (api().get_ulong)(env, term, ip) }
}

/// Extract an `f64`; non-zero on success. NIF 0.1. Wraps `enif_get_double`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_double(env: *mut Env, term: Term, dp: *mut f64) -> c_int {
    unsafe { (api().get_double)(env, term, dp) }
}

/// Split a list cell into head and tail; non-zero on success. NIF 0.1. Wraps `enif_get_list_cell`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_list_cell(env: *mut Env, term: Term, head: *mut Term, tail: *mut Term) -> c_int {
    unsafe { (api().get_list_cell)(env, term, head, tail) }
}

/// Get a tuple's elements as a read-only array; non-zero on success. NIF 0.1. Wraps `enif_get_tuple`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_tuple(
    env: *mut Env,
    tpl: Term,
    arity: *mut c_int,
    array: *mut *const Term,
) -> c_int {
    unsafe { (api().get_tuple)(env, tpl, arity, array) }
}

/// Non-zero if the terms are identical (`=:=`). NIF 0.1. Wraps `enif_is_identical`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_identical(lhs: Term, rhs: Term) -> c_int {
    unsafe { (api().is_identical)(lhs, rhs) }
}

/// Erlang term ordering: negative / zero / positive. NIF 0.1. Wraps `enif_compare`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn compare(lhs: Term, rhs: Term) -> c_int {
    unsafe { (api().compare)(lhs, rhs) }
}

/// Make a binary term from `bin`. NIF 0.1. Wraps `enif_make_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_binary(env: *mut Env, bin: *mut Binary) -> Term {
    unsafe { (api().make_binary)(env, bin) }
}

/// Make a `badarg` exception term. NIF 0.1. Wraps `enif_make_badarg`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_badarg(env: *mut Env) -> Term {
    unsafe { (api().make_badarg)(env) }
}

/// Make an integer term from a C `int`. NIF 0.1. Wraps `enif_make_int`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_int(env: *mut Env, i: c_int) -> Term {
    unsafe { (api().make_int)(env, i) }
}

/// Make an integer term from a C `unsigned long`. NIF 0.1. Wraps `enif_make_ulong`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_ulong(env: *mut Env, i: c_ulong) -> Term {
    unsafe { (api().make_ulong)(env, i) }
}

/// Make a float term. NIF 0.1. Wraps `enif_make_double`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_double(env: *mut Env, d: f64) -> Term {
    unsafe { (api().make_double)(env, d) }
}

/// Intern and make an atom from a NUL-terminated name. NIF 0.1. Wraps `enif_make_atom`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_atom(env: *mut Env, name: *const c_char) -> Term {
    unsafe { (api().make_atom)(env, name) }
}

/// Make an existing atom; non-zero if it exists. NIF 0.1. Wraps `enif_make_existing_atom`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_existing_atom(
    env: *mut Env,
    name: *const c_char,
    atom: *mut Term,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().make_existing_atom)(env, name, atom, encoding) }
}

/// Make a list cell `[car | cdr]`. NIF 0.1. Wraps `enif_make_list_cell`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list_cell(env: *mut Env, car: Term, cdr: Term) -> Term {
    unsafe { (api().make_list_cell)(env, car, cdr) }
}

/// Make a string term from a NUL-terminated string. NIF 0.1. Wraps `enif_make_string`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_string(env: *mut Env, string: *const c_char, encoding: CharEncoding) -> Term {
    unsafe { (api().make_string)(env, string, encoding) }
}

/// Make a new reference. NIF 0.1. Wraps `enif_make_ref`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_ref(env: *mut Env) -> Term {
    unsafe { (api().make_ref)(env) }
}

// ===========================================================================
// NIF 1.0 — thread primitives
// ===========================================================================

/// Create a mutex. NIF 1.0. Wraps `enif_mutex_create`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn mutex_create(name: *mut c_char) -> *mut Mutex {
    unsafe { (api().mutex_create)(name) }
}

/// Destroy a mutex. NIF 1.0. Wraps `enif_mutex_destroy`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn mutex_destroy(mtx: *mut Mutex) {
    unsafe { (api().mutex_destroy)(mtx) }
}

/// Try to lock a mutex; zero on success. NIF 1.0. Wraps `enif_mutex_trylock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn mutex_trylock(mtx: *mut Mutex) -> c_int {
    unsafe { (api().mutex_trylock)(mtx) }
}

/// Lock a mutex. NIF 1.0. Wraps `enif_mutex_lock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn mutex_lock(mtx: *mut Mutex) {
    unsafe { (api().mutex_lock)(mtx) }
}

/// Unlock a mutex. NIF 1.0. Wraps `enif_mutex_unlock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn mutex_unlock(mtx: *mut Mutex) {
    unsafe { (api().mutex_unlock)(mtx) }
}

/// Create a condition variable. NIF 1.0. Wraps `enif_cond_create`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn cond_create(name: *mut c_char) -> *mut Cond {
    unsafe { (api().cond_create)(name) }
}

/// Destroy a condition variable. NIF 1.0. Wraps `enif_cond_destroy`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn cond_destroy(cnd: *mut Cond) {
    unsafe { (api().cond_destroy)(cnd) }
}

/// Signal one waiter. NIF 1.0. Wraps `enif_cond_signal`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn cond_signal(cnd: *mut Cond) {
    unsafe { (api().cond_signal)(cnd) }
}

/// Broadcast to all waiters. NIF 1.0. Wraps `enif_cond_broadcast`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn cond_broadcast(cnd: *mut Cond) {
    unsafe { (api().cond_broadcast)(cnd) }
}

/// Wait on a condition variable. NIF 1.0. Wraps `enif_cond_wait`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn cond_wait(cnd: *mut Cond, mtx: *mut Mutex) {
    unsafe { (api().cond_wait)(cnd, mtx) }
}

/// Create a read-write lock. NIF 1.0. Wraps `enif_rwlock_create`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_create(name: *mut c_char) -> *mut RWLock {
    unsafe { (api().rwlock_create)(name) }
}

/// Destroy a read-write lock. NIF 1.0. Wraps `enif_rwlock_destroy`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_destroy(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_destroy)(rwlck) }
}

/// Try to read-lock; zero on success. NIF 1.0. Wraps `enif_rwlock_tryrlock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_tryrlock(rwlck: *mut RWLock) -> c_int {
    unsafe { (api().rwlock_tryrlock)(rwlck) }
}

/// Read-lock. NIF 1.0. Wraps `enif_rwlock_rlock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_rlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rlock)(rwlck) }
}

/// Read-unlock. NIF 1.0. Wraps `enif_rwlock_runlock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_runlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_runlock)(rwlck) }
}

/// Try to write-lock; zero on success. NIF 1.0. Wraps `enif_rwlock_tryrwlock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_tryrwlock(rwlck: *mut RWLock) -> c_int {
    unsafe { (api().rwlock_tryrwlock)(rwlck) }
}

/// Write-lock. NIF 1.0. Wraps `enif_rwlock_rwlock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_rwlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rwlock)(rwlck) }
}

/// Write-unlock. NIF 1.0. Wraps `enif_rwlock_rwunlock`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_rwunlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rwunlock)(rwlck) }
}

/// Create a thread-specific data key; zero on success. NIF 1.0. Wraps `enif_tsd_key_create`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn tsd_key_create(name: *mut c_char, key: *mut TSDKey) -> c_int {
    unsafe { (api().tsd_key_create)(name, key) }
}

/// Destroy a thread-specific data key. NIF 1.0. Wraps `enif_tsd_key_destroy`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn tsd_key_destroy(key: TSDKey) {
    unsafe { (api().tsd_key_destroy)(key) }
}

/// Set thread-specific data. NIF 1.0. Wraps `enif_tsd_set`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn tsd_set(key: TSDKey, data: *mut c_void) {
    unsafe { (api().tsd_set)(key, data) }
}

/// Get thread-specific data. NIF 1.0. Wraps `enif_tsd_get`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn tsd_get(key: TSDKey) -> *mut c_void {
    unsafe { (api().tsd_get)(key) }
}

/// Create thread options. NIF 1.0. Wraps `enif_thread_opts_create`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn thread_opts_create(name: *mut c_char) -> *mut ThreadOpts {
    unsafe { (api().thread_opts_create)(name) }
}

/// Destroy thread options. NIF 1.0. Wraps `enif_thread_opts_destroy`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn thread_opts_destroy(opts: *mut ThreadOpts) {
    unsafe { (api().thread_opts_destroy)(opts) }
}

/// Create a thread; zero on success. NIF 1.0. Wraps `enif_thread_create`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn thread_create(
    name: *mut c_char,
    tid: *mut Tid,
    func: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    args: *mut c_void,
    opts: *mut ThreadOpts,
) -> c_int {
    unsafe { (api().thread_create)(name, tid, func, args, opts) }
}

/// The calling thread's id. NIF 1.0. Wraps `enif_thread_self`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn thread_self() -> Tid {
    unsafe { (api().thread_self)() }
}

/// Non-zero if the two thread ids are equal. NIF 1.0. Wraps `enif_equal_tids`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn equal_tids(tid1: Tid, tid2: Tid) -> c_int {
    unsafe { (api().equal_tids)(tid1, tid2) }
}

/// Exit the calling thread. NIF 1.0. Wraps `enif_thread_exit`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn thread_exit(resp: *mut c_void) {
    unsafe { (api().thread_exit)(resp) }
}

/// Join a thread; zero on success. NIF 1.0. Wraps `enif_thread_join`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn thread_join(tid: Tid, respp: *mut *mut c_void) -> c_int {
    unsafe { (api().thread_join)(tid, respp) }
}

// ===========================================================================
// NIF 1.0 / 2.0 — more core, resources, strings, env, send
// ===========================================================================

/// Reallocate memory; `NULL` on failure. NIF 1.0. Wraps `enif_realloc`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    unsafe { (api().realloc)(ptr, size) }
}

/// Fill in BEAM system information. NIF 1.0. Wraps `enif_system_info`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn system_info(sip: *mut SysInfo, si_size: usize) {
    unsafe { (api().system_info)(sip, si_size) }
}

/// Inspect an iolist as a contiguous binary; non-zero on success. NIF 1.0. Wraps `enif_inspect_iolist_as_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn inspect_iolist_as_binary(env: *mut Env, term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().inspect_iolist_as_binary)(env, term, bin) }
}

/// Make a sub-binary of `bin_term`. NIF 1.0. Wraps `enif_make_sub_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_sub_binary(env: *mut Env, bin_term: Term, pos: usize, size: usize) -> Term {
    unsafe { (api().make_sub_binary)(env, bin_term, pos, size) }
}

/// Copy a string list into `buf`; chars written, or 0. NIF 1.0. Wraps `enif_get_string`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_string(
    env: *mut Env,
    list: Term,
    buf: *mut c_char,
    len: c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_string)(env, list, buf, len, encoding) }
}

/// Copy an atom's name into `buf`; chars written, or 0. NIF 1.0. Wraps `enif_get_atom`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_atom(
    env: *mut Env,
    atom: Term,
    buf: *mut c_char,
    len: c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_atom)(env, atom, buf, len, encoding) }
}

/// Non-zero if `term` is a fun. NIF 1.0. Wraps `enif_is_fun`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_fun(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_fun)(env, term) }
}

/// Non-zero if `term` is a pid. NIF 1.0. Wraps `enif_is_pid`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_pid(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_pid)(env, term) }
}

/// Non-zero if `term` is a port. NIF 1.0. Wraps `enif_is_port`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_port(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_port)(env, term) }
}

/// Extract a C `unsigned`; non-zero on success. NIF 1.0. Wraps `enif_get_uint`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_uint(env: *mut Env, term: Term, ip: *mut c_uint) -> c_int {
    unsafe { (api().get_uint)(env, term, ip) }
}

/// Extract a C `long`; non-zero on success. NIF 1.0. Wraps `enif_get_long`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_long(env: *mut Env, term: Term, ip: *mut c_long) -> c_int {
    unsafe { (api().get_long)(env, term, ip) }
}

/// Make an integer term from a C `unsigned`. NIF 1.0. Wraps `enif_make_uint`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_uint(env: *mut Env, i: c_uint) -> Term {
    unsafe { (api().make_uint)(env, i) }
}

/// Make an integer term from a C `long`. NIF 1.0. Wraps `enif_make_long`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_long(env: *mut Env, i: c_long) -> Term {
    unsafe { (api().make_long)(env, i) }
}

/// Make a tuple from an array of terms. NIF 1.0. Wraps `enif_make_tuple_from_array`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple_from_array(env: *mut Env, arr: *const Term, cnt: c_uint) -> Term {
    unsafe { (api().make_tuple_from_array)(env, arr, cnt) }
}

/// Make a list from an array of terms. NIF 1.0. Wraps `enif_make_list_from_array`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list_from_array(env: *mut Env, arr: *const Term, cnt: c_uint) -> Term {
    unsafe { (api().make_list_from_array)(env, arr, cnt) }
}

/// Non-zero if `term` is the empty list. NIF 1.0. Wraps `enif_is_empty_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_empty_list(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_empty_list)(env, term) }
}

/// Open (create/take over) a resource type. NIF 1.0. Wraps `enif_open_resource_type`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn open_resource_type(
    env: *mut Env,
    module_str: *const c_char,
    name_str: *const c_char,
    dtor: Option<unsafe extern "C" fn(*mut Env, *mut c_void)>,
    flags: ResourceFlags,
    tried: *mut ResourceFlags,
) -> *mut ResourceType {
    unsafe { (api().open_resource_type)(env, module_str, name_str, dtor, flags, tried) }
}

/// Allocate a resource object. NIF 1.0. Wraps `enif_alloc_resource`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn alloc_resource(ty: *mut ResourceType, size: usize) -> *mut c_void {
    unsafe { (api().alloc_resource)(ty, size) }
}

/// Release a resource reference. NIF 1.0. Wraps `enif_release_resource`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn release_resource(obj: *mut c_void) {
    unsafe { (api().release_resource)(obj) }
}

/// Make a resource term. NIF 1.0. Wraps `enif_make_resource`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_resource(env: *mut Env, obj: *mut c_void) -> Term {
    unsafe { (api().make_resource)(env, obj) }
}

/// Get a resource of the given type; non-zero on success. NIF 1.0. Wraps `enif_get_resource`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_resource(
    env: *mut Env,
    term: Term,
    ty: *mut ResourceType,
    objp: *mut *mut c_void,
) -> c_int {
    unsafe { (api().get_resource)(env, term, ty, objp) }
}

/// Size in bytes of a resource object. NIF 1.0. Wraps `enif_sizeof_resource`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn sizeof_resource(obj: *mut c_void) -> usize {
    unsafe { (api().sizeof_resource)(obj) }
}

/// Allocate a new binary term and return a writable data pointer. NIF 1.0. Wraps `enif_make_new_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_new_binary(env: *mut Env, size: usize, termp: *mut Term) -> *mut u8 {
    unsafe { (api().make_new_binary)(env, size, termp) }
}

/// Non-zero if `term` is a list. NIF 2.0. Wraps `enif_is_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_list(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_list)(env, term) }
}

/// Non-zero if `term` is a tuple. NIF 2.0. Wraps `enif_is_tuple`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_tuple(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_tuple)(env, term) }
}

/// Length (in bytes) of an atom's name; non-zero on success. NIF 2.0. Wraps `enif_get_atom_length`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_atom_length(
    env: *mut Env,
    atom: Term,
    len: *mut c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_atom_length)(env, atom, len, encoding) }
}

/// Length of a proper list; non-zero on success. NIF 2.0. Wraps `enif_get_list_length`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_list_length(env: *mut Env, term: Term, len: *mut c_uint) -> c_int {
    unsafe { (api().get_list_length)(env, term, len) }
}

/// Make an atom from a name with explicit length. NIF 2.0. Wraps `enif_make_atom_len`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_atom_len(env: *mut Env, name: *const c_char, len: usize) -> Term {
    unsafe { (api().make_atom_len)(env, name, len) }
}

/// Make an existing atom with explicit length; non-zero if it exists. NIF 2.0. Wraps `enif_make_existing_atom_len`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_existing_atom_len(
    env: *mut Env,
    name: *const c_char,
    len: usize,
    atom: *mut Term,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().make_existing_atom_len)(env, name, len, atom, encoding) }
}

/// Make a string term with explicit length. NIF 2.0. Wraps `enif_make_string_len`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_string_len(
    env: *mut Env,
    string: *const c_char,
    len: usize,
    encoding: CharEncoding,
) -> Term {
    unsafe { (api().make_string_len)(env, string, len, encoding) }
}

/// Allocate a process-independent environment. NIF 2.0. Wraps `enif_alloc_env`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn alloc_env() -> *mut Env {
    unsafe { (api().alloc_env)() }
}

/// Free a process-independent environment. NIF 2.0. Wraps `enif_free_env`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn free_env(env: *mut Env) {
    unsafe { (api().free_env)(env) }
}

/// Clear (reuse) a process-independent environment. NIF 2.0. Wraps `enif_clear_env`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn clear_env(env: *mut Env) {
    unsafe { (api().clear_env)(env) }
}

/// Send a message to a process; non-zero on success. NIF 2.0. Wraps `enif_send`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn send(env: *mut Env, to_pid: *const Pid, msg_env: *mut Env, msg: Term) -> c_int {
    unsafe { (api().send)(env, to_pid, msg_env, msg) }
}

/// Copy a term into another environment. NIF 2.0. Wraps `enif_make_copy`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_copy(dst_env: *mut Env, src_term: Term) -> Term {
    unsafe { (api().make_copy)(dst_env, src_term) }
}

/// The calling process's pid (into `pid`). NIF 2.0. Wraps `enif_self`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn self_(caller_env: *mut Env, pid: *mut Pid) -> *mut Pid {
    unsafe { (api().self_)(caller_env, pid) }
}

/// Extract a local pid from a term; non-zero on success. NIF 2.0. Wraps `enif_get_local_pid`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_local_pid(env: *mut Env, term: Term, pid: *mut Pid) -> c_int {
    unsafe { (api().get_local_pid)(env, term, pid) }
}

/// Add a reference to a resource object. NIF 2.0. Wraps `enif_keep_resource`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn keep_resource(obj: *mut c_void) {
    unsafe { (api().keep_resource)(obj) }
}

/// Make a binary term backed by resource memory. NIF 2.0. Wraps `enif_make_resource_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_resource_binary(
    env: *mut Env,
    obj: *mut c_void,
    data: *const c_void,
    size: usize,
) -> Term {
    unsafe { (api().make_resource_binary)(env, obj, data, size) }
}

/// Extract a 64-bit signed integer; non-zero on success. NIF 2.0. Wraps `enif_get_int64`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_int64(env: *mut Env, term: Term, ip: *mut i64) -> c_int {
    unsafe { (api().get_int64)(env, term, ip) }
}

/// Extract a 64-bit unsigned integer; non-zero on success. NIF 2.0. Wraps `enif_get_uint64`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_uint64(env: *mut Env, term: Term, ip: *mut u64) -> c_int {
    unsafe { (api().get_uint64)(env, term, ip) }
}

/// Make a term from a 64-bit signed integer. NIF 2.0. Wraps `enif_make_int64`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_int64(env: *mut Env, i: i64) -> Term {
    unsafe { (api().make_int64)(env, i) }
}

/// Make a term from a 64-bit unsigned integer. NIF 2.0. Wraps `enif_make_uint64`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_uint64(env: *mut Env, i: u64) -> Term {
    unsafe { (api().make_uint64)(env, i) }
}

// ===========================================================================
// NIF 2.2 – 2.4
// ===========================================================================

/// Non-zero if `term` is an exception. NIF 2.2. Wraps `enif_is_exception`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_exception(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_exception)(env, term) }
}

/// Reverse a proper list; non-zero on success. NIF 2.3. Wraps `enif_make_reverse_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_reverse_list(env: *mut Env, term: Term, list: *mut Term) -> c_int {
    unsafe { (api().make_reverse_list)(env, term, list) }
}

/// Non-zero if `term` is a number. NIF 2.3. Wraps `enif_is_number`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_number(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_number)(env, term) }
}

/// `dlopen` a shared library. NIF 2.4. Wraps `enif_dlopen`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn dlopen(
    lib: *const c_char,
    err_handler: Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
    err_arg: *mut c_void,
) -> *mut c_void {
    unsafe { (api().dlopen)(lib, err_handler, err_arg) }
}

/// `dlsym` a symbol. NIF 2.4. Wraps `enif_dlsym`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn dlsym(
    handle: *mut c_void,
    symbol: *const c_char,
    err_handler: Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
    err_arg: *mut c_void,
) -> *mut c_void {
    unsafe { (api().dlsym)(handle, symbol, err_handler, err_arg) }
}

/// Report consumed timeslice; non-zero if the NIF should yield. NIF 2.4. Wraps `enif_consume_timeslice`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn consume_timeslice(env: *mut Env, percent: c_int) -> c_int {
    unsafe { (api().consume_timeslice)(env, percent) }
}

// ===========================================================================
// NIF 2.6 — maps
// ===========================================================================

/// Non-zero if `term` is a map. NIF 2.6. Wraps `enif_is_map`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_map(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_map)(env, term) }
}

/// Number of pairs in a map; non-zero on success. NIF 2.6. Wraps `enif_get_map_size`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_map_size(env: *mut Env, term: Term, size: *mut usize) -> c_int {
    unsafe { (api().get_map_size)(env, term, size) }
}

/// Make an empty map. NIF 2.6. Wraps `enif_make_new_map`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_new_map(env: *mut Env) -> Term {
    unsafe { (api().make_new_map)(env) }
}

/// Insert/overwrite a key; non-zero on success. NIF 2.6. Wraps `enif_make_map_put`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_map_put(
    env: *mut Env,
    map_in: Term,
    key: Term,
    value: Term,
    map_out: *mut Term,
) -> c_int {
    unsafe { (api().make_map_put)(env, map_in, key, value, map_out) }
}

/// Look up a key; non-zero if found. NIF 2.6. Wraps `enif_get_map_value`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_map_value(env: *mut Env, map: Term, key: Term, value: *mut Term) -> c_int {
    unsafe { (api().get_map_value)(env, map, key, value) }
}

/// Update an existing key; non-zero if it existed. NIF 2.6. Wraps `enif_make_map_update`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_map_update(
    env: *mut Env,
    map_in: Term,
    key: Term,
    value: Term,
    map_out: *mut Term,
) -> c_int {
    unsafe { (api().make_map_update)(env, map_in, key, value, map_out) }
}

/// Remove a key; non-zero on success. NIF 2.6. Wraps `enif_make_map_remove`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_map_remove(env: *mut Env, map_in: Term, key: Term, map_out: *mut Term) -> c_int {
    unsafe { (api().make_map_remove)(env, map_in, key, map_out) }
}

/// Create a map iterator; non-zero on success. NIF 2.6. Wraps `enif_map_iterator_create`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn map_iterator_create(
    env: *mut Env,
    map: Term,
    iter: *mut MapIterator,
    entry: MapIteratorEntry,
) -> c_int {
    unsafe { (api().map_iterator_create)(env, map, iter, entry) }
}

/// Destroy a map iterator. NIF 2.6. Wraps `enif_map_iterator_destroy`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn map_iterator_destroy(env: *mut Env, iter: *mut MapIterator) {
    unsafe { (api().map_iterator_destroy)(env, iter) }
}

/// Non-zero if the iterator is at the head sentinel. NIF 2.6. Wraps `enif_map_iterator_is_head`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn map_iterator_is_head(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_is_head)(env, iter) }
}

/// Non-zero if the iterator is at the tail sentinel. NIF 2.6. Wraps `enif_map_iterator_is_tail`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn map_iterator_is_tail(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_is_tail)(env, iter) }
}

/// Advance the iterator; non-zero if positioned on a pair. NIF 2.6. Wraps `enif_map_iterator_next`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn map_iterator_next(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_next)(env, iter) }
}

/// Step the iterator back; non-zero if positioned on a pair. NIF 2.6. Wraps `enif_map_iterator_prev`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn map_iterator_prev(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_prev)(env, iter) }
}

/// Read the current key/value pair; non-zero on success. NIF 2.6. Wraps `enif_map_iterator_get_pair`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn map_iterator_get_pair(
    env: *mut Env,
    iter: *mut MapIterator,
    key: *mut Term,
    value: *mut Term,
) -> c_int {
    unsafe { (api().map_iterator_get_pair)(env, iter, key, value) }
}

// ===========================================================================
// NIF 2.7 – 2.11
// ===========================================================================

/// Reschedule a NIF (e.g. onto a dirty scheduler). NIF 2.7. Wraps `enif_schedule_nif`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn schedule_nif(
    env: *mut Env,
    fun_name: *const c_char,
    flags: c_int,
    fp: unsafe extern "C" fn(*mut Env, c_int, *const Term) -> Term,
    argc: c_int,
    argv: *const Term,
) -> Term {
    unsafe { (api().schedule_nif)(env, fun_name, flags, fp, argc, argv) }
}

/// Non-zero if a pending exception exists (and store its reason). NIF 2.8. Wraps `enif_has_pending_exception`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn has_pending_exception(env: *mut Env, reason: *mut Term) -> c_int {
    unsafe { (api().has_pending_exception)(env, reason) }
}

/// Raise an exception with the given reason. NIF 2.8. Wraps `enif_raise_exception`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn raise_exception(env: *mut Env, reason: Term) -> Term {
    unsafe { (api().raise_exception)(env, reason) }
}

/// Read an OS environment variable; non-zero on success. NIF 2.9. Wraps `enif_getenv`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn getenv(key: *const c_char, value: *mut c_char, value_size: *mut usize) -> c_int {
    unsafe { (api().getenv)(key, value, value_size) }
}

/// Erlang monotonic time in `unit`. NIF 2.10. Wraps `enif_monotonic_time`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn monotonic_time(unit: TimeUnit) -> Time {
    unsafe { (api().monotonic_time)(unit) }
}

/// Current time offset in `unit`. NIF 2.10. Wraps `enif_time_offset`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn time_offset(unit: TimeUnit) -> Time {
    unsafe { (api().time_offset)(unit) }
}

/// Convert a time value between units. NIF 2.10. Wraps `enif_convert_time_unit`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn convert_time_unit(time: Time, from_unit: TimeUnit, to_unit: TimeUnit) -> Time {
    unsafe { (api().convert_time_unit)(time, from_unit, to_unit) }
}

/// `os:timestamp/0` as a term. NIF 2.11. Wraps `enif_now_time`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn now_time(env: *mut Env) -> Term {
    unsafe { (api().now_time)(env) }
}

/// `os:perf_counter`-style CPU time as a term. NIF 2.11. Wraps `enif_cpu_time`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn cpu_time(env: *mut Env) -> Term {
    unsafe { (api().cpu_time)(env) }
}

/// Make a unique integer term. NIF 2.11. Wraps `enif_make_unique_integer`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_unique_integer(env: *mut Env, properties: UniqueInteger) -> Term {
    unsafe { (api().make_unique_integer)(env, properties) }
}

/// Non-zero if the calling process is alive. NIF 2.11. Wraps `enif_is_current_process_alive`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_current_process_alive(env: *mut Env) -> c_int {
    unsafe { (api().is_current_process_alive)(env) }
}

/// Non-zero if the given process is alive. NIF 2.11. Wraps `enif_is_process_alive`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_process_alive(env: *mut Env, pid: *const Pid) -> c_int {
    unsafe { (api().is_process_alive)(env, pid) }
}

/// Non-zero if the given port is alive. NIF 2.11. Wraps `enif_is_port_alive`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_port_alive(env: *mut Env, port_id: *const Port) -> c_int {
    unsafe { (api().is_port_alive)(env, port_id) }
}

/// Extract a local port from a term; non-zero on success. NIF 2.11. Wraps `enif_get_local_port`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_local_port(env: *mut Env, term: Term, port_id: *mut Port) -> c_int {
    unsafe { (api().get_local_port)(env, term, port_id) }
}

/// Serialize a term to the external format; non-zero on success. NIF 2.11. Wraps `enif_term_to_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn term_to_binary(env: *mut Env, term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().term_to_binary)(env, term, bin) }
}

/// Deserialize a term from the external format; bytes read, or 0. NIF 2.11. Wraps `enif_binary_to_term`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn binary_to_term(
    env: *mut Env,
    data: *const u8,
    sz: usize,
    term: *mut Term,
    opts: c_uint,
) -> usize {
    unsafe { (api().binary_to_term)(env, data, sz, term, opts) }
}

/// Send a command to a port; non-zero on success. NIF 2.11. Wraps `enif_port_command`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn port_command(
    env: *mut Env,
    to_port: *const Port,
    msg_env: *mut Env,
    msg: Term,
) -> c_int {
    unsafe { (api().port_command)(env, to_port, msg_env, msg) }
}

/// The calling thread's scheduler type. NIF 2.11. Wraps `enif_thread_type`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn thread_type() -> c_int {
    unsafe { (api().thread_type)() }
}

// ===========================================================================
// NIF 2.12 — select, monitors, hash, whereis
// ===========================================================================

/// Register for I/O readiness notification; bitmask result. NIF 2.12. Wraps `enif_select`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn select(
    env: *mut Env,
    e: Event,
    flags: SelectFlags,
    obj: *mut c_void,
    pid: *const Pid,
    eref: Term,
) -> c_int {
    unsafe { (api().select)(env, e, flags, obj, pid, eref) }
}

/// Open a resource type with a full callback table. NIF 2.12. Wraps `enif_open_resource_type_x`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn open_resource_type_x(
    env: *mut Env,
    name_str: *const c_char,
    init: *const ResourceTypeInit,
    flags: ResourceFlags,
    tried: *mut ResourceFlags,
) -> *mut ResourceType {
    unsafe { (api().open_resource_type_x)(env, name_str, init, flags, tried) }
}

/// Monitor a process from a resource; zero on success. NIF 2.12. Wraps `enif_monitor_process`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn monitor_process(
    env: *mut Env,
    obj: *mut c_void,
    pid: *const Pid,
    monitor: *mut Monitor,
) -> c_int {
    unsafe { (api().monitor_process)(env, obj, pid, monitor) }
}

/// Cancel a monitor; zero on success. NIF 2.12. Wraps `enif_demonitor_process`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn demonitor_process(env: *mut Env, obj: *mut c_void, monitor: *const Monitor) -> c_int {
    unsafe { (api().demonitor_process)(env, obj, monitor) }
}

/// Compare two monitors. NIF 2.12. Wraps `enif_compare_monitors`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn compare_monitors(monitor1: *const Monitor, monitor2: *const Monitor) -> c_int {
    unsafe { (api().compare_monitors)(monitor1, monitor2) }
}

/// Hash a term with the given algorithm and salt. NIF 2.12. Wraps `enif_hash`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn hash(hashtype: Hash, term: Term, salt: u64) -> u64 {
    unsafe { (api().hash)(hashtype, term, salt) }
}

/// Resolve a registered name to a pid; non-zero if found. NIF 2.12. Wraps `enif_whereis_pid`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn whereis_pid(env: *mut Env, name: Term, pid: *mut Pid) -> c_int {
    unsafe { (api().whereis_pid)(env, name, pid) }
}

/// Resolve a registered name to a port; non-zero if found. NIF 2.12. Wraps `enif_whereis_port`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn whereis_port(env: *mut Env, name: Term, port: *mut Port) -> c_int {
    unsafe { (api().whereis_port)(env, name, port) }
}

// ===========================================================================
// NIF 2.13 — I/O queue
// ===========================================================================

/// Create an I/O queue. NIF 2.13. Wraps `enif_ioq_create`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn ioq_create(opts: IOQueueOpts) -> *mut IOQueue {
    unsafe { (api().ioq_create)(opts) }
}

/// Destroy an I/O queue. NIF 2.13. Wraps `enif_ioq_destroy`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn ioq_destroy(q: *mut IOQueue) {
    unsafe { (api().ioq_destroy)(q) }
}

/// Enqueue a binary; non-zero on success. NIF 2.13. Wraps `enif_ioq_enq_binary`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn ioq_enq_binary(q: *mut IOQueue, bin: *mut Binary, skip: usize) -> c_int {
    unsafe { (api().ioq_enq_binary)(q, bin, skip) }
}

/// Enqueue an iovec; non-zero on success. NIF 2.13. Wraps `enif_ioq_enqv`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn ioq_enqv(q: *mut IOQueue, iov: *mut IOVec, skip: usize) -> c_int {
    unsafe { (api().ioq_enqv)(q, iov, skip) }
}

/// Total queued bytes. NIF 2.13. Wraps `enif_ioq_size`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn ioq_size(q: *mut IOQueue) -> usize {
    unsafe { (api().ioq_size)(q) }
}

/// Dequeue `count` bytes; non-zero on success. NIF 2.13. Wraps `enif_ioq_deq`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn ioq_deq(q: *mut IOQueue, count: usize, size: *mut usize) -> c_int {
    unsafe { (api().ioq_deq)(q, count, size) }
}

/// Peek the queue as an iovec array. NIF 2.13. Wraps `enif_ioq_peek`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn ioq_peek(q: *mut IOQueue, iovlen: *mut c_int) -> *mut SysIOVec {
    unsafe { (api().ioq_peek)(q, iovlen) }
}

/// Inspect a term as an iovec; non-zero on success. NIF 2.13. Wraps `enif_inspect_iovec`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn inspect_iovec(
    env: *mut Env,
    max_length: usize,
    iovec_term: Term,
    tail: *mut Term,
    iovec: *mut *mut IOVec,
) -> c_int {
    unsafe { (api().inspect_iovec)(env, max_length, iovec_term, tail, iovec) }
}

/// Free an iovec from [`inspect_iovec`]. NIF 2.13. Wraps `enif_free_iovec`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn free_iovec(iov: *mut IOVec) {
    unsafe { (api().free_iovec)(iov) }
}

// ===========================================================================
// NIF 2.14 — ioq_peek_head, *_name, make_map_from_arrays
// ===========================================================================

/// Peek the head of an I/O queue; non-zero on success. NIF 2.14. Wraps `enif_ioq_peek_head`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn ioq_peek_head(
    env: *mut Env,
    q: *mut IOQueue,
    size: *mut usize,
    head: *mut Term,
) -> c_int {
    unsafe { (api().ioq_peek_head)(env, q, size, head) }
}

/// A mutex's name. NIF 2.14. Wraps `enif_mutex_name`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn mutex_name(mtx: *mut Mutex) -> *mut c_char {
    unsafe { (api().mutex_name)(mtx) }
}

/// A condition variable's name. NIF 2.14. Wraps `enif_cond_name`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn cond_name(cnd: *mut Cond) -> *mut c_char {
    unsafe { (api().cond_name)(cnd) }
}

/// A read-write lock's name. NIF 2.14. Wraps `enif_rwlock_name`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn rwlock_name(rwlck: *mut RWLock) -> *mut c_char {
    unsafe { (api().rwlock_name)(rwlck) }
}

/// A thread's name. NIF 2.14. Wraps `enif_thread_name`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn thread_name(tid: Tid) -> *mut c_char {
    unsafe { (api().thread_name)(tid) }
}

/// Make a map from parallel key/value arrays; non-zero on success. NIF 2.14. Wraps `enif_make_map_from_arrays`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_map_from_arrays(
    env: *mut Env,
    keys: *const Term,
    values: *const Term,
    cnt: usize,
    map_out: *mut Term,
) -> c_int {
    unsafe { (api().make_map_from_arrays)(env, keys, values, cnt, map_out) }
}

// ===========================================================================
// NIF 2.15 — select_x, monitor term, pid-undefined, term_type
// ===========================================================================

/// Generalized [`select`] with an explicit message and message env. NIF 2.15. Wraps `enif_select_x`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn select_x(
    env: *mut Env,
    e: Event,
    flags: SelectFlags,
    obj: *mut c_void,
    pid: *const Pid,
    msg: Term,
    msg_env: *mut Env,
) -> c_int {
    unsafe { (api().select_x)(env, e, flags, obj, pid, msg, msg_env) }
}

/// Make a term identifying a monitor. NIF 2.15. Wraps `enif_make_monitor_term`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_monitor_term(env: *mut Env, mon: *const Monitor) -> Term {
    unsafe { (api().make_monitor_term)(env, mon) }
}

/// Set a pid to the undefined value. NIF 2.15. Wraps `enif_set_pid_undefined`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn set_pid_undefined(pid: *mut Pid) {
    unsafe { (api().set_pid_undefined)(pid) }
}

/// Non-zero if a pid is undefined. NIF 2.15. Wraps `enif_is_pid_undefined`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn is_pid_undefined(pid: *const Pid) -> c_int {
    unsafe { (api().is_pid_undefined)(pid) }
}

/// Raw term-type code (map with [`TermType::from_raw`]). NIF 2.15. Wraps `enif_term_type`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn term_type(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().term_type)(env, term) }
}

// ===========================================================================
// NIF 2.16 (OTP 24)
// ===========================================================================

/// Create a resource type with a callback table. NIF 2.16. Wraps `enif_init_resource_type`.
#[cfg(feature = "nif_2_16")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn init_resource_type(
    env: *mut Env,
    name_str: *const c_char,
    init: *const ResourceTypeInit,
    flags: ResourceFlags,
    tried: *mut ResourceFlags,
) -> *mut ResourceType {
    unsafe { (api().init_resource_type)(env, name_str, init, flags, tried) }
}

/// Call a resource's dynamic-call callback; zero on success. NIF 2.16. Wraps `enif_dynamic_resource_call`.
#[cfg(feature = "nif_2_16")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn dynamic_resource_call(
    env: *mut Env,
    module: Term,
    name: Term,
    rsrc: Term,
    call_data: *mut c_void,
) -> c_int {
    unsafe { (api().dynamic_resource_call)(env, module, name, rsrc, call_data) }
}

// ===========================================================================
// NIF 2.17 (OTP 26)
// ===========================================================================

/// Length of a string list without extracting it; non-zero on success. NIF 2.17. Wraps `enif_get_string_length`.
#[cfg(feature = "nif_2_17")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_string_length(
    env: *mut Env,
    list: Term,
    len: *mut c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_string_length)(env, list, len, encoding) }
}

/// Make an atom, failing if it does not exist and the table is full. NIF 2.17. Wraps `enif_make_new_atom`.
#[cfg(feature = "nif_2_17")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_new_atom(
    env: *mut Env,
    name: *const c_char,
    atom: *mut Term,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().make_new_atom)(env, name, atom, encoding) }
}

/// [`make_new_atom`] with explicit length. NIF 2.17. Wraps `enif_make_new_atom_len`.
#[cfg(feature = "nif_2_17")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_new_atom_len(
    env: *mut Env,
    name: *const c_char,
    len: usize,
    atom: *mut Term,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().make_new_atom_len)(env, name, len, atom, encoding) }
}

// ===========================================================================
// NIF 2.18 (OTP 29)
// ===========================================================================

/// Size of a term's external (term_to_binary) form. NIF 2.18. Wraps `enif_term_size`.
#[cfg(feature = "nif_2_18")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn term_size(term: Term) -> usize {
    unsafe { (api().term_size)(term) }
}

/// Atom cache index of a term; non-zero on success. NIF 2.18. Wraps `enif_get_atom_cache_index`.
#[cfg(feature = "nif_2_18")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn get_atom_cache_index(env: *mut Env, atom: Term, index: *mut c_uint) -> c_int {
    unsafe { (api().get_atom_cache_index)(env, atom, index) }
}

/// Maximum atom cache index. NIF 2.18. Wraps `enif_max_atom_cache_index`.
#[cfg(feature = "nif_2_18")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn max_atom_cache_index() -> c_uint {
    unsafe { (api().max_atom_cache_index)() }
}

// ===========================================================================
// Convenience wrappers for C macros (no exported symbol)
// ===========================================================================

/// Make a pid term from a [`Pid`]. NIF 2.0. C macro `enif_make_pid` (returns `pid.pid`).
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_pid(_env: *mut Env, pid: Pid) -> Term {
    pid.pid
}

/// Compare two pids by term order. NIF 2.0. C macro `enif_compare_pids` (`compare(a.pid, b.pid)`).
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn compare_pids(a: *const Pid, b: *const Pid) -> c_int {
    unsafe { compare((*a).pid, (*b).pid) }
}

/// Custom-message read [`select_x`]. NIF 2.15. C macro `enif_select_read`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn select_read(
    env: *mut Env,
    e: Event,
    obj: *mut c_void,
    pid: *const Pid,
    msg: Term,
    msg_env: *mut Env,
) -> c_int {
    unsafe {
        select_x(
            env,
            e,
            SelectFlags::READ | SelectFlags::CUSTOM_MSG,
            obj,
            pid,
            msg,
            msg_env,
        )
    }
}

/// Custom-message write [`select_x`]. NIF 2.15. C macro `enif_select_write`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn select_write(
    env: *mut Env,
    e: Event,
    obj: *mut c_void,
    pid: *const Pid,
    msg: Term,
    msg_env: *mut Env,
) -> c_int {
    unsafe {
        select_x(
            env,
            e,
            SelectFlags::WRITE | SelectFlags::CUSTOM_MSG,
            obj,
            pid,
            msg,
            msg_env,
        )
    }
}

/// Custom-message error [`select_x`]. NIF 2.16. C macro `enif_select_error`.
#[cfg(feature = "nif_2_16")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn select_error(
    env: *mut Env,
    e: Event,
    obj: *mut c_void,
    pid: *const Pid,
    msg: Term,
    msg_env: *mut Env,
) -> c_int {
    unsafe {
        select_x(
            env,
            e,
            SelectFlags::ERROR | SelectFlags::CUSTOM_MSG,
            obj,
            pid,
            msg,
            msg_env,
        )
    }
}

/// Set the [`Option_::DelayHalt`] option. NIF 2.17. Wraps variadic `enif_set_option`.
#[cfg(feature = "nif_2_17")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn set_option_delay_halt(env: *mut Env) -> c_int {
    unsafe { (api().set_option)(env, Option_::DelayHalt) }
}

/// Install an on-halt callback ([`Option_::OnHalt`]). NIF 2.17. Wraps variadic `enif_set_option`.
#[cfg(feature = "nif_2_17")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn set_option_on_halt(
    env: *mut Env,
    on_halt: unsafe extern "C" fn(*mut c_void),
) -> c_int {
    unsafe { (api().set_option)(env, Option_::OnHalt, on_halt) }
}

/// Install an on-unload-thread callback ([`Option_::OnUnloadThread`]). NIF 2.17 (OTP 27). Wraps variadic `enif_set_option`.
#[cfg(feature = "nif_2_17")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn set_option_on_unload_thread(
    env: *mut Env,
    on_unload_thread: unsafe extern "C" fn(*mut c_void),
) -> c_int {
    unsafe { (api().set_option)(env, Option_::OnUnloadThread, on_unload_thread) }
}

// ---------------------------------------------------------------------------
// Fixed-arity tuple constructors (call the variadic `enif_make_tuple`)
// ---------------------------------------------------------------------------

/// 1-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple1(env: *mut Env, e1: Term) -> Term {
    unsafe { (api().make_tuple)(env, 1, e1) }
}

/// 2-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple2(env: *mut Env, e1: Term, e2: Term) -> Term {
    unsafe { (api().make_tuple)(env, 2, e1, e2) }
}

/// 3-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple3(env: *mut Env, e1: Term, e2: Term, e3: Term) -> Term {
    unsafe { (api().make_tuple)(env, 3, e1, e2, e3) }
}

/// 4-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple4(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term) -> Term {
    unsafe { (api().make_tuple)(env, 4, e1, e2, e3, e4) }
}

/// 5-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple5(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term, e5: Term) -> Term {
    unsafe { (api().make_tuple)(env, 5, e1, e2, e3, e4, e5) }
}

/// 6-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple6(
    env: *mut Env,
    e1: Term,
    e2: Term,
    e3: Term,
    e4: Term,
    e5: Term,
    e6: Term,
) -> Term {
    unsafe { (api().make_tuple)(env, 6, e1, e2, e3, e4, e5, e6) }
}

/// 7-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple7(
    env: *mut Env,
    e1: Term,
    e2: Term,
    e3: Term,
    e4: Term,
    e5: Term,
    e6: Term,
    e7: Term,
) -> Term {
    unsafe { (api().make_tuple)(env, 7, e1, e2, e3, e4, e5, e6, e7) }
}

/// 8-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple8(
    env: *mut Env,
    e1: Term,
    e2: Term,
    e3: Term,
    e4: Term,
    e5: Term,
    e6: Term,
    e7: Term,
    e8: Term,
) -> Term {
    unsafe { (api().make_tuple)(env, 8, e1, e2, e3, e4, e5, e6, e7, e8) }
}

/// 9-tuple. NIF 0.1. Calls variadic `enif_make_tuple`.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_tuple9(
    env: *mut Env,
    e1: Term,
    e2: Term,
    e3: Term,
    e4: Term,
    e5: Term,
    e6: Term,
    e7: Term,
    e8: Term,
    e9: Term,
) -> Term {
    unsafe { (api().make_tuple)(env, 9, e1, e2, e3, e4, e5, e6, e7, e8, e9) }
}

// ---------------------------------------------------------------------------
// Fixed-arity list constructors (call the variadic `enif_make_list`)
// ---------------------------------------------------------------------------

/// 1-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list1(env: *mut Env, e1: Term) -> Term {
    unsafe { (api().make_list)(env, 1, e1) }
}

/// 2-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list2(env: *mut Env, e1: Term, e2: Term) -> Term {
    unsafe { (api().make_list)(env, 2, e1, e2) }
}

/// 3-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list3(env: *mut Env, e1: Term, e2: Term, e3: Term) -> Term {
    unsafe { (api().make_list)(env, 3, e1, e2, e3) }
}

/// 4-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list4(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term) -> Term {
    unsafe { (api().make_list)(env, 4, e1, e2, e3, e4) }
}

/// 5-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list5(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term, e5: Term) -> Term {
    unsafe { (api().make_list)(env, 5, e1, e2, e3, e4, e5) }
}

/// 6-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list6(
    env: *mut Env,
    e1: Term,
    e2: Term,
    e3: Term,
    e4: Term,
    e5: Term,
    e6: Term,
) -> Term {
    unsafe { (api().make_list)(env, 6, e1, e2, e3, e4, e5, e6) }
}

/// 7-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list7(
    env: *mut Env,
    e1: Term,
    e2: Term,
    e3: Term,
    e4: Term,
    e5: Term,
    e6: Term,
    e7: Term,
) -> Term {
    unsafe { (api().make_list)(env, 7, e1, e2, e3, e4, e5, e6, e7) }
}

/// 8-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list8(
    env: *mut Env,
    e1: Term,
    e2: Term,
    e3: Term,
    e4: Term,
    e5: Term,
    e6: Term,
    e7: Term,
    e8: Term,
) -> Term {
    unsafe { (api().make_list)(env, 8, e1, e2, e3, e4, e5, e6, e7, e8) }
}

/// 9-element list. NIF 0.1. Calls variadic `enif_make_list`.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn make_list9(
    env: *mut Env,
    e1: Term,
    e2: Term,
    e3: Term,
    e4: Term,
    e5: Term,
    e6: Term,
    e7: Term,
    e8: Term,
    e9: Term,
) -> Term {
    unsafe { (api().make_list)(env, 9, e1, e2, e3, e4, e5, e6, e7, e8, e9) }
}
