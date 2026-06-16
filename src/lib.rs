//! `enif-ffi` — raw FFI bindings to the Erlang NIF API (`erl_nif`).
//!
//! A thin, 1:1, all-`unsafe` binding to the `enif_*` C API exported by the
//! BEAM. It is the foundation a safe NIF library is built on; it adds no
//! abstractions of its own.
//!
//! # Naming
//!
//! Every C prefix is dropped — `enif_`, `ERL_NIF_`, `ErlNif`, `Erl` — because
//! the whole API lives under the `enif_ffi::` namespace and the prefix would be
//! pure redundancy:
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

mod types;

pub use types::*;
