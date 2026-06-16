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
//! Unix only for now. Windows uses a different binding mechanism (a callback
//! struct passed at load) and is a separate, later effort.

// Every `unsafe fn` body must still mark its unsafe operations with an inner
// `unsafe` block (the default in edition 2024; required here on edition 2021).
#![deny(unsafe_op_in_unsafe_fn)]

mod api;
mod ffi;
mod types;

pub use api::*;
pub use types::*;

/// Define the NIF library's entry point.
///
/// Generates the `nif_init` symbol the BEAM calls at load — with the correct
/// signature for the target platform — resolves the `enif_*` table (`dlsym` on
/// Unix, the BEAM-supplied callback table on Windows), then calls `$builder`,
/// your platform-agnostic function that returns the library descriptor.
///
/// `$builder` must be a `fn() -> *const Entry`. It runs once during load, after
/// the table is resolved, so it (and any wrapper it calls) can use the `enif_*`
/// API.
///
/// ```ignore
/// enif_ffi::loader::nif_init!(build_entry);
///
/// fn build_entry() -> *const enif_ffi::Entry {
///     // build and leak a 'static ErlNifEntry; see smoke_test/ for a full one
/// }
/// ```
#[macro_export]
macro_rules! nif_init {
    ($builder:path) => {
        #[cfg(unix)]
        #[no_mangle]
        pub extern "C" fn nif_init() -> *const $crate::Entry {
            if unsafe { $crate::loader::init() }.is_err() {
                return ::core::ptr::null();
            }
            $builder()
        }

        #[cfg(windows)]
        #[no_mangle]
        pub extern "C" fn nif_init(
            callbacks: *const $crate::loader::TWinDynNifCallbacks,
        ) -> *const $crate::Entry {
            unsafe { $crate::loader::init_windows(callbacks) };
            $builder()
        }
    };
}

/// Loader machinery — getting the binding wired up at NIF load time.
///
/// These items are specific to this crate; they have no `enif_*` counterpart.
/// Everything else in `enif_ffi` mirrors the C NIF API directly, while this
/// module is the glue that resolves the `enif_*` symbol table at load and
/// defines the entry point.
pub mod loader {
    /// Resolve the `enif_*` symbol table via `dlsym` (Unix).
    #[cfg(unix)]
    pub use crate::ffi::init;
    /// Store the BEAM-supplied callback table (Windows).
    #[cfg(windows)]
    pub use crate::ffi::{init_windows, TWinDynNifCallbacks};
    /// Define the library entry point; also available at the crate root.
    pub use crate::nif_init;
}
