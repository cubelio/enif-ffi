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

#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

use crate::ffi::api;
use crate::types::*;

// ===========================================================================
// NIF 0.1 / 1.0 — core term, binary, integer, atom, list/tuple
// ===========================================================================

/// Returns the pointer to the private data that was set by load or upgrade.
///
/// NIF 1.0. Wraps [`enif_priv_data`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_priv_data).
#[inline]
pub unsafe fn priv_data(env: *mut Env) -> *mut c_void {
    unsafe { (api().priv_data)(env) }
}

/// Allocates memory of size bytes.
///
/// NIF 1.0. Wraps [`enif_alloc`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_alloc).
#[inline]
pub unsafe fn alloc(size: usize) -> *mut c_void {
    unsafe { (api().alloc)(size) }
}

/// Frees memory allocated by enif_alloc.
///
/// NIF 1.0. Wraps [`enif_free`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_free).
#[inline]
pub unsafe fn free(ptr: *mut c_void) {
    unsafe { (api().free)(ptr) }
}

/// Returns true if term is an atom.
///
/// NIF 0.1. Wraps [`enif_is_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_atom).
#[inline]
pub unsafe fn is_atom(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_atom)(env, term) }
}

/// Returns true if term is a binary.
///
/// NIF 0.1. Wraps [`enif_is_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_binary).
#[inline]
pub unsafe fn is_binary(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_binary)(env, term) }
}

/// Returns true if term is a reference.
///
/// NIF 0.1. Wraps [`enif_is_ref`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_ref).
#[inline]
pub unsafe fn is_ref(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_ref)(env, term) }
}

/// Initializes the structure pointed to by bin with information about binary term bin_term. The data pointed to by bin is transient and does not need to be released unless it has been later reallocated with enif_realloc_binary.
///
/// NIF 0.1. Wraps [`enif_inspect_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_inspect_binary).
#[inline]
pub unsafe fn inspect_binary(env: *mut Env, term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().inspect_binary)(env, term, bin) }
}

/// Allocates a new binary of size size bytes. Initializes the structure pointed to by bin to refer to the allocated binary.
///
/// NIF 0.1. Wraps [`enif_alloc_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_alloc_binary).
#[inline]
pub unsafe fn alloc_binary(size: usize, bin: *mut Binary) -> c_int {
    unsafe { (api().alloc_binary)(size, bin) }
}

/// Changes the size of a binary bin. The source binary can be read-only, in which case it is left untouched and a mutable copy is allocated and assigned to *bin.
///
/// NIF 1.0. Wraps [`enif_realloc_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_realloc_binary).
#[inline]
pub unsafe fn realloc_binary(bin: *mut Binary, size: usize) -> c_int {
    unsafe { (api().realloc_binary)(bin, size) }
}

/// Releases a binary obtained from enif_alloc_binary.
///
/// NIF 2.0. Wraps [`enif_release_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_release_binary).
#[inline]
pub unsafe fn release_binary(bin: *mut Binary) {
    unsafe { (api().release_binary)(bin) }
}

/// Sets *ip to the integer value of term.
///
/// NIF 0.1. Wraps [`enif_get_int`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_int).
#[inline]
pub unsafe fn get_int(env: *mut Env, term: Term, ip: *mut c_int) -> c_int {
    unsafe { (api().get_int)(env, term, ip) }
}

/// Sets *ip to the unsigned long integer value of term.
///
/// NIF 0.1. Wraps [`enif_get_ulong`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_ulong).
#[inline]
pub unsafe fn get_ulong(env: *mut Env, term: Term, ip: *mut c_ulong) -> c_int {
    unsafe { (api().get_ulong)(env, term, ip) }
}

/// Sets *dp to the floating-point value of term.
///
/// NIF 0.1. Wraps [`enif_get_double`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_double).
#[inline]
pub unsafe fn get_double(env: *mut Env, term: Term, dp: *mut f64) -> c_int {
    unsafe { (api().get_double)(env, term, dp) }
}

/// Sets *head and *tail from list list.
///
/// NIF 0.1. Wraps [`enif_get_list_cell`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_list_cell).
#[inline]
pub unsafe fn get_list_cell(env: *mut Env, term: Term, head: *mut Term, tail: *mut Term) -> c_int {
    unsafe { (api().get_list_cell)(env, term, head, tail) }
}

/// If term is a tuple, this function sets *array to point to an array containing the elements of the tuple, and sets *arity to the number of elements. Notice that the array is read-only and (*array)[N-1] is the Nth element of the tuple.
///
/// NIF 0.1. Wraps [`enif_get_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_tuple).
#[inline]
pub unsafe fn get_tuple(
    env: *mut Env,
    tpl: Term,
    arity: *mut c_int,
    array: *mut *const Term,
) -> c_int {
    unsafe { (api().get_tuple)(env, tpl, arity, array) }
}

/// Returns true if the two terms are identical. Corresponds to the Erlang operators =:= and =/=.
///
/// NIF 0.1. Wraps [`enif_is_identical`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_identical).
#[inline]
pub unsafe fn is_identical(lhs: Term, rhs: Term) -> c_int {
    unsafe { (api().is_identical)(lhs, rhs) }
}

/// Returns an integer < 0 if lhs < rhs, 0 if lhs = rhs, and > 0 if lhs > rhs. Corresponds to the Erlang operators ==, /=, =<, <, >=, and > (but not =:= or =/=).
///
/// NIF 0.1. Wraps [`enif_compare`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_compare).
#[inline]
pub unsafe fn compare(lhs: Term, rhs: Term) -> c_int {
    unsafe { (api().compare)(lhs, rhs) }
}

/// Makes a binary term from bin. Any ownership of the binary data is transferred to the created term and bin is to be considered read-only for the rest of the NIF call and then as released.
///
/// NIF 0.1. Wraps [`enif_make_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_binary).
#[inline]
pub unsafe fn make_binary(env: *mut Env, bin: *mut Binary) -> Term {
    unsafe { (api().make_binary)(env, bin) }
}

/// Makes a badarg exception to be returned from a NIF, and associates it with environment env. Once a NIF or any function it calls invokes enif_make_badarg, the runtime ensures that a badarg exception is raised when the NIF returns, even if the NIF attempts to return a non-exception term instead.
///
/// NIF 0.1. Wraps [`enif_make_badarg`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_badarg).
#[inline]
pub unsafe fn make_badarg(env: *mut Env) -> Term {
    unsafe { (api().make_badarg)(env) }
}

/// Creates an integer term.
///
/// NIF 0.1. Wraps [`enif_make_int`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_int).
#[inline]
pub unsafe fn make_int(env: *mut Env, i: c_int) -> Term {
    unsafe { (api().make_int)(env, i) }
}

/// Creates an integer term from an unsigned long int.
///
/// NIF 0.1. Wraps [`enif_make_ulong`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_ulong).
#[inline]
pub unsafe fn make_ulong(env: *mut Env, i: c_ulong) -> Term {
    unsafe { (api().make_ulong)(env, i) }
}

/// Creates a floating-point term from a double. If argument double is not finite or is NaN, enif_make_double invokes enif_make_badarg.
///
/// NIF 0.1. Wraps [`enif_make_double`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_double).
#[inline]
pub unsafe fn make_double(env: *mut Env, d: f64) -> Term {
    unsafe { (api().make_double)(env, d) }
}

/// Creates an atom term from the NUL-terminated C-string name with ISO Latin-1 encoding. If the length of name exceeds the maximum length allowed for an atom (255 characters), enif_make_atom invokes enif_make_badarg.
///
/// NIF 0.1. Wraps [`enif_make_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_atom).
#[inline]
pub unsafe fn make_atom(env: *mut Env, name: *const c_char) -> Term {
    unsafe { (api().make_atom)(env, name) }
}

/// Tries to create the term of an already existing atom from the NUL-terminated C-string name with encoding.
///
/// NIF 0.1. Wraps [`enif_make_existing_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_existing_atom).
#[inline]
pub unsafe fn make_existing_atom(
    env: *mut Env,
    name: *const c_char,
    atom: *mut Term,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().make_existing_atom)(env, name, atom, encoding) }
}

/// Creates a list cell [head | tail].
///
/// NIF 0.1. Wraps [`enif_make_list_cell`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list_cell).
#[inline]
pub unsafe fn make_list_cell(env: *mut Env, car: Term, cdr: Term) -> Term {
    unsafe { (api().make_list_cell)(env, car, cdr) }
}

/// Creates a list containing the characters of the NUL-terminated string string with encoding.
///
/// NIF 0.1. Wraps [`enif_make_string`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_string).
#[inline]
pub unsafe fn make_string(env: *mut Env, string: *const c_char, encoding: CharEncoding) -> Term {
    unsafe { (api().make_string)(env, string, encoding) }
}

/// Creates a reference like erlang:make_ref/0.
///
/// NIF 0.1. Wraps [`enif_make_ref`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_ref).
#[inline]
pub unsafe fn make_ref(env: *mut Env) -> Term {
    unsafe { (api().make_ref)(env) }
}

// ===========================================================================
// NIF 1.0 — thread primitives
// ===========================================================================

/// Same as erl_drv_mutex_create.
///
/// NIF 1.0. Wraps [`enif_mutex_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_create).
#[inline]
pub unsafe fn mutex_create(name: *mut c_char) -> *mut Mutex {
    unsafe { (api().mutex_create)(name) }
}

/// Same as erl_drv_mutex_destroy.
///
/// NIF 1.0. Wraps [`enif_mutex_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_destroy).
#[inline]
pub unsafe fn mutex_destroy(mtx: *mut Mutex) {
    unsafe { (api().mutex_destroy)(mtx) }
}

/// Same as erl_drv_mutex_trylock.
///
/// NIF 1.0. Wraps [`enif_mutex_trylock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_trylock).
#[inline]
pub unsafe fn mutex_trylock(mtx: *mut Mutex) -> c_int {
    unsafe { (api().mutex_trylock)(mtx) }
}

/// Same as erl_drv_mutex_lock.
///
/// NIF 1.0. Wraps [`enif_mutex_lock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_lock).
#[inline]
pub unsafe fn mutex_lock(mtx: *mut Mutex) {
    unsafe { (api().mutex_lock)(mtx) }
}

/// Same as erl_drv_mutex_unlock.
///
/// NIF 1.0. Wraps [`enif_mutex_unlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_unlock).
#[inline]
pub unsafe fn mutex_unlock(mtx: *mut Mutex) {
    unsafe { (api().mutex_unlock)(mtx) }
}

/// Same as erl_drv_cond_create.
///
/// NIF 1.0. Wraps [`enif_cond_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_create).
#[inline]
pub unsafe fn cond_create(name: *mut c_char) -> *mut Cond {
    unsafe { (api().cond_create)(name) }
}

/// Same as erl_drv_cond_destroy.
///
/// NIF 1.0. Wraps [`enif_cond_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_destroy).
#[inline]
pub unsafe fn cond_destroy(cnd: *mut Cond) {
    unsafe { (api().cond_destroy)(cnd) }
}

/// Same as erl_drv_cond_signal.
///
/// NIF 1.0. Wraps [`enif_cond_signal`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_signal).
#[inline]
pub unsafe fn cond_signal(cnd: *mut Cond) {
    unsafe { (api().cond_signal)(cnd) }
}

/// Same as erl_drv_cond_broadcast.
///
/// NIF 1.0. Wraps [`enif_cond_broadcast`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_broadcast).
#[inline]
pub unsafe fn cond_broadcast(cnd: *mut Cond) {
    unsafe { (api().cond_broadcast)(cnd) }
}

/// Same as erl_drv_cond_wait.
///
/// NIF 1.0. Wraps [`enif_cond_wait`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_wait).
#[inline]
pub unsafe fn cond_wait(cnd: *mut Cond, mtx: *mut Mutex) {
    unsafe { (api().cond_wait)(cnd, mtx) }
}

/// Same as erl_drv_rwlock_create.
///
/// NIF 1.0. Wraps [`enif_rwlock_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_create).
#[inline]
pub unsafe fn rwlock_create(name: *mut c_char) -> *mut RWLock {
    unsafe { (api().rwlock_create)(name) }
}

/// Same as erl_drv_rwlock_destroy.
///
/// NIF 1.0. Wraps [`enif_rwlock_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_destroy).
#[inline]
pub unsafe fn rwlock_destroy(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_destroy)(rwlck) }
}

/// Same as erl_drv_rwlock_tryrlock.
///
/// NIF 1.0. Wraps [`enif_rwlock_tryrlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_tryrlock).
#[inline]
pub unsafe fn rwlock_tryrlock(rwlck: *mut RWLock) -> c_int {
    unsafe { (api().rwlock_tryrlock)(rwlck) }
}

/// Same as erl_drv_rwlock_rlock.
///
/// NIF 1.0. Wraps [`enif_rwlock_rlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_rlock).
#[inline]
pub unsafe fn rwlock_rlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rlock)(rwlck) }
}

/// Same as erl_drv_rwlock_runlock.
///
/// NIF 1.0. Wraps [`enif_rwlock_runlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_runlock).
#[inline]
pub unsafe fn rwlock_runlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_runlock)(rwlck) }
}

/// Same as erl_drv_rwlock_tryrwlock.
///
/// NIF 1.0. Wraps [`enif_rwlock_tryrwlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_tryrwlock).
#[inline]
pub unsafe fn rwlock_tryrwlock(rwlck: *mut RWLock) -> c_int {
    unsafe { (api().rwlock_tryrwlock)(rwlck) }
}

/// Same as erl_drv_rwlock_rwlock.
///
/// NIF 1.0. Wraps [`enif_rwlock_rwlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_rwlock).
#[inline]
pub unsafe fn rwlock_rwlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rwlock)(rwlck) }
}

/// Same as erl_drv_rwlock_rwunlock.
///
/// NIF 1.0. Wraps [`enif_rwlock_rwunlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_rwunlock).
#[inline]
pub unsafe fn rwlock_rwunlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rwunlock)(rwlck) }
}

/// Same as erl_drv_tsd_key_create.
///
/// NIF 1.0. Wraps [`enif_tsd_key_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_tsd_key_create).
#[inline]
pub unsafe fn tsd_key_create(name: *mut c_char, key: *mut TSDKey) -> c_int {
    unsafe { (api().tsd_key_create)(name, key) }
}

/// Same as erl_drv_tsd_key_destroy.
///
/// NIF 1.0. Wraps [`enif_tsd_key_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_tsd_key_destroy).
#[inline]
pub unsafe fn tsd_key_destroy(key: TSDKey) {
    unsafe { (api().tsd_key_destroy)(key) }
}

/// Same as erl_drv_tsd_set.
///
/// NIF 1.0. Wraps [`enif_tsd_set`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_tsd_set).
#[inline]
pub unsafe fn tsd_set(key: TSDKey, data: *mut c_void) {
    unsafe { (api().tsd_set)(key, data) }
}

/// Same as erl_drv_tsd_get.
///
/// NIF 1.0. Wraps [`enif_tsd_get`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_tsd_get).
#[inline]
pub unsafe fn tsd_get(key: TSDKey) -> *mut c_void {
    unsafe { (api().tsd_get)(key) }
}

/// Same as erl_drv_thread_opts_create.
///
/// NIF 1.0. Wraps [`enif_thread_opts_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_opts_create).
#[inline]
pub unsafe fn thread_opts_create(name: *mut c_char) -> *mut ThreadOpts {
    unsafe { (api().thread_opts_create)(name) }
}

/// Same as erl_drv_thread_opts_destroy.
///
/// NIF 1.0. Wraps [`enif_thread_opts_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_opts_destroy).
#[inline]
pub unsafe fn thread_opts_destroy(opts: *mut ThreadOpts) {
    unsafe { (api().thread_opts_destroy)(opts) }
}

/// Same as erl_drv_thread_create.
///
/// NIF 1.0. Wraps [`enif_thread_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_create).
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

/// Same as erl_drv_thread_self.
///
/// NIF 1.0. Wraps [`enif_thread_self`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_self).
#[inline]
pub unsafe fn thread_self() -> Tid {
    unsafe { (api().thread_self)() }
}

/// Same as erl_drv_equal_tids.
///
/// NIF 1.0. Wraps [`enif_equal_tids`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_equal_tids).
#[inline]
pub unsafe fn equal_tids(tid1: Tid, tid2: Tid) -> c_int {
    unsafe { (api().equal_tids)(tid1, tid2) }
}

/// Same as erl_drv_thread_exit.
///
/// NIF 1.0. Wraps [`enif_thread_exit`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_exit).
#[inline]
pub unsafe fn thread_exit(resp: *mut c_void) {
    unsafe { (api().thread_exit)(resp) }
}

/// Same as erl_drv_thread_join.
///
/// NIF 1.0. Wraps [`enif_thread_join`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_join).
#[inline]
pub unsafe fn thread_join(tid: Tid, respp: *mut *mut c_void) -> c_int {
    unsafe { (api().thread_join)(tid, respp) }
}

// ===========================================================================
// NIF 1.0 / 2.0 — more core, resources, strings, env, send
// ===========================================================================

/// Reallocates memory allocated by enif_alloc to size bytes.
///
/// NIF 1.0. Wraps [`enif_realloc`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_realloc).
#[inline]
pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    unsafe { (api().realloc)(ptr, size) }
}

/// Same as driver_system_info.
///
/// NIF 1.0. Wraps [`enif_system_info`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_system_info).
#[inline]
pub unsafe fn system_info(sip: *mut SysInfo, si_size: usize) {
    unsafe { (api().system_info)(sip, si_size) }
}

/// Initializes the structure pointed to by bin with a continuous buffer with the same byte content as iolist. As with inspect_binary, the data pointed to by bin is transient and does not need to be released unless it has been later reallocated with enif_realloc_binary.
///
/// NIF 1.0. Wraps [`enif_inspect_iolist_as_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_inspect_iolist_as_binary).
#[inline]
pub unsafe fn inspect_iolist_as_binary(env: *mut Env, term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().inspect_iolist_as_binary)(env, term, bin) }
}

/// Makes a subbinary of binary bin_term, starting at zero-based position pos with a length of size bytes. bin_term must be a binary or bitstring.
///
/// NIF 1.0. Wraps [`enif_make_sub_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_sub_binary).
#[inline]
pub unsafe fn make_sub_binary(env: *mut Env, bin_term: Term, pos: usize, size: usize) -> Term {
    unsafe { (api().make_sub_binary)(env, bin_term, pos, size) }
}

/// Writes a NUL-terminated string in the buffer pointed to by buf with size size, consisting of the characters in the string list. The characters are written using encoding.
///
/// NIF 1.0. Wraps [`enif_get_string`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_string).
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

/// Writes a NUL-terminated string in the buffer pointed to by buf of size size bytes, consisting of the string representation of the atom term with encoding.
///
/// NIF 1.0. Wraps [`enif_get_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_atom).
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

/// Returns true if term is a fun.
///
/// NIF 1.0. Wraps [`enif_is_fun`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_fun).
#[inline]
pub unsafe fn is_fun(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_fun)(env, term) }
}

/// Returns true if term is a pid.
///
/// NIF 1.0. Wraps [`enif_is_pid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_pid).
#[inline]
pub unsafe fn is_pid(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_pid)(env, term) }
}

/// Returns true if term is a port.
///
/// NIF 1.0. Wraps [`enif_is_port`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_port).
#[inline]
pub unsafe fn is_port(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_port)(env, term) }
}

/// Sets *ip to the unsigned integer value of term.
///
/// NIF 1.0. Wraps [`enif_get_uint`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_uint).
#[inline]
pub unsafe fn get_uint(env: *mut Env, term: Term, ip: *mut c_uint) -> c_int {
    unsafe { (api().get_uint)(env, term, ip) }
}

/// Sets *ip to the long integer value of term.
///
/// NIF 1.0. Wraps [`enif_get_long`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_long).
#[inline]
pub unsafe fn get_long(env: *mut Env, term: Term, ip: *mut c_long) -> c_int {
    unsafe { (api().get_long)(env, term, ip) }
}

/// Creates an integer term from an unsigned int.
///
/// NIF 1.0. Wraps [`enif_make_uint`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_uint).
#[inline]
pub unsafe fn make_uint(env: *mut Env, i: c_uint) -> Term {
    unsafe { (api().make_uint)(env, i) }
}

/// Creates an integer term from a long int.
///
/// NIF 1.0. Wraps [`enif_make_long`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_long).
#[inline]
pub unsafe fn make_long(env: *mut Env, i: c_long) -> Term {
    unsafe { (api().make_long)(env, i) }
}

/// Creates a tuple containing the elements of array arr of length cnt.
///
/// NIF 1.0. Wraps [`enif_make_tuple_from_array`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple_from_array).
#[inline]
pub unsafe fn make_tuple_from_array(env: *mut Env, arr: *const Term, cnt: c_uint) -> Term {
    unsafe { (api().make_tuple_from_array)(env, arr, cnt) }
}

/// Creates an ordinary list containing the elements of array arr of length cnt.
///
/// NIF 1.0. Wraps [`enif_make_list_from_array`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list_from_array).
#[inline]
pub unsafe fn make_list_from_array(env: *mut Env, arr: *const Term, cnt: c_uint) -> Term {
    unsafe { (api().make_list_from_array)(env, arr, cnt) }
}

/// Returns true if term is an empty list.
///
/// NIF 1.0. Wraps [`enif_is_empty_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_empty_list).
#[inline]
pub unsafe fn is_empty_list(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_empty_list)(env, term) }
}

/// Creates or takes over a resource type identified by the string name and gives it the destructor function pointed to by dtor.
///
/// NIF 1.0. Wraps [`enif_open_resource_type`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_open_resource_type).
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

/// Allocates a memory-managed resource object of type type and size size bytes.
///
/// NIF 1.0. Wraps [`enif_alloc_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_alloc_resource).
#[inline]
pub unsafe fn alloc_resource(ty: *mut ResourceType, size: usize) -> *mut c_void {
    unsafe { (api().alloc_resource)(ty, size) }
}

/// Removes a reference to resource object obj obtained from enif_alloc_resource. The resource object is destructed when the last reference is removed.
///
/// NIF 1.0. Wraps [`enif_release_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_release_resource).
#[inline]
pub unsafe fn release_resource(obj: *mut c_void) {
    unsafe { (api().release_resource)(obj) }
}

/// Creates an opaque handle to a memory-managed resource object obtained by enif_alloc_resource. No ownership transfer is done, as the resource object still needs to be released by enif_release_resource.
///
/// NIF 1.0. Wraps [`enif_make_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_resource).
#[inline]
pub unsafe fn make_resource(env: *mut Env, obj: *mut c_void) -> Term {
    unsafe { (api().make_resource)(env, obj) }
}

/// Sets *objp to point to the resource object referred to by term.
///
/// NIF 1.0. Wraps [`enif_get_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_resource).
#[inline]
pub unsafe fn get_resource(
    env: *mut Env,
    term: Term,
    ty: *mut ResourceType,
    objp: *mut *mut c_void,
) -> c_int {
    unsafe { (api().get_resource)(env, term, ty, objp) }
}

/// Gets the byte size of resource object obj obtained by enif_alloc_resource.
///
/// NIF 1.0. Wraps [`enif_sizeof_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_sizeof_resource).
#[inline]
pub unsafe fn sizeof_resource(obj: *mut c_void) -> usize {
    unsafe { (api().sizeof_resource)(obj) }
}

/// Allocates a binary of size size bytes and creates an owning term. The binary data is mutable until the calling NIF returns.
///
/// NIF 1.0. Wraps [`enif_make_new_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_new_binary).
#[inline]
pub unsafe fn make_new_binary(env: *mut Env, size: usize, termp: *mut Term) -> *mut u8 {
    unsafe { (api().make_new_binary)(env, size, termp) }
}

/// Returns true if term is a list.
///
/// NIF 2.0. Wraps [`enif_is_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_list).
#[inline]
pub unsafe fn is_list(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_list)(env, term) }
}

/// Returns true if term is a tuple.
///
/// NIF 2.0. Wraps [`enif_is_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_tuple).
#[inline]
pub unsafe fn is_tuple(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_tuple)(env, term) }
}

/// Sets *len to the length (number of bytes excluding terminating NUL byte) of the atom term with encoding.
///
/// NIF 2.0. Wraps [`enif_get_atom_length`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_atom_length).
#[inline]
pub unsafe fn get_atom_length(
    env: *mut Env,
    atom: Term,
    len: *mut c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_atom_length)(env, atom, len, encoding) }
}

/// Sets *len to the length of list term.
///
/// NIF 2.0. Wraps [`enif_get_list_length`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_list_length).
#[inline]
pub unsafe fn get_list_length(env: *mut Env, term: Term, len: *mut c_uint) -> c_int {
    unsafe { (api().get_list_length)(env, term, len) }
}

/// Creates an atom term from the string name with length len and ISO Latin-1 encoding. NUL bytes are treated as any other characters.
///
/// NIF 2.0. Wraps [`enif_make_atom_len`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_atom_len).
#[inline]
pub unsafe fn make_atom_len(env: *mut Env, name: *const c_char, len: usize) -> Term {
    unsafe { (api().make_atom_len)(env, name, len) }
}

/// Tries to create the term of an already existing atom from the string name with length len bytes and encoding. NUL bytes are treated as any other characters.
///
/// NIF 2.0. Wraps [`enif_make_existing_atom_len`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_existing_atom_len).
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

/// Creates a list containing the characters of the string string with length len and encoding. NUL bytes are treated as any other characters.
///
/// NIF 2.0. Wraps [`enif_make_string_len`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_string_len).
#[inline]
pub unsafe fn make_string_len(
    env: *mut Env,
    string: *const c_char,
    len: usize,
    encoding: CharEncoding,
) -> Term {
    unsafe { (api().make_string_len)(env, string, len, encoding) }
}

/// Allocates a new process independent environment. The environment can be used to hold terms that are not bound to any process.
///
/// NIF 2.0. Wraps [`enif_alloc_env`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_alloc_env).
#[inline]
pub unsafe fn alloc_env() -> *mut Env {
    unsafe { (api().alloc_env)() }
}

/// Frees an environment allocated with enif_alloc_env. All terms created in the environment are freed as well.
///
/// NIF 2.0. Wraps [`enif_free_env`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_free_env).
#[inline]
pub unsafe fn free_env(env: *mut Env) {
    unsafe { (api().free_env)(env) }
}

/// Frees all terms in an environment and clears it for reuse. The environment must have been allocated with enif_alloc_env.
///
/// NIF 2.0. Wraps [`enif_clear_env`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_clear_env).
#[inline]
pub unsafe fn clear_env(env: *mut Env) {
    unsafe { (api().clear_env)(env) }
}

/// Sends a message to a process.
///
/// NIF 2.0. Wraps [`enif_send`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_send).
#[inline]
pub unsafe fn send(env: *mut Env, to_pid: *const Pid, msg_env: *mut Env, msg: Term) -> c_int {
    unsafe { (api().send)(env, to_pid, msg_env, msg) }
}

/// Makes a copy of term src_term. The copy is created in environment dst_env.
///
/// NIF 2.0. Wraps [`enif_make_copy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_copy).
#[inline]
pub unsafe fn make_copy(dst_env: *mut Env, src_term: Term) -> Term {
    unsafe { (api().make_copy)(dst_env, src_term) }
}

/// Initializes the ErlNifPid variable at *pid to represent the calling process.
///
/// NIF 2.0. Wraps [`enif_self`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_self).
#[inline]
pub unsafe fn self_(caller_env: *mut Env, pid: *mut Pid) -> *mut Pid {
    unsafe { (api().self_)(caller_env, pid) }
}

/// If term is the pid of a node local process, this function initializes the pid variable *pid from it and returns true. Otherwise returns false.
///
/// NIF 2.0. Wraps [`enif_get_local_pid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_local_pid).
#[inline]
pub unsafe fn get_local_pid(env: *mut Env, term: Term, pid: *mut Pid) -> c_int {
    unsafe { (api().get_local_pid)(env, term, pid) }
}

/// Adds a reference to resource object obj obtained from enif_alloc_resource. Each call to enif_keep_resource for an object must be balanced by a call to enif_release_resource before the object is destructed.
///
/// NIF 2.0. Wraps [`enif_keep_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_keep_resource).
#[inline]
pub unsafe fn keep_resource(obj: *mut c_void) {
    unsafe { (api().keep_resource)(obj) }
}

/// Creates a binary term that is memory-managed by a resource object obj obtained by enif_alloc_resource. The returned binary term consists of size bytes pointed to by data.
///
/// NIF 2.0. Wraps [`enif_make_resource_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_resource_binary).
#[inline]
pub unsafe fn make_resource_binary(
    env: *mut Env,
    obj: *mut c_void,
    data: *const c_void,
    size: usize,
) -> Term {
    unsafe { (api().make_resource_binary)(env, obj, data, size) }
}

/// Sets *ip to the integer value of term.
///
/// NIF 2.0. Wraps [`enif_get_int64`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_int64).
#[inline]
pub unsafe fn get_int64(env: *mut Env, term: Term, ip: *mut i64) -> c_int {
    unsafe { (api().get_int64)(env, term, ip) }
}

/// Sets *ip to the unsigned integer value of term.
///
/// NIF 2.0. Wraps [`enif_get_uint64`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_uint64).
#[inline]
pub unsafe fn get_uint64(env: *mut Env, term: Term, ip: *mut u64) -> c_int {
    unsafe { (api().get_uint64)(env, term, ip) }
}

/// Creates an integer term from a signed 64-bit integer.
///
/// NIF 2.0. Wraps [`enif_make_int64`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_int64).
#[inline]
pub unsafe fn make_int64(env: *mut Env, i: i64) -> Term {
    unsafe { (api().make_int64)(env, i) }
}

/// Creates an integer term from an unsigned 64-bit integer.
///
/// NIF 2.0. Wraps [`enif_make_uint64`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_uint64).
#[inline]
pub unsafe fn make_uint64(env: *mut Env, i: u64) -> Term {
    unsafe { (api().make_uint64)(env, i) }
}

// ===========================================================================
// NIF 2.2 – 2.4
// ===========================================================================

/// Returns true if term is an exception.
///
/// NIF 2.2. Wraps [`enif_is_exception`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_exception).
#[inline]
pub unsafe fn is_exception(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_exception)(env, term) }
}

/// Sets *list_out to the reverse list of the list list_in and returns true, or returns false if list_in is not a list.
///
/// NIF 2.3. Wraps [`enif_make_reverse_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_reverse_list).
#[inline]
pub unsafe fn make_reverse_list(env: *mut Env, term: Term, list: *mut Term) -> c_int {
    unsafe { (api().make_reverse_list)(env, term, list) }
}

/// Returns true if term is a number.
///
/// NIF 2.3. Wraps [`enif_is_number`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_number).
#[inline]
pub unsafe fn is_number(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_number)(env, term) }
}

/// Opens a shared library (`dlopen`).
///
/// NIF 2.4. Wraps [`enif_dlopen`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_dlopen).
#[inline]
pub unsafe fn dlopen(
    lib: *const c_char,
    err_handler: Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
    err_arg: *mut c_void,
) -> *mut c_void {
    unsafe { (api().dlopen)(lib, err_handler, err_arg) }
}

/// Resolves a symbol (`dlsym`).
///
/// NIF 2.4. Wraps [`enif_dlsym`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_dlsym).
#[inline]
pub unsafe fn dlsym(
    handle: *mut c_void,
    symbol: *const c_char,
    err_handler: Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
    err_arg: *mut c_void,
) -> *mut c_void {
    unsafe { (api().dlsym)(handle, symbol, err_handler, err_arg) }
}

/// Gives the runtime system a hint about how much CPU time the current NIF call has consumed since the last hint, or since the start of the NIF if no previous hint has been specified. The time is specified as a percent of the timeslice that a process is allowed to execute Erlang code until it can be suspended to give time for other runnable processes.
///
/// NIF 2.4. Wraps [`enif_consume_timeslice`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_consume_timeslice).
#[inline]
pub unsafe fn consume_timeslice(env: *mut Env, percent: c_int) -> c_int {
    unsafe { (api().consume_timeslice)(env, percent) }
}

// ===========================================================================
// NIF 2.6 — maps
// ===========================================================================

/// Returns true if term is a map, otherwise false.
///
/// NIF 2.6. Wraps [`enif_is_map`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_map).
#[inline]
pub unsafe fn is_map(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_map)(env, term) }
}

/// Sets *size to the number of key-value pairs in the map term.
///
/// NIF 2.6. Wraps [`enif_get_map_size`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_map_size).
#[inline]
pub unsafe fn get_map_size(env: *mut Env, term: Term, size: *mut usize) -> c_int {
    unsafe { (api().get_map_size)(env, term, size) }
}

/// Makes an empty map term.
///
/// NIF 2.6. Wraps [`enif_make_new_map`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_new_map).
#[inline]
pub unsafe fn make_new_map(env: *mut Env) -> Term {
    unsafe { (api().make_new_map)(env) }
}

/// Makes a copy of map map_in and inserts key with value. If key already exists in map_in, the old associated value is replaced by value.
///
/// NIF 2.6. Wraps [`enif_make_map_put`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_map_put).
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

/// Sets *value to the value associated with key in the map map.
///
/// NIF 2.6. Wraps [`enif_get_map_value`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_map_value).
#[inline]
pub unsafe fn get_map_value(env: *mut Env, map: Term, key: Term, value: *mut Term) -> c_int {
    unsafe { (api().get_map_value)(env, map, key, value) }
}

/// Makes a copy of map map_in and replaces the old associated value for key with new_value.
///
/// NIF 2.6. Wraps [`enif_make_map_update`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_map_update).
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

/// If map map_in contains key, this function makes a copy of map_in in *map_out, and removes key and the associated value. If map map_in does not contain key, *map_out is set to map_in.
///
/// NIF 2.6. Wraps [`enif_make_map_remove`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_map_remove).
#[inline]
pub unsafe fn make_map_remove(env: *mut Env, map_in: Term, key: Term, map_out: *mut Term) -> c_int {
    unsafe { (api().make_map_remove)(env, map_in, key, map_out) }
}

/// Creates an iterator for the map map by initializing the structure pointed to by iter. Argument entry determines the start position of the iterator: ERL_NIF_MAP_ITERATOR_FIRST or ERL_NIF_MAP_ITERATOR_LAST.
///
/// NIF 2.6. Wraps [`enif_map_iterator_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_create).
#[inline]
pub unsafe fn map_iterator_create(
    env: *mut Env,
    map: Term,
    iter: *mut MapIterator,
    entry: MapIteratorEntry,
) -> c_int {
    unsafe { (api().map_iterator_create)(env, map, iter, entry) }
}

/// Destroys a map iterator created by enif_map_iterator_create.
///
/// NIF 2.6. Wraps [`enif_map_iterator_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_destroy).
#[inline]
pub unsafe fn map_iterator_destroy(env: *mut Env, iter: *mut MapIterator) {
    unsafe { (api().map_iterator_destroy)(env, iter) }
}

/// Returns true if map iterator iter is positioned before the first entry.
///
/// NIF 2.6. Wraps [`enif_map_iterator_is_head`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_is_head).
#[inline]
pub unsafe fn map_iterator_is_head(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_is_head)(env, iter) }
}

/// Returns true if map iterator iter is positioned after the last entry.
///
/// NIF 2.6. Wraps [`enif_map_iterator_is_tail`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_is_tail).
#[inline]
pub unsafe fn map_iterator_is_tail(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_is_tail)(env, iter) }
}

/// Increments map iterator to point to the next key-value entry.
///
/// NIF 2.6. Wraps [`enif_map_iterator_next`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_next).
#[inline]
pub unsafe fn map_iterator_next(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_next)(env, iter) }
}

/// Decrements map iterator to point to the previous key-value entry.
///
/// NIF 2.6. Wraps [`enif_map_iterator_prev`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_prev).
#[inline]
pub unsafe fn map_iterator_prev(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_prev)(env, iter) }
}

/// Gets key and value terms at the current map iterator position.
///
/// NIF 2.6. Wraps [`enif_map_iterator_get_pair`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_get_pair).
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

/// Schedules NIF fp to execute. This function allows an application to break up long-running work into multiple regular NIF calls or to schedule a dirty NIF to execute on a dirty scheduler thread.
///
/// NIF 2.7. Wraps [`enif_schedule_nif`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_schedule_nif).
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

/// Returns true if a pending exception is associated with the environment env. If reason is a NULL pointer, ignore it.
///
/// NIF 2.8. Wraps [`enif_has_pending_exception`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_has_pending_exception).
#[inline]
pub unsafe fn has_pending_exception(env: *mut Env, reason: *mut Term) -> c_int {
    unsafe { (api().has_pending_exception)(env, reason) }
}

/// Creates an error exception with the term reason to be returned from a NIF, and associates it with environment env. Once a NIF or any function it calls invokes enif_raise_exception, the runtime ensures that the exception it creates is raised when the NIF returns, even if the NIF attempts to return a non-exception term instead.
///
/// NIF 2.8. Wraps [`enif_raise_exception`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_raise_exception).
#[inline]
pub unsafe fn raise_exception(env: *mut Env, reason: Term) -> Term {
    unsafe { (api().raise_exception)(env, reason) }
}

/// Same as erl_drv_getenv.
///
/// NIF 2.9. Wraps [`enif_getenv`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_getenv).
#[inline]
pub unsafe fn getenv(key: *const c_char, value: *mut c_char, value_size: *mut usize) -> c_int {
    unsafe { (api().getenv)(key, value, value_size) }
}

/// Returns the current Erlang monotonic time. Notice that it is not uncommon with negative values.
///
/// NIF 2.10. Wraps [`enif_monotonic_time`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_monotonic_time).
#[inline]
pub unsafe fn monotonic_time(unit: TimeUnit) -> Time {
    unsafe { (api().monotonic_time)(unit) }
}

/// Returns the current time offset between Erlang monotonic time and Erlang system time converted into the time_unit passed as argument.
///
/// NIF 2.10. Wraps [`enif_time_offset`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_time_offset).
#[inline]
pub unsafe fn time_offset(unit: TimeUnit) -> Time {
    unsafe { (api().time_offset)(unit) }
}

/// Converts the val value of time unit from to the corresponding value of time unit to. The result is rounded using the floor function.
///
/// NIF 2.10. Wraps [`enif_convert_time_unit`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_convert_time_unit).
#[inline]
pub unsafe fn convert_time_unit(time: Time, from_unit: TimeUnit, to_unit: TimeUnit) -> Time {
    unsafe { (api().convert_time_unit)(time, from_unit, to_unit) }
}

/// Returns an erlang:now() time stamp.
///
/// NIF 2.11. Wraps [`enif_now_time`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_now_time).
#[inline]
pub unsafe fn now_time(env: *mut Env) -> Term {
    unsafe { (api().now_time)(env) }
}

/// Returns the CPU time in the same format as erlang:timestamp(). The CPU time is the time the current logical CPU has spent executing since some arbitrary point in the past.
///
/// NIF 2.11. Wraps [`enif_cpu_time`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cpu_time).
#[inline]
pub unsafe fn cpu_time(env: *mut Env) -> Term {
    unsafe { (api().cpu_time)(env) }
}

/// Returns a unique integer with the same properties as specified by erlang:unique_integer/1.
///
/// NIF 2.11. Wraps [`enif_make_unique_integer`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_unique_integer).
#[inline]
pub unsafe fn make_unique_integer(env: *mut Env, properties: UniqueInteger) -> Term {
    unsafe { (api().make_unique_integer)(env, properties) }
}

/// Returns true if the currently executing process is currently alive, otherwise false.
///
/// NIF 2.11. Wraps [`enif_is_current_process_alive`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_current_process_alive).
#[inline]
pub unsafe fn is_current_process_alive(env: *mut Env) -> c_int {
    unsafe { (api().is_current_process_alive)(env) }
}

/// Returns true if pid is alive.
///
/// NIF 2.11. Wraps [`enif_is_process_alive`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_process_alive).
#[inline]
pub unsafe fn is_process_alive(env: *mut Env, pid: *const Pid) -> c_int {
    unsafe { (api().is_process_alive)(env, pid) }
}

/// Returns true if port_id is alive.
///
/// NIF 2.11. Wraps [`enif_is_port_alive`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_port_alive).
#[inline]
pub unsafe fn is_port_alive(env: *mut Env, port_id: *const Port) -> c_int {
    unsafe { (api().is_port_alive)(env, port_id) }
}

/// If term identifies a node local port, this function initializes the port variable *port_id from it and returns true. Otherwise returns false.
///
/// NIF 2.11. Wraps [`enif_get_local_port`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_local_port).
#[inline]
pub unsafe fn get_local_port(env: *mut Env, term: Term, port_id: *mut Port) -> c_int {
    unsafe { (api().get_local_port)(env, term, port_id) }
}

/// Allocates a new binary with enif_alloc_binary and stores the result of encoding term according to the Erlang external term format.
///
/// NIF 2.11. Wraps [`enif_term_to_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_term_to_binary).
#[inline]
pub unsafe fn term_to_binary(env: *mut Env, term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().term_to_binary)(env, term, bin) }
}

/// Creates a term that is the result of decoding the binary data at data, which must be encoded according to the Erlang external term format. No more than size bytes are read from data.
///
/// NIF 2.11. Wraps [`enif_binary_to_term`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_binary_to_term).
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

/// Works as erlang:port_command/2, except that it is always completely asynchronous.
///
/// NIF 2.11. Wraps [`enif_port_command`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_port_command).
#[inline]
pub unsafe fn port_command(
    env: *mut Env,
    to_port: *const Port,
    msg_env: *mut Env,
    msg: Term,
) -> c_int {
    unsafe { (api().port_command)(env, to_port, msg_env, msg) }
}

/// Determines the type of the currently executing thread. A positive value indicates a scheduler thread while a negative value or zero indicates another type of thread.
///
/// NIF 2.11. Wraps [`enif_thread_type`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_type).
#[inline]
pub unsafe fn thread_type() -> c_int {
    unsafe { (api().thread_type)() }
}

// ===========================================================================
// NIF 2.12 — select, monitors, hash, whereis
// ===========================================================================

/// This function can be used to receive asynchronous notifications when OS-specific event objects become ready for either read or write operations.
///
/// NIF 2.12. Wraps [`enif_select`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select).
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

/// Same as enif_open_resource_type except it accepts additional callback functions for resource types that are used together with enif_select and enif_monitor_process.
///
/// NIF 2.12. Wraps [`enif_open_resource_type_x`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_open_resource_type_x).
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

/// Starts monitoring a process from a resource. When a process is monitored, a process exit results in a call to the provided down callback associated with the resource type.
///
/// NIF 2.12. Wraps [`enif_monitor_process`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_monitor_process).
#[inline]
pub unsafe fn monitor_process(
    env: *mut Env,
    obj: *mut c_void,
    pid: *const Pid,
    monitor: *mut Monitor,
) -> c_int {
    unsafe { (api().monitor_process)(env, obj, pid, monitor) }
}

/// Cancels a monitor created earlier with enif_monitor_process. Argument obj is a pointer to the resource holding the monitor and *mon identifies the monitor.
///
/// NIF 2.12. Wraps [`enif_demonitor_process`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_demonitor_process).
#[inline]
pub unsafe fn demonitor_process(env: *mut Env, obj: *mut c_void, monitor: *const Monitor) -> c_int {
    unsafe { (api().demonitor_process)(env, obj, monitor) }
}

/// Compares two ErlNifMonitors. Can also be used to imply some artificial order on monitors, for whatever reason.
///
/// NIF 2.12. Wraps [`enif_compare_monitors`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_compare_monitors).
#[inline]
pub unsafe fn compare_monitors(monitor1: *const Monitor, monitor2: *const Monitor) -> c_int {
    unsafe { (api().compare_monitors)(monitor1, monitor2) }
}

/// Hashes term according to the specified ErlNifHash type.
///
/// NIF 2.12. Wraps [`enif_hash`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_hash).
#[inline]
pub unsafe fn hash(hashtype: Hash, term: Term, salt: u64) -> u64 {
    unsafe { (api().hash)(hashtype, term, salt) }
}

/// Looks up a process by its registered name.
///
/// NIF 2.12. Wraps [`enif_whereis_pid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_whereis_pid).
#[inline]
pub unsafe fn whereis_pid(env: *mut Env, name: Term, pid: *mut Pid) -> c_int {
    unsafe { (api().whereis_pid)(env, name, pid) }
}

/// Looks up a port by its registered name.
///
/// NIF 2.12. Wraps [`enif_whereis_port`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_whereis_port).
#[inline]
pub unsafe fn whereis_port(env: *mut Env, name: Term, port: *mut Port) -> c_int {
    unsafe { (api().whereis_port)(env, name, port) }
}

// ===========================================================================
// NIF 2.13 — I/O queue
// ===========================================================================

/// Creates a new I/O queue that can be used to store data. opts has to be set to ERL_NIF_IOQ_NORMAL.
///
/// NIF 2.13. Wraps [`enif_ioq_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_create).
#[inline]
pub unsafe fn ioq_create(opts: IOQueueOpts) -> *mut IOQueue {
    unsafe { (api().ioq_create)(opts) }
}

/// Destroys the I/O queue and frees all of its contents.
///
/// NIF 2.13. Wraps [`enif_ioq_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_destroy).
#[inline]
pub unsafe fn ioq_destroy(q: *mut IOQueue) {
    unsafe { (api().ioq_destroy)(q) }
}

/// Enqueues the bin into q skipping the first skip bytes.
///
/// NIF 2.13. Wraps [`enif_ioq_enq_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_enq_binary).
#[inline]
pub unsafe fn ioq_enq_binary(q: *mut IOQueue, bin: *mut Binary, skip: usize) -> c_int {
    unsafe { (api().ioq_enq_binary)(q, bin, skip) }
}

/// Enqueues the iovec into q skipping the first skip bytes.
///
/// NIF 2.13. Wraps [`enif_ioq_enqv`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_enqv).
#[inline]
pub unsafe fn ioq_enqv(q: *mut IOQueue, iov: *mut IOVec, skip: usize) -> c_int {
    unsafe { (api().ioq_enqv)(q, iov, skip) }
}

/// Gets the size of q.
///
/// NIF 2.13. Wraps [`enif_ioq_size`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_size).
#[inline]
pub unsafe fn ioq_size(q: *mut IOQueue) -> usize {
    unsafe { (api().ioq_size)(q) }
}

/// Dequeues count bytes from the I/O queue. If size is not NULL, the new size of the queue is placed there.
///
/// NIF 2.13. Wraps [`enif_ioq_deq`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_deq).
#[inline]
pub unsafe fn ioq_deq(q: *mut IOQueue, count: usize, size: *mut usize) -> c_int {
    unsafe { (api().ioq_deq)(q, count, size) }
}

/// Gets the I/O queue as a pointer to an array of SysIOVecs. It also returns the number of elements in iovlen.
///
/// NIF 2.13. Wraps [`enif_ioq_peek`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_peek).
#[inline]
pub unsafe fn ioq_peek(q: *mut IOQueue, iovlen: *mut c_int) -> *mut SysIOVec {
    unsafe { (api().ioq_peek)(q, iovlen) }
}

/// Fills iovec with the list of binaries provided in iovec_term. The number of elements handled in the call is limited to max_elements, and tail is set to the remainder of the list.
///
/// NIF 2.13. Wraps [`enif_inspect_iovec`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_inspect_iovec).
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

/// Frees an io vector returned from enif_inspect_iovec. This is needed only if a NULL environment is passed to enif_inspect_iovec.
///
/// NIF 2.13. Wraps [`enif_free_iovec`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_free_iovec).
#[inline]
pub unsafe fn free_iovec(iov: *mut IOVec) {
    unsafe { (api().free_iovec)(iov) }
}

// ===========================================================================
// NIF 2.14 — ioq_peek_head, *_name, make_map_from_arrays
// ===========================================================================

/// Gets the head of the I/O queue as a binary term.
///
/// NIF 2.14. Wraps [`enif_ioq_peek_head`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_peek_head).
#[inline]
pub unsafe fn ioq_peek_head(
    env: *mut Env,
    q: *mut IOQueue,
    size: *mut usize,
    head: *mut Term,
) -> c_int {
    unsafe { (api().ioq_peek_head)(env, q, size, head) }
}

/// Same as erl_drv_mutex_name.
///
/// NIF 2.14. Wraps [`enif_mutex_name`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_name).
#[inline]
pub unsafe fn mutex_name(mtx: *mut Mutex) -> *mut c_char {
    unsafe { (api().mutex_name)(mtx) }
}

/// Same as erl_drv_cond_name.
///
/// NIF 2.14. Wraps [`enif_cond_name`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_name).
#[inline]
pub unsafe fn cond_name(cnd: *mut Cond) -> *mut c_char {
    unsafe { (api().cond_name)(cnd) }
}

/// Same as erl_drv_rwlock_name.
///
/// NIF 2.14. Wraps [`enif_rwlock_name`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_name).
#[inline]
pub unsafe fn rwlock_name(rwlck: *mut RWLock) -> *mut c_char {
    unsafe { (api().rwlock_name)(rwlck) }
}

/// Same as erl_drv_thread_name.
///
/// NIF 2.14. Wraps [`enif_thread_name`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_name).
#[inline]
pub unsafe fn thread_name(tid: Tid) -> *mut c_char {
    unsafe { (api().thread_name)(tid) }
}

/// Makes a map term from the given keys and values.
///
/// NIF 2.14. Wraps [`enif_make_map_from_arrays`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_map_from_arrays).
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

/// Generalized [`select`] with an explicit message and message env.
///
/// NIF 2.15. Wraps [`enif_select_x`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select_x).
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

/// Creates a term identifying the given monitor received from enif_monitor_process.
///
/// NIF 2.15. Wraps [`enif_make_monitor_term`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_monitor_term).
#[inline]
pub unsafe fn make_monitor_term(env: *mut Env, mon: *const Monitor) -> Term {
    unsafe { (api().make_monitor_term)(env, mon) }
}

/// Sets an ErlNifPid variable as undefined. See enif_is_pid_undefined.
///
/// NIF 2.15. Wraps [`enif_set_pid_undefined`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_set_pid_undefined).
#[inline]
pub unsafe fn set_pid_undefined(pid: *mut Pid) {
    unsafe { (api().set_pid_undefined)(pid) }
}

/// Returns true if pid has been set as undefined by enif_set_pid_undefined.
///
/// NIF 2.15. Wraps [`enif_is_pid_undefined`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_pid_undefined).
#[inline]
pub unsafe fn is_pid_undefined(pid: *const Pid) -> c_int {
    unsafe { (api().is_pid_undefined)(pid) }
}

/// Determines the type of the given term. The term must be an ordinary Erlang term and not one of the special terms returned by enif_raise_exception, enif_schedule_nif, or similar.
///
/// NIF 2.15. Wraps [`enif_term_type`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_term_type).
#[inline]
pub unsafe fn term_type(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().term_type)(env, term) }
}

// ===========================================================================
// NIF 2.16 (OTP 24)
// ===========================================================================

/// Same as enif_open_resource_type_x except it accepts an additional callback function for resource types that are used together with enif_dynamic_resource_call.
///
/// NIF 2.16. Wraps [`enif_init_resource_type`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_init_resource_type).
#[cfg(feature = "nif_2_16")]
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

/// Calls code of a resource type implemented by another NIF module. The atoms rt_module and rt_name identify the resource type to be called.
///
/// NIF 2.16. Wraps [`enif_dynamic_resource_call`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_dynamic_resource_call).
#[cfg(feature = "nif_2_16")]
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

/// Sets *len to the length (number of bytes excluding terminating NUL byte) of the string list with encoding.
///
/// NIF 2.17. Wraps [`enif_get_string_length`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_string_length).
#[cfg(feature = "nif_2_17")]
#[inline]
pub unsafe fn get_string_length(
    env: *mut Env,
    list: Term,
    len: *mut c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_string_length)(env, list, len, encoding) }
}

/// Creates an atom term from the NUL-terminated C-string name with encoding.
///
/// NIF 2.17. Wraps [`enif_make_new_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_new_atom).
#[cfg(feature = "nif_2_17")]
#[inline]
pub unsafe fn make_new_atom(
    env: *mut Env,
    name: *const c_char,
    atom: *mut Term,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().make_new_atom)(env, name, atom, encoding) }
}

/// Creates an atom term from string name with length len bytes and encoding.
///
/// NIF 2.17. Wraps [`enif_make_new_atom_len`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_new_atom_len).
#[cfg(feature = "nif_2_17")]
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

/// Gets the number of bytes used to store term. The size does not include ERL_NIF_TERM itself or binary data held by the term.
///
/// NIF 2.18. Wraps [`enif_term_size`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_term_size).
#[cfg(feature = "nif_2_18")]
#[inline]
pub unsafe fn term_size(term: Term) -> usize {
    unsafe { (api().term_size)(term) }
}

/// Gets the atom cache index of a term.
///
/// NIF 2.18. Wraps [`enif_get_atom_cache_index`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_atom_cache_index).
#[cfg(feature = "nif_2_18")]
#[inline]
pub unsafe fn get_atom_cache_index(env: *mut Env, atom: Term, index: *mut c_uint) -> c_int {
    unsafe { (api().get_atom_cache_index)(env, atom, index) }
}

/// Returns the maximum atom cache index.
///
/// NIF 2.18. Wraps [`enif_max_atom_cache_index`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_max_atom_cache_index).
#[cfg(feature = "nif_2_18")]
#[inline]
pub unsafe fn max_atom_cache_index() -> c_uint {
    unsafe { (api().max_atom_cache_index)() }
}

// ===========================================================================
// Convenience wrappers for C macros (no exported symbol)
// ===========================================================================

/// Makes a pid term or the atom undefined from *pid.
///
/// NIF 2.0. Wraps [`enif_make_pid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_pid).
#[inline]
pub unsafe fn make_pid(_env: *mut Env, pid: Pid) -> Term {
    pid.pid
}

/// Compares two pids by Erlang term order.
///
/// NIF 2.0. Wraps [`enif_compare_pids`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_compare_pids).
#[inline]
pub unsafe fn compare_pids(a: *const Pid, b: *const Pid) -> c_int {
    unsafe { compare((*a).pid, (*b).pid) }
}

/// Custom-message read select: calls [`select_x`] with `READ | CUSTOM_MSG`.
///
/// NIF 2.15. Wraps [`enif_select_read`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select_read).
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

/// Custom-message write select: calls [`select_x`] with `WRITE | CUSTOM_MSG`.
///
/// NIF 2.15. Wraps [`enif_select_write`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select_write).
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

/// Custom-message error select: calls [`select_x`] with `ERROR | CUSTOM_MSG`.
///
/// NIF 2.16. Wraps [`enif_select_error`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select_error).
#[cfg(feature = "nif_2_16")]
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

/// Sets the [`Option_::DelayHalt`] option: delay runtime-system halt until NIF calls return. Settable only during load.
///
/// NIF 2.17. Wraps [`enif_set_option`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_set_option).
#[cfg(feature = "nif_2_17")]
#[inline]
pub unsafe fn set_option_delay_halt(env: *mut Env) -> c_int {
    unsafe { (api().set_option)(env, Option_::DelayHalt) }
}

/// Installs an on-halt callback via the [`Option_::OnHalt`] option. Settable only during load.
///
/// NIF 2.17. Wraps [`enif_set_option`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_set_option).
#[cfg(feature = "nif_2_17")]
#[inline]
pub unsafe fn set_option_on_halt(
    env: *mut Env,
    on_halt: unsafe extern "C" fn(*mut c_void),
) -> c_int {
    unsafe { (api().set_option)(env, Option_::OnHalt, on_halt) }
}

/// Installs a per-scheduler on-unload-thread callback via the [`Option_::OnUnloadThread`] option. Settable only during load.
///
/// NIF 2.17 (OTP 27). Wraps [`enif_set_option`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_set_option).
#[cfg(feature = "nif_2_17")]
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

/// Creates a 1-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
#[inline]
pub unsafe fn make_tuple1(env: *mut Env, e1: Term) -> Term {
    unsafe { (api().make_tuple)(env, 1, e1) }
}

/// Creates a 2-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
#[inline]
pub unsafe fn make_tuple2(env: *mut Env, e1: Term, e2: Term) -> Term {
    unsafe { (api().make_tuple)(env, 2, e1, e2) }
}

/// Creates a 3-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
#[inline]
pub unsafe fn make_tuple3(env: *mut Env, e1: Term, e2: Term, e3: Term) -> Term {
    unsafe { (api().make_tuple)(env, 3, e1, e2, e3) }
}

/// Creates a 4-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
#[inline]
pub unsafe fn make_tuple4(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term) -> Term {
    unsafe { (api().make_tuple)(env, 4, e1, e2, e3, e4) }
}

/// Creates a 5-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
#[inline]
pub unsafe fn make_tuple5(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term, e5: Term) -> Term {
    unsafe { (api().make_tuple)(env, 5, e1, e2, e3, e4, e5) }
}

/// Creates a 6-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
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

/// Creates a 7-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
#[allow(clippy::too_many_arguments)]
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

/// Creates a 8-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
#[allow(clippy::too_many_arguments)]
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

/// Creates a 9-tuple.
///
/// NIF 0.1. Wraps [`enif_make_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple).
#[allow(clippy::too_many_arguments)]
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

/// Creates a 1-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
#[inline]
pub unsafe fn make_list1(env: *mut Env, e1: Term) -> Term {
    unsafe { (api().make_list)(env, 1, e1) }
}

/// Creates a 2-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
#[inline]
pub unsafe fn make_list2(env: *mut Env, e1: Term, e2: Term) -> Term {
    unsafe { (api().make_list)(env, 2, e1, e2) }
}

/// Creates a 3-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
#[inline]
pub unsafe fn make_list3(env: *mut Env, e1: Term, e2: Term, e3: Term) -> Term {
    unsafe { (api().make_list)(env, 3, e1, e2, e3) }
}

/// Creates a 4-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
#[inline]
pub unsafe fn make_list4(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term) -> Term {
    unsafe { (api().make_list)(env, 4, e1, e2, e3, e4) }
}

/// Creates a 5-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
#[inline]
pub unsafe fn make_list5(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term, e5: Term) -> Term {
    unsafe { (api().make_list)(env, 5, e1, e2, e3, e4, e5) }
}

/// Creates a 6-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
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

/// Creates a 7-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
#[allow(clippy::too_many_arguments)]
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

/// Creates a 8-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
#[allow(clippy::too_many_arguments)]
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

/// Creates a 9-element list.
///
/// NIF 0.1. Wraps [`enif_make_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list).
#[allow(clippy::too_many_arguments)]
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
