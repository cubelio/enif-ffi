//! `enif-ffi` — raw FFI bindings to the Erlang NIF API (`erl_nif`).
//!
//! A thin, 1:1, all-`unsafe` binding to the `enif_*` C API exported by the
//! BEAM. It is the foundation a safe NIF library is built on; it adds no
//! abstractions of its own.
//!
//! # Naming
//!
//! Every C prefix is dropped — `enif_` (functions), `ERL_NIF_` (the term type,
//! macros, constants), and `ErlNif` (everything else) — because the whole API
//! lives under the `enif_ffi::` namespace and the prefix would be pure
//! redundancy:
//!
//! - `ERL_NIF_TERM` → [`Term`], `ErlNifEnv` → [`Env`], `ErlNifBinary` → [`Binary`]
//! - `enif_make_atom` → `make_atom`, `enif_is_atom` → `is_atom`
//! - `ERL_NIF_SELECT_READ` → [`SelectFlags::READ`]
//!
//! A bare name that would collide with a Rust keyword or a `std` prelude item
//! takes a trailing underscore: `ErlNifOption` → [`Option_`], `enif_self` →
//! `self_`.
//!
//! The public surface is flat: everything is re-exported to the crate root.
//! Prefer namespaced use (`enif_ffi::Term`, `enif_ffi::make_atom`) over a glob
//! import.
//!
//! # Who this is for
//!
//! Most NIF authors want a *safe* library on top of this — writing NIFs by hand
//! against a raw, all-`unsafe` table is deliberate, low-level work. Reach for
//! `enif-ffi` directly when you are building that safe layer, or when you want
//! the bare `enif_*` surface with no abstraction in the way.
//!
//! # A minimal NIF
//!
//! A consumer touches three things: the C-ABI functions (the BEAM calling
//! convention), the [`Entry`] that registers them and carries the load-time
//! metadata, and [`nif_init!`] to define the entry point. The same source
//! compiles on Unix and Windows.
//!
//! ```no_run
//! use enif_ffi::*;
//! use std::ffi::c_int;
//!
//! // add(A, B) -> A + B  — decode two integers, return their sum.
//! unsafe extern "C" fn nif_add(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
//!     if argc != 2 {
//!         return unsafe { make_badarg(env) };
//!     }
//!     let args = unsafe { std::slice::from_raw_parts(argv, 2) };
//!     let (mut a, mut b): (c_int, c_int) = (0, 0);
//!     if unsafe { get_int(env, args[0], &mut a) } == 0
//!         || unsafe { get_int(env, args[1], &mut b) } == 0
//!     {
//!         return unsafe { make_badarg(env) };
//!     }
//!     unsafe { make_int(env, a.wrapping_add(b)) }
//! }
//!
//! // mk_tuple() -> {ok, 42}  — build a 2-tuple of an atom and an integer.
//! unsafe extern "C" fn nif_mk_tuple(env: *mut Env, _argc: c_int, _argv: *const Term) -> Term {
//!     let ok = unsafe { make_atom(env, c"ok".as_ptr()) };
//!     let n = unsafe { make_int(env, 42) };
//!     unsafe { make_tuple2(env, ok, n) }
//! }
//!
//! // Build the library descriptor: the function table plus the version and name
//! // metadata the BEAM reads at load. Both the table and the `Entry` must outlive
//! // the call, so they are leaked.
//! fn build_entry() -> *const Entry {
//!     let funcs = vec![
//!         Func { name: c"add".as_ptr(), arity: 2, fptr: nif_add, flags: 0 },
//!         Func { name: c"mk_tuple".as_ptr(), arity: 0, fptr: nif_mk_tuple, flags: 0 },
//!     ]
//!     .leak();
//!
//!     Box::leak(Box::new(Entry {
//!         major: MAJOR_VERSION,
//!         minor: MINOR_VERSION,
//!         name: c"example".as_ptr(),
//!         num_of_funcs: funcs.len() as c_int,
//!         funcs: funcs.as_mut_ptr(),
//!         load: None,
//!         reload: None,
//!         upgrade: None,
//!         unload: None,
//!         vm_variant: VM_VARIANT.as_ptr(),
//!         options: 1,
//!         sizeof_resource_type_init: std::mem::size_of::<ResourceTypeInit>(),
//!         min_erts: MIN_ERTS_VERSION.as_ptr(),
//!     }))
//! }
//!
//! // Generates the platform-correct `nif_init` symbol the BEAM calls at load.
//! enif_ffi::nif_init!(build_entry);
//! ```
//!
//! On the Erlang side, once the module has loaded the library:
//!
//! ```erlang
//! example:add(2, 3).    %=> 5
//! example:mk_tuple().   %=> {ok, 42}
//! ```
//!
//! `smoke_test/` in the repository is the full, CI-exercised version of this:
//! built and loaded into real BEAMs across every supported NIF version.
//!
//! # Version support
//!
//! The floor is **NIF 2.15 (OTP 22)** — always compiled. Newer API is opt-in
//! through an additive feature ladder, each rung pulling in the one below:
//!
//! - `nif_2_16` — OTP 24
//! - `nif_2_17` — OTP 26
//! - `nif_2_18` — OTP 29
//!
//! Because the rungs chain, the enabled set is always a contiguous prefix, so
//! each version-gated item carries exactly one `#[cfg]`. Enabling a rung means
//! "I require at least this OTP"; the symbols it adds are resolved at load time
//! and a BEAM older than the target will fail the load rather than misbehave.
//!
//! Item-level version annotations and the gating boundaries are derived from the
//! tagged `erl_nif` header history (the `otp-enif` snapshot repo).
//!
//! # Platform
//!
//! Unix and Windows are both supported; the binding mechanism is chosen at
//! compile time. On Unix the `enif_*` table is resolved with `dlsym` at load;
//! on Windows the BEAM passes a callback struct to `nif_init`, which is stored
//! instead. [`nif_init!`] generates the correctly-typed entry point for the
//! target, so a NIF's source is identical on both.
//!
//! # Verification
//!
//! Correctness rests on two checks rather than exhaustive per-symbol tests:
//! the `enif_*` table layout is audited against the tagged `erl_nif` header
//! history (every field version-gated and order-matched), and a smoke NIF is
//! built and loaded into real BEAMs across NIF 2.15–2.18 on both Unix and
//! Windows in CI, proving the binding resolves and dispatches the table on
//! every supported version.

// Every `unsafe fn` body must still mark its unsafe operations with an inner
// `unsafe` block (the default in edition 2024; required here on edition 2021).
#![deny(unsafe_op_in_unsafe_fn)]

mod api;
mod ffi;
mod types;

// Platform-specific definitions: the load-time symbol resolver and the
// divergent `SysIOVec`. Exactly one is compiled. These are `pub` only so the
// exported `nif_init!` macro can name their `init` (and `TWinDynNifCallbacks`)
// from a downstream crate; `#[doc(hidden)]` keeps that machinery — `init`
// included — out of the documented surface.
#[cfg(unix)]
#[doc(hidden)]
pub mod unix;
#[cfg(windows)]
#[doc(hidden)]
pub mod windows;

#[cfg(not(any(unix, windows)))]
compile_error!("enif-ffi supports only Unix and Windows targets");

pub use api::*;
pub use types::*;

/// Define the NIF library's entry point.
///
/// Generates the `nif_init` symbol the BEAM calls at load — with the correct
/// signature for the target platform — resolves the `enif_*` table (`dlsym` on
/// Unix, the BEAM-supplied callback table on Windows), then calls `$builder`,
/// your platform-agnostic function returning the library descriptor. The symbol
/// resolution is handled entirely inside the generated entry point; nothing else
/// in the crate needs to be called to wire the binding up.
///
/// `$builder` must be a `fn() -> *const `[`Entry`](crate::Entry). It runs once
/// during load, after the table is resolved, so it (and any wrapper it calls)
/// can use the `enif_*` API.
///
/// ```no_run
/// # use enif_ffi::*;
/// enif_ffi::nif_init!(build_entry);
///
/// fn build_entry() -> *const enif_ffi::Entry {
///     // build and register your functions, then leak a 'static `Entry`;
///     // see the crate-level example, or `smoke_test/` for a complete one
/// #   Box::leak(Box::new(Entry {
/// #       major: MAJOR_VERSION,
/// #       minor: MINOR_VERSION,
/// #       name: c"example".as_ptr(),
/// #       num_of_funcs: 0,
/// #       funcs: std::ptr::null_mut(),
/// #       load: None,
/// #       reload: None,
/// #       upgrade: None,
/// #       unload: None,
/// #       vm_variant: VM_VARIANT.as_ptr(),
/// #       options: 1,
/// #       sizeof_resource_type_init: std::mem::size_of::<ResourceTypeInit>(),
/// #       min_erts: MIN_ERTS_VERSION.as_ptr(),
/// #   }))
/// }
/// ```
#[macro_export]
macro_rules! nif_init {
    ($builder:path) => {
        #[cfg(unix)]
        #[no_mangle]
        pub extern "C" fn nif_init() -> *const $crate::Entry {
            if unsafe { $crate::unix::init() }.is_err() {
                return ::core::ptr::null();
            }
            $builder()
        }

        #[cfg(windows)]
        #[no_mangle]
        pub extern "C" fn nif_init(
            callbacks: *const $crate::windows::TWinDynNifCallbacks,
        ) -> *const $crate::Entry {
            unsafe { $crate::windows::init(callbacks) };
            $builder()
        }
    };
}
