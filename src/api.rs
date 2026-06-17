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

/// The library's private data pointer.
///
/// Returns the pointer last stored through the `*priv_data` out-parameter of the
/// module's `load` or `upgrade` callback, or null if none was set. The same
/// pointer is shared by every NIF call into the library.
///
/// [`enif_priv_data`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_priv_data) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn priv_data(env: *mut Env) -> *mut c_void {
    unsafe { (api().priv_data)(env) }
}

/// Allocates a block of memory from the BEAM allocator.
///
/// Returns a pointer to at least `size` bytes, or null on failure. The block is
/// owned by the caller and is not garbage-collected; release it with [`free`].
///
/// [`enif_alloc`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_alloc) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn alloc(size: usize) -> *mut c_void {
    unsafe { (api().alloc)(size) }
}

/// Frees memory from the BEAM allocator.
///
/// Releases a block previously returned by [`alloc`] or [`realloc`]. Passing a
/// pointer not obtained from those, or freeing the same block twice, is
/// undefined behavior.
///
/// [`enif_free`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_free) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn free(ptr: *mut c_void) {
    unsafe { (api().free)(ptr) }
}

/// Tests whether a term is an atom.
///
/// Returns a non-zero value if `term` is an atom, `0` otherwise.
///
/// [`enif_is_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_atom) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn is_atom(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_atom)(env, term) }
}

/// Tests whether a term is a binary.
///
/// Returns a non-zero value if `term` is a binary, `0` otherwise.
///
/// [`enif_is_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_binary) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn is_binary(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_binary)(env, term) }
}

/// Tests whether a term is a reference.
///
/// Returns a non-zero value if `term` is a reference — as made by [`make_ref`]
/// or `erlang:make_ref/0` — and `0` otherwise.
///
/// [`enif_is_ref`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_ref) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn is_ref(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_ref)(env, term) }
}

/// Inspects the contents of a binary term.
///
/// On success fills `bin` with the byte count and a read-only data pointer for
/// `bin_term` and returns a non-zero value; returns `0` if `bin_term` is not a
/// binary. The data is valid for the rest of the NIF call and needs no release
/// unless it is later grown into an owned binary with [`realloc_binary`].
///
/// [`enif_inspect_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_inspect_binary) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn inspect_binary(env: *mut Env, bin_term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().inspect_binary)(env, bin_term, bin) }
}

/// Allocates a new, mutable binary.
///
/// Allocates `size` bytes and initializes `bin` to refer to them, returning a
/// non-zero value on success or `0` on failure. The binary is owned by the
/// caller until it is handed to a term with [`make_binary`] or discarded with
/// [`release_binary`].
///
/// [`enif_alloc_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_alloc_binary) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn alloc_binary(size: usize, bin: *mut Binary) -> c_int {
    unsafe { (api().alloc_binary)(size, bin) }
}

/// Resizes a mutable binary.
///
/// Changes `bin` to `size` bytes, preserving the existing data up to the smaller
/// of the two lengths. If `bin` currently refers to read-only data it is left
/// untouched and a fresh mutable copy is assigned in its place. Returns a
/// non-zero value on success or `0` on failure.
///
/// [`enif_realloc_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_realloc_binary) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn realloc_binary(bin: *mut Binary, size: usize) -> c_int {
    unsafe { (api().realloc_binary)(bin, size) }
}

/// Releases a mutable binary.
///
/// Frees a binary previously initialized by [`alloc_binary`] or
/// [`realloc_binary`] that has not been transferred to a term with
/// [`make_binary`]. Do not release a binary obtained from [`inspect_binary`].
///
/// [`enif_release_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_release_binary) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn release_binary(bin: *mut Binary) {
    unsafe { (api().release_binary)(bin) }
}

/// Decodes a fixed-width signed integer from a term.
///
/// If `term` is an integer that fits in a C `int`, writes it through `ip` and
/// returns a non-zero value; otherwise leaves `*ip` untouched and returns `0`
/// (a float, a non-integer, or a bignum that overflows the range).
///
/// [`enif_get_int`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_int) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn get_int(env: *mut Env, term: Term, ip: *mut c_int) -> c_int {
    unsafe { (api().get_int)(env, term, ip) }
}

/// Decodes an unsigned long integer from a term.
///
/// If `term` is a non-negative integer that fits in a C `unsigned long`, writes
/// it through `ip` and returns a non-zero value; otherwise leaves `*ip`
/// untouched and returns `0`.
///
/// [`enif_get_ulong`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_ulong) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn get_ulong(env: *mut Env, term: Term, ip: *mut c_ulong) -> c_int {
    unsafe { (api().get_ulong)(env, term, ip) }
}

/// Decodes a floating-point value from a term.
///
/// If `term` is a float, writes it through `dp` and returns a non-zero value;
/// otherwise leaves `*dp` untouched and returns `0`. Integer terms are not
/// accepted.
///
/// [`enif_get_double`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_double) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn get_double(env: *mut Env, term: Term, dp: *mut f64) -> c_int {
    unsafe { (api().get_double)(env, term, dp) }
}

/// Splits a non-empty list into its head and tail.
///
/// If `list` is a non-empty list, writes its first element through `head` and
/// the remainder through `tail` and returns a non-zero value; returns `0` for
/// the empty list or a non-list term.
///
/// [`enif_get_list_cell`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_list_cell) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn get_list_cell(env: *mut Env, list: Term, head: *mut Term, tail: *mut Term) -> c_int {
    unsafe { (api().get_list_cell)(env, list, head, tail) }
}

/// Borrows the elements of a tuple.
///
/// If `term` is a tuple, sets `array` to a read-only pointer to its elements and
/// `arity` to their count, returning a non-zero value; returns `0` otherwise.
/// Element N is `(*array)[N-1]`, and the array is valid for the rest of the NIF
/// call.
///
/// [`enif_get_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_tuple) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn get_tuple(
    env: *mut Env,
    term: Term,
    arity: *mut c_int,
    array: *mut *const Term,
) -> c_int {
    unsafe { (api().get_tuple)(env, term, arity, array) }
}

/// Tests two terms for exact equality.
///
/// Returns a non-zero value if `lhs` and `rhs` are identical, matching the
/// Erlang `=:=` operator — so the integer `1` and the float `1.0` are *not*
/// identical.
///
/// [`enif_is_identical`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_identical) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn is_identical(lhs: Term, rhs: Term) -> c_int {
    unsafe { (api().is_identical)(lhs, rhs) }
}

/// Orders two terms by Erlang term order.
///
/// Returns a negative, zero, or positive value according to whether `lhs` sorts
/// before, equal to, or after `rhs` under the standard term order — the
/// `<`/`=<`/`>`/`>=` and arithmetic `==` operators, not `=:=`.
///
/// [`enif_compare`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_compare) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn compare(lhs: Term, rhs: Term) -> c_int {
    unsafe { (api().compare)(lhs, rhs) }
}

/// Builds a binary term from a mutable binary.
///
/// Transfers ownership of `bin`'s data into a new binary term and returns it.
/// After the call `bin` is read-only for the rest of the NIF call and must not
/// be passed to [`release_binary`].
///
/// [`enif_make_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_binary) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn make_binary(env: *mut Env, bin: *mut Binary) -> Term {
    unsafe { (api().make_binary)(env, bin) }
}

/// Raises a `badarg` exception when the NIF returns.
///
/// Associates a pending `badarg` with `env`; once invoked, the runtime raises
/// `badarg` on return regardless of the term the NIF actually returns. The
/// returned term is a convenience for returning directly. For arbitrary
/// exception terms see [`raise_exception`].
///
/// [`enif_make_badarg`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_badarg) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn make_badarg(env: *mut Env) -> Term {
    unsafe { (api().make_badarg)(env) }
}

/// Builds an Erlang integer from a 32-bit signed value.
///
/// Constructs the integer term in `env`; always succeeds. For values outside the
/// `c_int` range use [`make_int64`] or [`make_uint64`].
///
/// [`enif_make_int`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_int) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn make_int(env: *mut Env, i: c_int) -> Term {
    unsafe { (api().make_int)(env, i) }
}

/// Builds an Erlang integer from an unsigned long value.
///
/// Constructs the integer term in `env`; always succeeds.
///
/// [`enif_make_ulong`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_ulong) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn make_ulong(env: *mut Env, i: c_ulong) -> Term {
    unsafe { (api().make_ulong)(env, i) }
}

/// Builds an Erlang float from a `double`.
///
/// Constructs the float term in `env`. If the value is infinite or NaN — which
/// Erlang floats cannot represent — a `badarg` is raised on return, as if via
/// [`make_badarg`].
///
/// [`enif_make_double`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_double) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_double(env: *mut Env, d: f64) -> Term {
    unsafe { (api().make_double)(env, d) }
}

/// Builds an atom from a Latin-1 C string.
///
/// Creates, or reuses, the atom named by the NUL-terminated `name`, interpreted
/// as ISO Latin-1. A name longer than the 255-character atom limit raises
/// `badarg` on return. For a length-counted name see [`make_atom_len`].
///
/// [`enif_make_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_atom) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn make_atom(env: *mut Env, name: *const c_char) -> Term {
    unsafe { (api().make_atom)(env, name) }
}

/// Looks up an already-existing atom by name.
///
/// If an atom named by the NUL-terminated `name`, in the given [`CharEncoding`],
/// already exists, writes it through `atom` and returns a non-zero value; returns
/// `0` without creating one otherwise. Useful to avoid growing the atom table
/// from untrusted input.
///
/// [`enif_make_existing_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_existing_atom) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_existing_atom(
    env: *mut Env,
    name: *const c_char,
    atom: *mut Term,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().make_existing_atom)(env, name, atom, encoding) }
}

/// Prepends an element to a list.
///
/// Builds the cons cell `[head | tail]` in `env` and returns it. `tail` need not
/// be a proper list.
///
/// [`enif_make_list_cell`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list_cell) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn make_list_cell(env: *mut Env, head: Term, tail: Term) -> Term {
    unsafe { (api().make_list_cell)(env, head, tail) }
}

/// Builds a string as a list of character codepoints.
///
/// Creates the Erlang string for the NUL-terminated `string`, in the given
/// [`CharEncoding`], and returns it as a list of integer codepoints.
///
/// [`enif_make_string`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_string) — NIF 0.1 — OTP R13B03
#[inline]
pub unsafe fn make_string(env: *mut Env, string: *const c_char, encoding: CharEncoding) -> Term {
    unsafe { (api().make_string)(env, string, encoding) }
}

/// Creates a fresh, unique reference.
///
/// Returns a new reference term bound to `env`, equivalent to
/// `erlang:make_ref/0`.
///
/// [`enif_make_ref`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_ref) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_ref(env: *mut Env) -> Term {
    unsafe { (api().make_ref)(env) }
}

// ===========================================================================
// NIF 1.0 — thread primitives
// ===========================================================================

/// Creates a mutex.
///
/// Returns a new mutex, or null on failure. `name` is an identifying string used
/// in lock checking and debugging. Behaves like the driver's
/// [`erl_drv_mutex_create`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_mutex_create).
///
/// [`enif_mutex_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_create) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn mutex_create(name: *mut c_char) -> *mut Mutex {
    unsafe { (api().mutex_create)(name) }
}

/// Destroys a mutex.
///
/// Frees a mutex created by [`mutex_create`]; it must be unlocked. Like
/// [`erl_drv_mutex_destroy`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_mutex_destroy).
///
/// [`enif_mutex_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_destroy) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn mutex_destroy(mtx: *mut Mutex) {
    unsafe { (api().mutex_destroy)(mtx) }
}

/// Tries to lock a mutex without blocking.
///
/// Locks `mtx` and returns `0` if it is free; returns `EBUSY` without blocking if
/// it is already held. Like
/// [`erl_drv_mutex_trylock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_mutex_trylock).
///
/// [`enif_mutex_trylock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_trylock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn mutex_trylock(mtx: *mut Mutex) -> c_int {
    unsafe { (api().mutex_trylock)(mtx) }
}

/// Locks a mutex, blocking until it is free.
///
/// Re-locking a mutex already held by the calling thread is undefined behavior.
/// Like [`erl_drv_mutex_lock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_mutex_lock).
///
/// [`enif_mutex_lock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_lock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn mutex_lock(mtx: *mut Mutex) {
    unsafe { (api().mutex_lock)(mtx) }
}

/// Unlocks a mutex.
///
/// Releases `mtx`, which must be held by the calling thread. Like
/// [`erl_drv_mutex_unlock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_mutex_unlock).
///
/// [`enif_mutex_unlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_unlock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn mutex_unlock(mtx: *mut Mutex) {
    unsafe { (api().mutex_unlock)(mtx) }
}

/// Creates a condition variable.
///
/// Returns a new condition variable, or null on failure. `name` identifies it for
/// debugging. Like
/// [`erl_drv_cond_create`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_cond_create).
///
/// [`enif_cond_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_create) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn cond_create(name: *mut c_char) -> *mut Cond {
    unsafe { (api().cond_create)(name) }
}

/// Destroys a condition variable.
///
/// Frees a condition variable created by [`cond_create`]. Like
/// [`erl_drv_cond_destroy`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_cond_destroy).
///
/// [`enif_cond_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_destroy) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn cond_destroy(cnd: *mut Cond) {
    unsafe { (api().cond_destroy)(cnd) }
}

/// Wakes one thread waiting on a condition variable.
///
/// Like [`erl_drv_cond_signal`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_cond_signal).
///
/// [`enif_cond_signal`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_signal) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn cond_signal(cnd: *mut Cond) {
    unsafe { (api().cond_signal)(cnd) }
}

/// Wakes all threads waiting on a condition variable.
///
/// Like [`erl_drv_cond_broadcast`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_cond_broadcast).
///
/// [`enif_cond_broadcast`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_broadcast) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn cond_broadcast(cnd: *mut Cond) {
    unsafe { (api().cond_broadcast)(cnd) }
}

/// Waits on a condition variable.
///
/// Atomically releases `mtx` and blocks until `cnd` is signaled, then re-acquires
/// `mtx` before returning. Call it in a loop that re-checks the predicate, since
/// spurious wakeups are possible. Like
/// [`erl_drv_cond_wait`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_cond_wait).
///
/// [`enif_cond_wait`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_wait) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn cond_wait(cnd: *mut Cond, mtx: *mut Mutex) {
    unsafe { (api().cond_wait)(cnd, mtx) }
}

/// Creates a read/write lock.
///
/// Returns a new rwlock, or null on failure. `name` identifies it for debugging.
/// Like [`erl_drv_rwlock_create`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_rwlock_create).
///
/// [`enif_rwlock_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_create) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn rwlock_create(name: *mut c_char) -> *mut RWLock {
    unsafe { (api().rwlock_create)(name) }
}

/// Destroys a read/write lock.
///
/// Frees an rwlock created by [`rwlock_create`]; it must be unlocked. Like
/// [`erl_drv_rwlock_destroy`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_rwlock_destroy).
///
/// [`enif_rwlock_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_destroy) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn rwlock_destroy(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_destroy)(rwlck) }
}

/// Tries to take a read lock without blocking.
///
/// Acquires `rwlck` for reading and returns `0` if possible; otherwise returns
/// `EBUSY` without blocking. Like
/// [`erl_drv_rwlock_tryrlock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_rwlock_tryrlock).
///
/// [`enif_rwlock_tryrlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_tryrlock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn rwlock_tryrlock(rwlck: *mut RWLock) -> c_int {
    unsafe { (api().rwlock_tryrlock)(rwlck) }
}

/// Takes a read lock, blocking until available.
///
/// Several readers may hold the lock at once, but never alongside a writer. Like
/// [`erl_drv_rwlock_rlock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_rwlock_rlock).
///
/// [`enif_rwlock_rlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_rlock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn rwlock_rlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rlock)(rwlck) }
}

/// Releases a read lock.
///
/// Like [`erl_drv_rwlock_runlock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_rwlock_runlock).
///
/// [`enif_rwlock_runlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_runlock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn rwlock_runlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_runlock)(rwlck) }
}

/// Tries to take the write lock without blocking.
///
/// Acquires `rwlck` exclusively and returns `0` if possible; otherwise returns
/// `EBUSY` without blocking. Like
/// [`erl_drv_rwlock_tryrwlock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_rwlock_tryrwlock).
///
/// [`enif_rwlock_tryrwlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_tryrwlock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn rwlock_tryrwlock(rwlck: *mut RWLock) -> c_int {
    unsafe { (api().rwlock_tryrwlock)(rwlck) }
}

/// Takes the write lock, blocking until exclusive.
///
/// Excludes all other readers and writers for the duration. Like
/// [`erl_drv_rwlock_rwlock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_rwlock_rwlock).
///
/// [`enif_rwlock_rwlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_rwlock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn rwlock_rwlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rwlock)(rwlck) }
}

/// Releases the write lock.
///
/// Like [`erl_drv_rwlock_rwunlock`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_rwlock_rwunlock).
///
/// [`enif_rwlock_rwunlock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_rwunlock) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn rwlock_rwunlock(rwlck: *mut RWLock) {
    unsafe { (api().rwlock_rwunlock)(rwlck) }
}

/// Creates a thread-specific data key.
///
/// Allocates a key through which each thread can store its own pointer, writing
/// it through `key` and returning `0` on success. `name` identifies it for
/// debugging. Like
/// [`erl_drv_tsd_key_create`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_tsd_key_create).
///
/// [`enif_tsd_key_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_tsd_key_create) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn tsd_key_create(name: *mut c_char, key: *mut TSDKey) -> c_int {
    unsafe { (api().tsd_key_create)(name, key) }
}

/// Destroys a thread-specific data key.
///
/// Frees a key created by [`tsd_key_create`]; every thread must first have reset
/// its value for the key to null. Like
/// [`erl_drv_tsd_key_destroy`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_tsd_key_destroy).
///
/// [`enif_tsd_key_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_tsd_key_destroy) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn tsd_key_destroy(key: TSDKey) {
    unsafe { (api().tsd_key_destroy)(key) }
}

/// Stores the calling thread's value for a key.
///
/// Associates `data` with `key` for the current thread only. Like
/// [`erl_drv_tsd_set`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_tsd_set).
///
/// [`enif_tsd_set`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_tsd_set) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn tsd_set(key: TSDKey, data: *mut c_void) {
    unsafe { (api().tsd_set)(key, data) }
}

/// Reads the calling thread's value for a key.
///
/// Returns the pointer last stored for `key` by the current thread, or null if it
/// has set none. Like
/// [`erl_drv_tsd_get`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_tsd_get).
///
/// [`enif_tsd_get`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_tsd_get) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn tsd_get(key: TSDKey) -> *mut c_void {
    unsafe { (api().tsd_get)(key) }
}

/// Allocates a thread-options block.
///
/// Returns options to pass to [`thread_create`], or null on failure. `name`
/// identifies it for debugging. Like
/// [`erl_drv_thread_opts_create`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_thread_opts_create).
///
/// [`enif_thread_opts_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_opts_create) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn thread_opts_create(name: *mut c_char) -> *mut ThreadOpts {
    unsafe { (api().thread_opts_create)(name) }
}

/// Frees a thread-options block.
///
/// Releases options created by [`thread_opts_create`]. Like
/// [`erl_drv_thread_opts_destroy`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_thread_opts_destroy).
///
/// [`enif_thread_opts_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_opts_destroy) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn thread_opts_destroy(opts: *mut ThreadOpts) {
    unsafe { (api().thread_opts_destroy)(opts) }
}

/// Spawns a new BEAM-managed thread.
///
/// Starts a thread running `func(args)`, writing its id through `tid` and
/// returning `0` on success. `opts` may be null for defaults. Every created
/// thread must eventually be reaped with [`thread_join`]. Like
/// [`erl_drv_thread_create`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_thread_create).
///
/// [`enif_thread_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_create) — NIF 1.0 — OTP R13B04
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

/// Returns the calling thread's identifier.
///
/// Like [`erl_drv_thread_self`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_thread_self).
///
/// [`enif_thread_self`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_self) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn thread_self() -> Tid {
    unsafe { (api().thread_self)() }
}

/// Compares two thread identifiers.
///
/// Returns a non-zero value if `tid1` and `tid2` refer to the same thread. Like
/// [`erl_drv_equal_tids`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_equal_tids).
///
/// [`enif_equal_tids`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_equal_tids) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn equal_tids(tid1: Tid, tid2: Tid) -> c_int {
    unsafe { (api().equal_tids)(tid1, tid2) }
}

/// Terminates the calling thread.
///
/// Ends the current thread with result `resp`, which a joiner receives through
/// [`thread_join`]. Valid only on threads started by [`thread_create`]. Like
/// [`erl_drv_thread_exit`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_thread_exit).
///
/// [`enif_thread_exit`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_exit) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn thread_exit(resp: *mut c_void) {
    unsafe { (api().thread_exit)(resp) }
}

/// Waits for a thread to finish and collects its result.
///
/// Blocks until thread `tid` exits, writing its result through `respp` (when
/// non-null) and returning `0`. Like
/// [`erl_drv_thread_join`](https://www.erlang.org/doc/apps/erts/erl_driver.html#erl_drv_thread_join).
///
/// [`enif_thread_join`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_join) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn thread_join(tid: Tid, respp: *mut *mut c_void) -> c_int {
    unsafe { (api().thread_join)(tid, respp) }
}

// ===========================================================================
// NIF 1.0 / 2.0 — more core, resources, strings, env, send
// ===========================================================================

/// Resizes a block from the BEAM allocator.
///
/// Grows or shrinks the block at `ptr` (from [`alloc`]) to `size` bytes,
/// preserving contents up to the smaller size, and returns the possibly-moved
/// pointer or null on failure. On failure the original block is left valid.
///
/// [`enif_realloc`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_realloc) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    unsafe { (api().realloc)(ptr, size) }
}

/// Fills in a system-information struct.
///
/// Writes BEAM runtime details — ERTS and OTP versions, scheduler counts, SMP
/// and dirty-scheduler support — into the [`SysInfo`] at `sys_info_ptr`, which
/// must be `size` bytes. Mirrors the driver's
/// [`driver_system_info`](https://www.erlang.org/doc/apps/erts/erl_driver.html#driver_system_info).
///
/// [`enif_system_info`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_system_info) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn system_info(sys_info_ptr: *mut SysInfo, size: usize) {
    unsafe { (api().system_info)(sys_info_ptr, size) }
}

/// Flattens an iolist into a contiguous binary.
///
/// Initializes `bin` with a single contiguous buffer holding the bytes of the
/// iolist `term`, returning a non-zero value on success or `0` if `term` is not
/// an iolist. As with [`inspect_binary`], the data is transient and needs no
/// release unless later grown with [`realloc_binary`].
///
/// [`enif_inspect_iolist_as_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_inspect_iolist_as_binary) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn inspect_iolist_as_binary(env: *mut Env, term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().inspect_iolist_as_binary)(env, term, bin) }
}

/// Builds a sub-binary referencing part of another binary.
///
/// Returns a binary term covering `size` bytes of `bin_term` starting at the
/// zero-based byte offset `pos`. `bin_term` must be a binary or bitstring and
/// `pos + size` must lie within it. No bytes are copied.
///
/// [`enif_make_sub_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_sub_binary) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_sub_binary(env: *mut Env, bin_term: Term, pos: usize, size: usize) -> Term {
    unsafe { (api().make_sub_binary)(env, bin_term, pos, size) }
}

/// Copies an Erlang string into a C buffer.
///
/// Writes the characters of the string `list`, in the given [`CharEncoding`], as
/// a NUL-terminated string into `buf` (capacity `size`). Returns the number of
/// bytes written including the NUL; a negative value whose magnitude is the
/// buffer size if the string was truncated; or `0` if `list` is not a string.
///
/// [`enif_get_string`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_string) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn get_string(
    env: *mut Env,
    list: Term,
    buf: *mut c_char,
    size: c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_string)(env, list, buf, size, encoding) }
}

/// Copies an atom's name into a C buffer.
///
/// Writes the name of the atom `term`, in the given [`CharEncoding`], as a
/// NUL-terminated string into `buf` (capacity `size`). Returns the number of
/// bytes written including the NUL, or `0` if `term` is not an atom or the name
/// does not fit.
///
/// [`enif_get_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_atom) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn get_atom(
    env: *mut Env,
    term: Term,
    buf: *mut c_char,
    size: c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_atom)(env, term, buf, size, encoding) }
}

/// Tests whether a term is a fun.
///
/// Returns a non-zero value if `term` is a fun, `0` otherwise.
///
/// [`enif_is_fun`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_fun) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn is_fun(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_fun)(env, term) }
}

/// Tests whether a term is a pid.
///
/// Returns a non-zero value if `term` is a pid, `0` otherwise.
///
/// [`enif_is_pid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_pid) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn is_pid(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_pid)(env, term) }
}

/// Tests whether a term is a port.
///
/// Returns a non-zero value if `term` is a port, `0` otherwise.
///
/// [`enif_is_port`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_port) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn is_port(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_port)(env, term) }
}

/// Decodes an unsigned integer from a term.
///
/// If `term` is a non-negative integer that fits in a C `unsigned int`, writes it
/// through `ip` and returns a non-zero value; otherwise leaves `*ip` untouched
/// and returns `0`.
///
/// [`enif_get_uint`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_uint) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn get_uint(env: *mut Env, term: Term, ip: *mut c_uint) -> c_int {
    unsafe { (api().get_uint)(env, term, ip) }
}

/// Decodes a long integer from a term.
///
/// If `term` is an integer that fits in a C `long`, writes it through `ip` and
/// returns a non-zero value; otherwise leaves `*ip` untouched and returns `0`.
///
/// [`enif_get_long`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_long) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn get_long(env: *mut Env, term: Term, ip: *mut c_long) -> c_int {
    unsafe { (api().get_long)(env, term, ip) }
}

/// Builds an Erlang integer from an unsigned int.
///
/// Constructs the integer term in `env`; always succeeds.
///
/// [`enif_make_uint`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_uint) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_uint(env: *mut Env, i: c_uint) -> Term {
    unsafe { (api().make_uint)(env, i) }
}

/// Builds an Erlang integer from a long value.
///
/// Constructs the integer term in `env`; always succeeds.
///
/// [`enif_make_long`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_long) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_long(env: *mut Env, i: c_long) -> Term {
    unsafe { (api().make_long)(env, i) }
}

/// Builds a tuple from an array of terms.
///
/// Returns a tuple whose elements are the `cnt` terms at `arr`, in order.
///
/// [`enif_make_tuple_from_array`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple_from_array) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_tuple_from_array(env: *mut Env, arr: *const Term, cnt: c_uint) -> Term {
    unsafe { (api().make_tuple_from_array)(env, arr, cnt) }
}

/// Builds a proper list from an array of terms.
///
/// Returns the list whose elements are the `cnt` terms at `arr`, in order; an
/// empty array yields the empty list.
///
/// [`enif_make_list_from_array`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list_from_array) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_list_from_array(env: *mut Env, arr: *const Term, cnt: c_uint) -> Term {
    unsafe { (api().make_list_from_array)(env, arr, cnt) }
}

/// Tests whether a term is the empty list.
///
/// Returns a non-zero value if `term` is `[]`, `0` otherwise.
///
/// [`enif_is_empty_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_empty_list) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn is_empty_list(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_empty_list)(env, term) }
}

/// Registers a resource type for the calling module.
///
/// Creates — or during a code upgrade takes over — the resource type named
/// `name` with destructor `dtor`, returning an opaque [`ResourceType`] handle or
/// null on failure. `flags` selects [`ResourceFlags::CREATE`] and/or
/// [`ResourceFlags::TAKEOVER`], and the operation actually performed is written
/// through `tried`. `module_str` is reserved and must be null. Callable only
/// from the module's `load` or `upgrade` callback; for stop/down/dyncall
/// callbacks use [`open_resource_type_x`].
///
/// [`enif_open_resource_type`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_open_resource_type) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn open_resource_type(
    env: *mut Env,
    module_str: *const c_char,
    name: *const c_char,
    dtor: Option<unsafe extern "C" fn(*mut Env, *mut c_void)>,
    flags: ResourceFlags,
    tried: *mut ResourceFlags,
) -> *mut ResourceType {
    unsafe { (api().open_resource_type)(env, module_str, name, dtor, flags, tried) }
}

/// Allocates a reference-counted resource object.
///
/// Allocates `size` bytes of a resource of type `type_` (from
/// [`open_resource_type`]) and returns a pointer with reference count 1. Publish
/// it to Erlang with [`make_resource`] and drop your own reference with
/// [`release_resource`]; the destructor runs once the count reaches zero.
///
/// [`enif_alloc_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_alloc_resource) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn alloc_resource(type_: *mut ResourceType, size: usize) -> *mut c_void {
    unsafe { (api().alloc_resource)(type_, size) }
}

/// Drops one reference to a resource object.
///
/// Decrements the reference count of `obj` (from [`alloc_resource`]); when it
/// reaches zero the type's destructor runs and the memory is freed. Terms made
/// with [`make_resource`] hold their own references, so this need not free
/// immediately.
///
/// [`enif_release_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_release_resource) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn release_resource(obj: *mut c_void) {
    unsafe { (api().release_resource)(obj) }
}

/// Wraps a resource object in an opaque term.
///
/// Returns a term referring to `obj` (from [`alloc_resource`]). Ownership is not
/// transferred: you still hold your own reference and must [`release_resource`]
/// it. The term keeps `obj` alive independently of that reference.
///
/// [`enif_make_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_resource) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_resource(env: *mut Env, obj: *mut c_void) -> Term {
    unsafe { (api().make_resource)(env, obj) }
}

/// Unwraps a resource object from a term.
///
/// If `term` is a resource of type `type_`, writes its object pointer through
/// `objp` and returns a non-zero value; returns `0` otherwise. The pointer is
/// borrowed — valid while the term (or any other reference) keeps the object
/// alive — not a new reference.
///
/// [`enif_get_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_resource) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn get_resource(
    env: *mut Env,
    term: Term,
    type_: *mut ResourceType,
    objp: *mut *mut c_void,
) -> c_int {
    unsafe { (api().get_resource)(env, term, type_, objp) }
}

/// Returns the byte size of a resource object.
///
/// Reports the `size` that `obj` was allocated with by [`alloc_resource`].
///
/// [`enif_sizeof_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_sizeof_resource) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn sizeof_resource(obj: *mut c_void) -> usize {
    unsafe { (api().sizeof_resource)(obj) }
}

/// Allocates a binary term and returns a writable pointer to its bytes.
///
/// Creates a binary of `size` bytes, writes the owning term through `termp`, and
/// returns a pointer to its data. The bytes are mutable only until the calling
/// NIF returns. A convenience combining [`alloc_binary`] and [`make_binary`].
///
/// [`enif_make_new_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_new_binary) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn make_new_binary(env: *mut Env, size: usize, termp: *mut Term) -> *mut u8 {
    unsafe { (api().make_new_binary)(env, size, termp) }
}

/// Tests whether a term is a list.
///
/// Returns a non-zero value if `term` is a list — including the empty list — and
/// `0` otherwise.
///
/// [`enif_is_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_list) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn is_list(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_list)(env, term) }
}

/// Tests whether a term is a tuple.
///
/// Returns a non-zero value if `term` is a tuple, `0` otherwise.
///
/// [`enif_is_tuple`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_tuple) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn is_tuple(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_tuple)(env, term) }
}

/// Reads the byte length of an atom's name.
///
/// Writes through `len` the number of bytes in the atom `term`'s name in the
/// given [`CharEncoding`], excluding any terminating NUL, and returns a non-zero
/// value; returns `0` if `term` is not an atom.
///
/// [`enif_get_atom_length`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_atom_length) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn get_atom_length(
    env: *mut Env,
    term: Term,
    len: *mut c_uint,
    encoding: CharEncoding,
) -> c_int {
    unsafe { (api().get_atom_length)(env, term, len, encoding) }
}

/// Reads the length of a proper list.
///
/// If `term` is a proper list, writes its element count through `len` and returns
/// a non-zero value; returns `0` for an improper or non-list term.
///
/// [`enif_get_list_length`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_list_length) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn get_list_length(env: *mut Env, term: Term, len: *mut c_uint) -> c_int {
    unsafe { (api().get_list_length)(env, term, len) }
}

/// Builds an atom from a length-counted Latin-1 string.
///
/// Like [`make_atom`], but reads exactly `len` bytes of `name` as ISO Latin-1, so
/// embedded NUL bytes are ordinary characters. A name over the 255-character
/// atom limit raises `badarg` on return.
///
/// [`enif_make_atom_len`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_atom_len) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn make_atom_len(env: *mut Env, name: *const c_char, len: usize) -> Term {
    unsafe { (api().make_atom_len)(env, name, len) }
}

/// Looks up an existing atom by a length-counted name.
///
/// Like [`make_existing_atom`], but reads exactly `len` bytes of `name`, so
/// embedded NUL bytes are ordinary characters. Writes the atom through `atom` and
/// returns a non-zero value only if it already exists.
///
/// [`enif_make_existing_atom_len`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_existing_atom_len) — NIF 2.0 — OTP R14A
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

/// Builds a string from a length-counted C string.
///
/// Like [`make_string`], but reads exactly `len` bytes of `string`, so embedded
/// NUL bytes become ordinary characters in the resulting list.
///
/// [`enif_make_string_len`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_string_len) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn make_string_len(
    env: *mut Env,
    string: *const c_char,
    len: usize,
    encoding: CharEncoding,
) -> Term {
    unsafe { (api().make_string_len)(env, string, len, encoding) }
}

/// Allocates a process-independent environment.
///
/// Returns a fresh [`Env`] not tied to any process, for holding terms across NIF
/// calls — for example a message to send later. Free it with [`free_env`].
///
/// [`enif_alloc_env`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_alloc_env) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn alloc_env() -> *mut Env {
    unsafe { (api().alloc_env)() }
}

/// Frees a process-independent environment.
///
/// Releases an environment from [`alloc_env`] along with every term created in
/// it. Never call it on a NIF call environment.
///
/// [`enif_free_env`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_free_env) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn free_env(env: *mut Env) {
    unsafe { (api().free_env)(env) }
}

/// Clears a process-independent environment for reuse.
///
/// Frees all terms in an environment from [`alloc_env`] and resets it to empty,
/// avoiding a [`free_env`]/[`alloc_env`] cycle.
///
/// [`enif_clear_env`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_clear_env) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn clear_env(env: *mut Env) {
    unsafe { (api().clear_env)(env) }
}

/// Sends a message to a process.
///
/// Delivers `msg` to the process `to_pid`, returning a non-zero value if the
/// process was alive. `msg` must live in `msg_env`: pass an environment from
/// [`alloc_env`] (which the send consumes and clears), or null to send a copy of
/// a term rooted in `caller_env`. `caller_env` is the calling NIF's environment,
/// or null when sending from a thread that is not running a NIF.
///
/// [`enif_send`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_send) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn send(
    caller_env: *mut Env,
    to_pid: *const Pid,
    msg_env: *mut Env,
    msg: Term,
) -> c_int {
    unsafe { (api().send)(caller_env, to_pid, msg_env, msg) }
}

/// Deep-copies a term into another environment.
///
/// Returns a copy of `src_term` allocated in `dst_env`, so it can outlive
/// `src_term`'s original environment.
///
/// [`enif_make_copy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_copy) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn make_copy(dst_env: *mut Env, src_term: Term) -> Term {
    unsafe { (api().make_copy)(dst_env, src_term) }
}

/// Records the calling process as a pid.
///
/// Writes the calling process's pid through `pid` and returns that same pointer.
/// `caller_env` must be a NIF call environment.
///
/// [`enif_self`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_self) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn self_(caller_env: *mut Env, pid: *mut Pid) -> *mut Pid {
    unsafe { (api().self_)(caller_env, pid) }
}

/// Extracts a node-local pid from a term.
///
/// If `term` is the pid of a process on the local node, writes it through `pid`
/// and returns a non-zero value; returns `0` otherwise.
///
/// [`enif_get_local_pid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_local_pid) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn get_local_pid(env: *mut Env, term: Term, pid: *mut Pid) -> c_int {
    unsafe { (api().get_local_pid)(env, term, pid) }
}

/// Adds a reference to a resource object.
///
/// Increments the reference count of `obj` (from [`alloc_resource`]); balance
/// each call with [`release_resource`] before the object can be destructed.
///
/// [`enif_keep_resource`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_keep_resource) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn keep_resource(obj: *mut c_void) {
    unsafe { (api().keep_resource)(obj) }
}

/// Builds a binary term backed by a resource's memory.
///
/// Returns a binary of `size` bytes at `data` whose lifetime is managed by the
/// resource `obj` (from [`alloc_resource`]) rather than copied. The resource is
/// kept alive while the binary exists — a way to expose bytes owned by a resource
/// without copying them.
///
/// [`enif_make_resource_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_resource_binary) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn make_resource_binary(
    env: *mut Env,
    obj: *mut c_void,
    data: *const c_void,
    size: usize,
) -> Term {
    unsafe { (api().make_resource_binary)(env, obj, data, size) }
}

/// Decodes a 64-bit signed integer from a term.
///
/// If `term` is an integer that fits in a signed 64-bit value, writes it through
/// `ip` and returns a non-zero value; otherwise leaves `*ip` untouched and
/// returns `0`.
///
/// [`enif_get_int64`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_int64) — NIF 2.0 — OTP R14B
#[inline]
pub unsafe fn get_int64(env: *mut Env, term: Term, ip: *mut i64) -> c_int {
    unsafe { (api().get_int64)(env, term, ip) }
}

/// Decodes a 64-bit unsigned integer from a term.
///
/// If `term` is a non-negative integer that fits in an unsigned 64-bit value,
/// writes it through `ip` and returns a non-zero value; otherwise leaves `*ip`
/// untouched and returns `0`.
///
/// [`enif_get_uint64`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_uint64) — NIF 2.0 — OTP R14B
#[inline]
pub unsafe fn get_uint64(env: *mut Env, term: Term, ip: *mut u64) -> c_int {
    unsafe { (api().get_uint64)(env, term, ip) }
}

/// Builds an Erlang integer from a signed 64-bit value.
///
/// Constructs the integer term in `env`; always succeeds.
///
/// [`enif_make_int64`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_int64) — NIF 2.0 — OTP R14B
#[inline]
pub unsafe fn make_int64(env: *mut Env, i: i64) -> Term {
    unsafe { (api().make_int64)(env, i) }
}

/// Builds an Erlang integer from an unsigned 64-bit value.
///
/// Constructs the integer term in `env`; always succeeds.
///
/// [`enif_make_uint64`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_uint64) — NIF 2.0 — OTP R14B
#[inline]
pub unsafe fn make_uint64(env: *mut Env, i: u64) -> Term {
    unsafe { (api().make_uint64)(env, i) }
}

// ===========================================================================
// NIF 2.2 – 2.4
// ===========================================================================

/// Returns `true` if `term` is an exception.
///
/// [`enif_is_exception`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_exception) — NIF 2.2 — OTP R14B03
#[inline]
pub unsafe fn is_exception(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_exception)(env, term) }
}

/// Sets `*list_out` to the reverse list of the list `list_in` and returns `true`, or returns `false` if `list_in` is not a list.
///
/// [`enif_make_reverse_list`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_reverse_list) — NIF 2.3 — OTP R15A
#[inline]
pub unsafe fn make_reverse_list(env: *mut Env, list_in: Term, list_out: *mut Term) -> c_int {
    unsafe { (api().make_reverse_list)(env, list_in, list_out) }
}

/// Returns `true` if `term` is a number.
///
/// [`enif_is_number`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_number) — NIF 2.3 — OTP R15A
#[inline]
pub unsafe fn is_number(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_number)(env, term) }
}

/// Opens a shared library (`dlopen`).
///
/// [`enif_dlopen`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_dlopen) — NIF 2.4 — OTP R16B
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
/// [`enif_dlsym`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_dlsym) — NIF 2.4 — OTP R16B
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
/// [`enif_consume_timeslice`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_consume_timeslice) — NIF 2.4 — OTP R16B
#[inline]
pub unsafe fn consume_timeslice(env: *mut Env, percent: c_int) -> c_int {
    unsafe { (api().consume_timeslice)(env, percent) }
}

// ===========================================================================
// NIF 2.6 — maps
// ===========================================================================

/// Returns `true` if `term` is a map, otherwise `false`.
///
/// [`enif_is_map`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_map) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn is_map(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().is_map)(env, term) }
}

/// Sets `*size` to the number of key-value pairs in the map `term`.
///
/// [`enif_get_map_size`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_map_size) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn get_map_size(env: *mut Env, term: Term, size: *mut usize) -> c_int {
    unsafe { (api().get_map_size)(env, term, size) }
}

/// Makes an empty map term.
///
/// [`enif_make_new_map`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_new_map) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn make_new_map(env: *mut Env) -> Term {
    unsafe { (api().make_new_map)(env) }
}

/// Makes a copy of map `map_in` and inserts `key` with `value`. If `key` already exists in `map_in`, the old associated value is replaced by `value`.
///
/// [`enif_make_map_put`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_map_put) — NIF 2.6 — OTP 17
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

/// Sets `*value` to the value associated with `key` in the map `map`.
///
/// [`enif_get_map_value`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_map_value) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn get_map_value(env: *mut Env, map: Term, key: Term, value: *mut Term) -> c_int {
    unsafe { (api().get_map_value)(env, map, key, value) }
}

/// Makes a copy of map `map_in` and replaces the old associated value for `key` with `new_value`.
///
/// [`enif_make_map_update`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_map_update) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn make_map_update(
    env: *mut Env,
    map_in: Term,
    key: Term,
    new_value: Term,
    map_out: *mut Term,
) -> c_int {
    unsafe { (api().make_map_update)(env, map_in, key, new_value, map_out) }
}

/// If map `map_in` contains `key`, this function makes a copy of `map_in` in `*map_out`, and removes `key` and the associated value. If map `map_in` does not contain `key`, `*map_out` is set to `map_in`.
///
/// [`enif_make_map_remove`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_map_remove) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn make_map_remove(env: *mut Env, map_in: Term, key: Term, map_out: *mut Term) -> c_int {
    unsafe { (api().make_map_remove)(env, map_in, key, map_out) }
}

/// Creates an iterator for the map `map` by initializing the structure pointed to by `iter`. Argument `entry` determines the start position of the iterator: `ERL_NIF_MAP_ITERATOR_FIRST` or `ERL_NIF_MAP_ITERATOR_LAST`.
///
/// [`enif_map_iterator_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_create) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn map_iterator_create(
    env: *mut Env,
    map: Term,
    iter: *mut MapIterator,
    entry: MapIteratorEntry,
) -> c_int {
    unsafe { (api().map_iterator_create)(env, map, iter, entry) }
}

/// Destroys a map iterator created by `enif_map_iterator_create`.
///
/// [`enif_map_iterator_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_destroy) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn map_iterator_destroy(env: *mut Env, iter: *mut MapIterator) {
    unsafe { (api().map_iterator_destroy)(env, iter) }
}

/// Returns `true` if map iterator `iter` is positioned before the first entry.
///
/// [`enif_map_iterator_is_head`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_is_head) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn map_iterator_is_head(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_is_head)(env, iter) }
}

/// Returns `true` if map iterator `iter` is positioned after the last entry.
///
/// [`enif_map_iterator_is_tail`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_is_tail) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn map_iterator_is_tail(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_is_tail)(env, iter) }
}

/// Increments map iterator to point to the next key-value entry.
///
/// [`enif_map_iterator_next`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_next) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn map_iterator_next(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_next)(env, iter) }
}

/// Decrements map iterator to point to the previous key-value entry.
///
/// [`enif_map_iterator_prev`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_prev) — NIF 2.6 — OTP 17
#[inline]
pub unsafe fn map_iterator_prev(env: *mut Env, iter: *mut MapIterator) -> c_int {
    unsafe { (api().map_iterator_prev)(env, iter) }
}

/// Gets key and value terms at the current map iterator position.
///
/// [`enif_map_iterator_get_pair`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_map_iterator_get_pair) — NIF 2.6 — OTP 17
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

/// Schedules NIF `fp` to execute. This function allows an application to break up long-running work into multiple regular NIF calls or to schedule a dirty NIF to execute on a dirty scheduler thread.
///
/// [`enif_schedule_nif`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_schedule_nif) — NIF 2.7 — OTP 17.3
#[inline]
pub unsafe fn schedule_nif(
    caller_env: *mut Env,
    fun_name: *const c_char,
    flags: c_int,
    fp: unsafe extern "C" fn(*mut Env, c_int, *const Term) -> Term,
    argc: c_int,
    argv: *const Term,
) -> Term {
    unsafe { (api().schedule_nif)(caller_env, fun_name, flags, fp, argc, argv) }
}

/// Returns `true` if a pending exception is associated with the environment `env`. If `reason` is a `NULL` pointer, ignore it.
///
/// [`enif_has_pending_exception`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_has_pending_exception) — NIF 2.8 — OTP 18
#[inline]
pub unsafe fn has_pending_exception(env: *mut Env, reason: *mut Term) -> c_int {
    unsafe { (api().has_pending_exception)(env, reason) }
}

/// Creates an error exception with the term `reason` to be returned from a NIF, and associates it with environment `env`. Once a NIF or any function it calls invokes `enif_raise_exception`, the runtime ensures that the exception it creates is raised when the NIF returns, even if the NIF attempts to return a non-exception term instead.
///
/// [`enif_raise_exception`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_raise_exception) — NIF 2.8 — OTP 18
#[inline]
pub unsafe fn raise_exception(env: *mut Env, reason: Term) -> Term {
    unsafe { (api().raise_exception)(env, reason) }
}

/// Same as `erl_drv_getenv`.
///
/// [`enif_getenv`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_getenv) — NIF 2.9 — OTP 18.2
#[inline]
pub unsafe fn getenv(key: *const c_char, value: *mut c_char, value_size: *mut usize) -> c_int {
    unsafe { (api().getenv)(key, value, value_size) }
}

/// Returns the current Erlang monotonic time. Notice that it is not uncommon with negative values.
///
/// [`enif_monotonic_time`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_monotonic_time) — NIF 2.10 — OTP 18.3
#[inline]
pub unsafe fn monotonic_time(time_unit: TimeUnit) -> Time {
    unsafe { (api().monotonic_time)(time_unit) }
}

/// Returns the current time offset between Erlang monotonic time and Erlang system time converted into the `time_unit` passed as argument.
///
/// [`enif_time_offset`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_time_offset) — NIF 2.10 — OTP 18.3
#[inline]
pub unsafe fn time_offset(time_unit: TimeUnit) -> Time {
    unsafe { (api().time_offset)(time_unit) }
}

/// Converts the `val` value of time unit `from` to the corresponding value of time unit `to`. The result is rounded using the floor function.
///
/// [`enif_convert_time_unit`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_convert_time_unit) — NIF 2.10 — OTP 18.3
#[inline]
pub unsafe fn convert_time_unit(val: Time, from: TimeUnit, to: TimeUnit) -> Time {
    unsafe { (api().convert_time_unit)(val, from, to) }
}

/// Returns an `erlang:now()` time stamp.
///
/// [`enif_now_time`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_now_time) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn now_time(env: *mut Env) -> Term {
    unsafe { (api().now_time)(env) }
}

/// Returns the CPU time in the same format as `erlang:timestamp()`. The CPU time is the time the current logical CPU has spent executing since some arbitrary point in the past.
///
/// [`enif_cpu_time`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cpu_time) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn cpu_time(env: *mut Env) -> Term {
    unsafe { (api().cpu_time)(env) }
}

/// Returns a unique integer with the same properties as specified by `erlang:unique_integer/1`.
///
/// [`enif_make_unique_integer`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_unique_integer) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn make_unique_integer(env: *mut Env, properties: UniqueInteger) -> Term {
    unsafe { (api().make_unique_integer)(env, properties) }
}

/// Returns `true` if the currently executing process is currently alive, otherwise `false`.
///
/// [`enif_is_current_process_alive`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_current_process_alive) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn is_current_process_alive(env: *mut Env) -> c_int {
    unsafe { (api().is_current_process_alive)(env) }
}

/// Returns `true` if `pid` is alive.
///
/// [`enif_is_process_alive`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_process_alive) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn is_process_alive(env: *mut Env, pid: *const Pid) -> c_int {
    unsafe { (api().is_process_alive)(env, pid) }
}

/// Returns `true` if `port_id` is alive.
///
/// [`enif_is_port_alive`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_port_alive) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn is_port_alive(env: *mut Env, port_id: *const Port) -> c_int {
    unsafe { (api().is_port_alive)(env, port_id) }
}

/// If `term` identifies a node local port, this function initializes the port variable `*port_id` from it and returns `true`. Otherwise returns `false`.
///
/// [`enif_get_local_port`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_local_port) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn get_local_port(env: *mut Env, term: Term, port_id: *mut Port) -> c_int {
    unsafe { (api().get_local_port)(env, term, port_id) }
}

/// Allocates a new binary with `enif_alloc_binary` and stores the result of encoding `term` according to the Erlang external term format.
///
/// [`enif_term_to_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_term_to_binary) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn term_to_binary(env: *mut Env, term: Term, bin: *mut Binary) -> c_int {
    unsafe { (api().term_to_binary)(env, term, bin) }
}

/// Creates a term that is the result of decoding the binary data at `data`, which must be encoded according to the Erlang external term format. No more than `size` bytes are read from `data`.
///
/// [`enif_binary_to_term`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_binary_to_term) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn binary_to_term(
    env: *mut Env,
    data: *const u8,
    size: usize,
    term: *mut Term,
    opts: c_uint,
) -> usize {
    unsafe { (api().binary_to_term)(env, data, size, term, opts) }
}

/// Works as `erlang:port_command/2`, except that it is always completely asynchronous.
///
/// [`enif_port_command`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_port_command) — NIF 2.11 — OTP 19
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
/// [`enif_thread_type`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_type) — NIF 2.11 — OTP 19
#[inline]
pub unsafe fn thread_type() -> c_int {
    unsafe { (api().thread_type)() }
}

// ===========================================================================
// NIF 2.12 — select, monitors, hash, whereis
// ===========================================================================

/// This function can be used to receive asynchronous notifications when OS-specific event objects become ready for either read or write operations.
///
/// [`enif_select`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select) — NIF 2.12 — OTP 20
#[inline]
pub unsafe fn select(
    env: *mut Env,
    event: Event,
    mode: SelectFlags,
    obj: *mut c_void,
    pid: *const Pid,
    ref_: Term,
) -> c_int {
    unsafe { (api().select)(env, event, mode, obj, pid, ref_) }
}

/// Same as `enif_open_resource_type` except it accepts additional callback functions for resource types that are used together with `enif_select` and `enif_monitor_process`.
///
/// [`enif_open_resource_type_x`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_open_resource_type_x) — NIF 2.12 — OTP 20
#[inline]
pub unsafe fn open_resource_type_x(
    env: *mut Env,
    name: *const c_char,
    init: *const ResourceTypeInit,
    flags: ResourceFlags,
    tried: *mut ResourceFlags,
) -> *mut ResourceType {
    unsafe { (api().open_resource_type_x)(env, name, init, flags, tried) }
}

/// Starts monitoring a process from a resource. When a process is monitored, a process exit results in a call to the provided `down` callback associated with the resource type.
///
/// [`enif_monitor_process`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_monitor_process) — NIF 2.12 — OTP 20
#[inline]
pub unsafe fn monitor_process(
    caller_env: *mut Env,
    obj: *mut c_void,
    target_pid: *const Pid,
    mon: *mut Monitor,
) -> c_int {
    unsafe { (api().monitor_process)(caller_env, obj, target_pid, mon) }
}

/// Cancels a monitor created earlier with `enif_monitor_process`. Argument `obj` is a pointer to the resource holding the monitor and `*mon` identifies the monitor.
///
/// [`enif_demonitor_process`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_demonitor_process) — NIF 2.12 — OTP 20
#[inline]
pub unsafe fn demonitor_process(
    caller_env: *mut Env,
    obj: *mut c_void,
    mon: *const Monitor,
) -> c_int {
    unsafe { (api().demonitor_process)(caller_env, obj, mon) }
}

/// Compares two `ErlNifMonitor`s. Can also be used to imply some artificial order on monitors, for whatever reason.
///
/// [`enif_compare_monitors`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_compare_monitors) — NIF 2.12 — OTP 20
#[inline]
pub unsafe fn compare_monitors(monitor1: *const Monitor, monitor2: *const Monitor) -> c_int {
    unsafe { (api().compare_monitors)(monitor1, monitor2) }
}

/// Hashes `term` according to the specified `ErlNifHash` `type`.
///
/// [`enif_hash`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_hash) — NIF 2.12 — OTP 20
#[inline]
pub unsafe fn hash(type_: Hash, term: Term, salt: u64) -> u64 {
    unsafe { (api().hash)(type_, term, salt) }
}

/// Looks up a process by its registered name.
///
/// [`enif_whereis_pid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_whereis_pid) — NIF 2.12 — OTP 20
#[inline]
pub unsafe fn whereis_pid(caller_env: *mut Env, name: Term, pid: *mut Pid) -> c_int {
    unsafe { (api().whereis_pid)(caller_env, name, pid) }
}

/// Looks up a port by its registered name.
///
/// [`enif_whereis_port`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_whereis_port) — NIF 2.12 — OTP 20
#[inline]
pub unsafe fn whereis_port(caller_env: *mut Env, name: Term, port: *mut Port) -> c_int {
    unsafe { (api().whereis_port)(caller_env, name, port) }
}

// ===========================================================================
// NIF 2.13 — I/O queue
// ===========================================================================

/// Creates a new I/O queue that can be used to store data. `opts` has to be set to `ERL_NIF_IOQ_NORMAL`.
///
/// [`enif_ioq_create`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_create) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn ioq_create(opts: IOQueueOpts) -> *mut IOQueue {
    unsafe { (api().ioq_create)(opts) }
}

/// Destroys the I/O queue and frees all of its contents.
///
/// [`enif_ioq_destroy`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_destroy) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn ioq_destroy(q: *mut IOQueue) {
    unsafe { (api().ioq_destroy)(q) }
}

/// Enqueues the `bin` into `q` skipping the first `skip` bytes.
///
/// [`enif_ioq_enq_binary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_enq_binary) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn ioq_enq_binary(q: *mut IOQueue, bin: *mut Binary, skip: usize) -> c_int {
    unsafe { (api().ioq_enq_binary)(q, bin, skip) }
}

/// Enqueues the `iovec` into `q` skipping the first `skip` bytes.
///
/// [`enif_ioq_enqv`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_enqv) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn ioq_enqv(q: *mut IOQueue, iovec: *mut IOVec, skip: usize) -> c_int {
    unsafe { (api().ioq_enqv)(q, iovec, skip) }
}

/// Gets the size of `q`.
///
/// [`enif_ioq_size`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_size) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn ioq_size(q: *mut IOQueue) -> usize {
    unsafe { (api().ioq_size)(q) }
}

/// Dequeues `count` bytes from the I/O queue. If `size` is not `NULL`, the new size of the queue is placed there.
///
/// [`enif_ioq_deq`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_deq) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn ioq_deq(q: *mut IOQueue, count: usize, size: *mut usize) -> c_int {
    unsafe { (api().ioq_deq)(q, count, size) }
}

/// Gets the I/O queue as a pointer to an array of `SysIOVec`s. It also returns the number of elements in `iovlen`.
///
/// [`enif_ioq_peek`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_peek) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn ioq_peek(q: *mut IOQueue, iovlen: *mut c_int) -> *mut SysIOVec {
    unsafe { (api().ioq_peek)(q, iovlen) }
}

/// Fills `iovec` with the list of binaries provided in `iovec_term`. The number of elements handled in the call is limited to `max_elements`, and `tail` is set to the remainder of the list.
///
/// [`enif_inspect_iovec`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_inspect_iovec) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn inspect_iovec(
    env: *mut Env,
    max_elements: usize,
    iovec_term: Term,
    tail: *mut Term,
    iovec: *mut *mut IOVec,
) -> c_int {
    unsafe { (api().inspect_iovec)(env, max_elements, iovec_term, tail, iovec) }
}

/// Frees an io vector returned from `enif_inspect_iovec`. This is needed only if a `NULL` environment is passed to `enif_inspect_iovec`.
///
/// [`enif_free_iovec`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_free_iovec) — NIF 2.12 — OTP 20.1
#[inline]
pub unsafe fn free_iovec(iov: *mut IOVec) {
    unsafe { (api().free_iovec)(iov) }
}

// ===========================================================================
// NIF 2.14 — ioq_peek_head, *_name, make_map_from_arrays
// ===========================================================================

/// Gets the head of the I/O queue as a binary term.
///
/// [`enif_ioq_peek_head`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_ioq_peek_head) — NIF 2.14 — OTP 21
#[inline]
pub unsafe fn ioq_peek_head(
    env: *mut Env,
    q: *mut IOQueue,
    size: *mut usize,
    bin_term: *mut Term,
) -> c_int {
    unsafe { (api().ioq_peek_head)(env, q, size, bin_term) }
}

/// Same as `erl_drv_mutex_name`.
///
/// [`enif_mutex_name`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_mutex_name) — NIF 2.14 — OTP 21
#[inline]
pub unsafe fn mutex_name(mtx: *mut Mutex) -> *mut c_char {
    unsafe { (api().mutex_name)(mtx) }
}

/// Same as `erl_drv_cond_name`.
///
/// [`enif_cond_name`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_cond_name) — NIF 2.14 — OTP 21
#[inline]
pub unsafe fn cond_name(cnd: *mut Cond) -> *mut c_char {
    unsafe { (api().cond_name)(cnd) }
}

/// Same as `erl_drv_rwlock_name`.
///
/// [`enif_rwlock_name`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_rwlock_name) — NIF 2.14 — OTP 21
#[inline]
pub unsafe fn rwlock_name(rwlck: *mut RWLock) -> *mut c_char {
    unsafe { (api().rwlock_name)(rwlck) }
}

/// Same as `erl_drv_thread_name`.
///
/// [`enif_thread_name`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_thread_name) — NIF 2.14 — OTP 21
#[inline]
pub unsafe fn thread_name(tid: Tid) -> *mut c_char {
    unsafe { (api().thread_name)(tid) }
}

/// Makes a map term from the given keys and values.
///
/// [`enif_make_map_from_arrays`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_map_from_arrays) — NIF 2.14 — OTP 21
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
/// [`enif_select_x`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select_x) — NIF 2.15 — OTP 22
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

/// Creates a term identifying the given monitor received from `enif_monitor_process`.
///
/// [`enif_make_monitor_term`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_monitor_term) — NIF 2.15 — OTP 22
#[inline]
pub unsafe fn make_monitor_term(env: *mut Env, mon: *const Monitor) -> Term {
    unsafe { (api().make_monitor_term)(env, mon) }
}

/// Sets an `ErlNifPid` variable as undefined. See `enif_is_pid_undefined`.
///
/// [`enif_set_pid_undefined`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_set_pid_undefined) — NIF 2.15 — OTP 22
#[inline]
pub unsafe fn set_pid_undefined(pid: *mut Pid) {
    unsafe { (api().set_pid_undefined)(pid) }
}

/// Returns `true` if `pid` has been set as undefined by `enif_set_pid_undefined`.
///
/// [`enif_is_pid_undefined`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_is_pid_undefined) — NIF 2.15 — OTP 22
#[inline]
pub unsafe fn is_pid_undefined(pid: *const Pid) -> c_int {
    unsafe { (api().is_pid_undefined)(pid) }
}

/// Determines the type of the given term. The term must be an ordinary Erlang term and not one of the special terms returned by `enif_raise_exception`, `enif_schedule_nif`, or similar.
///
/// [`enif_term_type`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_term_type) — NIF 2.15 — OTP 22
#[inline]
pub unsafe fn term_type(env: *mut Env, term: Term) -> c_int {
    unsafe { (api().term_type)(env, term) }
}

// ===========================================================================
// NIF 2.16 (OTP 24)
// ===========================================================================

/// Same as `enif_open_resource_type_x` except it accepts an additional callback function for resource types that are used together with `enif_dynamic_resource_call`.
///
/// [`enif_init_resource_type`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_init_resource_type) — NIF 2.16 — OTP 24
#[cfg(feature = "nif_2_16")]
#[inline]
pub unsafe fn init_resource_type(
    env: *mut Env,
    name: *const c_char,
    init: *const ResourceTypeInit,
    flags: ResourceFlags,
    tried: *mut ResourceFlags,
) -> *mut ResourceType {
    unsafe { (api().init_resource_type)(env, name, init, flags, tried) }
}

/// Calls code of a resource type implemented by another NIF module. The atoms `rt_module` and `rt_name` identify the resource type to be called.
///
/// [`enif_dynamic_resource_call`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_dynamic_resource_call) — NIF 2.16 — OTP 24
#[cfg(feature = "nif_2_16")]
#[inline]
pub unsafe fn dynamic_resource_call(
    caller_env: *mut Env,
    rt_module: Term,
    rt_name: Term,
    resource: Term,
    call_data: *mut c_void,
) -> c_int {
    unsafe { (api().dynamic_resource_call)(caller_env, rt_module, rt_name, resource, call_data) }
}

// ===========================================================================
// NIF 2.17 (OTP 26)
// ===========================================================================

/// Sets `*len` to the length (number of bytes excluding terminating NUL byte) of the string `list` with encoding.
///
/// [`enif_get_string_length`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_string_length) — NIF 2.17 — OTP 26
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

/// Creates an atom term from the NUL-terminated C-string `name` with encoding.
///
/// [`enif_make_new_atom`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_new_atom) — NIF 2.17 — OTP 26
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

/// Creates an atom term from string `name` with length `len` bytes and encoding.
///
/// [`enif_make_new_atom_len`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_new_atom_len) — NIF 2.17 — OTP 26
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
/// [`enif_term_size`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_term_size) — NIF 2.18 — OTP 29
#[cfg(feature = "nif_2_18")]
#[inline]
pub unsafe fn term_size(term: Term) -> usize {
    unsafe { (api().term_size)(term) }
}

/// Gets the atom cache index of `atom`.
///
/// [`enif_get_atom_cache_index`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_get_atom_cache_index) — NIF 2.18 — OTP 29
#[cfg(feature = "nif_2_18")]
#[inline]
pub unsafe fn get_atom_cache_index(env: *mut Env, atom: Term, index: *mut c_uint) -> c_int {
    unsafe { (api().get_atom_cache_index)(env, atom, index) }
}

/// Returns the maximum atom cache index.
///
/// [`enif_max_atom_cache_index`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_max_atom_cache_index) — NIF 2.18 — OTP 29
#[cfg(feature = "nif_2_18")]
#[inline]
pub unsafe fn max_atom_cache_index() -> c_uint {
    unsafe { (api().max_atom_cache_index)() }
}

// ===========================================================================
// Convenience wrappers for C macros (no exported symbol)
// ===========================================================================

/// Makes a pid term or the atom `undefined` from `*pid`.
///
/// [`enif_make_pid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_pid) — NIF 2.0 — OTP R14A
#[inline]
pub unsafe fn make_pid(_env: *mut Env, pid: Pid) -> Term {
    pid.pid
}

/// Compares two pids by Erlang term order.
///
/// [`enif_compare_pids`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_compare_pids) — NIF 2.15 — OTP 22
#[inline]
pub unsafe fn compare_pids(pid1: *const Pid, pid2: *const Pid) -> c_int {
    unsafe { compare((*pid1).pid, (*pid2).pid) }
}

/// Custom-message read select: calls [`select_x`] with `READ | CUSTOM_MSG`.
///
/// [`enif_select_read`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select_read) — NIF 2.15 — OTP 22
#[inline]
pub unsafe fn select_read(
    env: *mut Env,
    event: Event,
    obj: *mut c_void,
    pid: *const Pid,
    msg: Term,
    msg_env: *mut Env,
) -> c_int {
    unsafe {
        select_x(
            env,
            event,
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
/// [`enif_select_write`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select_write) — NIF 2.15 — OTP 22
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
/// [`enif_select_error`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_select_error) — NIF 2.16 — OTP 24
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
/// [`enif_set_option_delay_halt`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_set_option_delay_halt) — NIF 2.17 — OTP 26
#[cfg(feature = "nif_2_17")]
#[inline]
pub unsafe fn set_option_delay_halt(env: *mut Env) -> c_int {
    unsafe { (api().set_option)(env, Option_::DelayHalt) }
}

/// Installs an on-halt callback via the [`Option_::OnHalt`] option. Settable only during load.
///
/// [`enif_set_option_on_halt`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_set_option_on_halt) — NIF 2.17 — OTP 26
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
/// [`enif_set_option_on_unload_thread`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_set_option_on_unload_thread) — NIF 2.17 — OTP 27
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
/// [`enif_make_tuple1`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple1) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_tuple1(env: *mut Env, e1: Term) -> Term {
    unsafe { (api().make_tuple)(env, 1, e1) }
}

/// Creates a 2-tuple.
///
/// [`enif_make_tuple2`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple2) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_tuple2(env: *mut Env, e1: Term, e2: Term) -> Term {
    unsafe { (api().make_tuple)(env, 2, e1, e2) }
}

/// Creates a 3-tuple.
///
/// [`enif_make_tuple3`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple3) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_tuple3(env: *mut Env, e1: Term, e2: Term, e3: Term) -> Term {
    unsafe { (api().make_tuple)(env, 3, e1, e2, e3) }
}

/// Creates a 4-tuple.
///
/// [`enif_make_tuple4`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple4) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_tuple4(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term) -> Term {
    unsafe { (api().make_tuple)(env, 4, e1, e2, e3, e4) }
}

/// Creates a 5-tuple.
///
/// [`enif_make_tuple5`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple5) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_tuple5(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term, e5: Term) -> Term {
    unsafe { (api().make_tuple)(env, 5, e1, e2, e3, e4, e5) }
}

/// Creates a 6-tuple.
///
/// [`enif_make_tuple6`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple6) — NIF 1.0 — OTP R13B04
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
/// [`enif_make_tuple7`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple7) — NIF 1.0 — OTP R13B04
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
/// [`enif_make_tuple8`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple8) — NIF 1.0 — OTP R13B04
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
/// [`enif_make_tuple9`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_tuple9) — NIF 1.0 — OTP R13B04
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
/// [`enif_make_list1`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list1) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_list1(env: *mut Env, e1: Term) -> Term {
    unsafe { (api().make_list)(env, 1, e1) }
}

/// Creates a 2-element list.
///
/// [`enif_make_list2`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list2) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_list2(env: *mut Env, e1: Term, e2: Term) -> Term {
    unsafe { (api().make_list)(env, 2, e1, e2) }
}

/// Creates a 3-element list.
///
/// [`enif_make_list3`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list3) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_list3(env: *mut Env, e1: Term, e2: Term, e3: Term) -> Term {
    unsafe { (api().make_list)(env, 3, e1, e2, e3) }
}

/// Creates a 4-element list.
///
/// [`enif_make_list4`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list4) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_list4(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term) -> Term {
    unsafe { (api().make_list)(env, 4, e1, e2, e3, e4) }
}

/// Creates a 5-element list.
///
/// [`enif_make_list5`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list5) — NIF 1.0 — OTP R13B04
#[inline]
pub unsafe fn make_list5(env: *mut Env, e1: Term, e2: Term, e3: Term, e4: Term, e5: Term) -> Term {
    unsafe { (api().make_list)(env, 5, e1, e2, e3, e4, e5) }
}

/// Creates a 6-element list.
///
/// [`enif_make_list6`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list6) — NIF 1.0 — OTP R13B04
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
/// [`enif_make_list7`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list7) — NIF 1.0 — OTP R13B04
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
/// [`enif_make_list8`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list8) — NIF 1.0 — OTP R13B04
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
/// [`enif_make_list9`](https://www.erlang.org/doc/apps/erts/erl_nif.html#enif_make_list9) — NIF 1.0 — OTP R13B04
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
