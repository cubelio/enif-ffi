# enif-ffi

Raw FFI bindings to the Erlang NIF API (`erl_nif`) — the unsafe floor for
writing Erlang NIFs in Rust.

`enif-ffi` is a thin, 1:1, all-`unsafe` binding to the `enif_*` C API that the
BEAM exposes to NIF libraries. It adds no abstractions of its own; it is the
layer a safe NIF library is built on, much as `rustler_sys` is to `rustler`.

It does **not** provide a safe interface over terms, environments, or
resources — that belongs in a higher layer. If you want to *write* a NIF, you
probably want such a layer; reach for this crate when you are building one, or
need the raw API directly.

## What it provides

- Every `enif_*` function as a thin `unsafe` wrapper.
- The full `#[repr(C)]` type and constant layer (`Env`, `Term`, `Binary`,
  `Pid`, `ResourceTypeInit`, `SelectFlags`, …).
- A single `init()` that resolves the `enif_*` symbol table at load time (the
  BEAM does not let a NIF library link against `enif_*` directly).

## Naming

Every C prefix is dropped, since the whole API lives under `enif_ffi::`:

- `ERL_NIF_TERM` → `Term`, `ErlNifEnv` → `Env`, `ErlNifBinary` → `Binary`
- `enif_make_atom` → `make_atom`, `enif_is_atom` → `is_atom`
- `ERL_NIF_SELECT_READ` → `SelectFlags::READ`

A name that would collide with a Rust keyword or a `std` prelude item takes a
trailing underscore (`Option_`, `self_`). Every wrapper is documented with its
NIF version and a link to the upstream `erl_nif` reference.

## Version support

The floor is **NIF 2.15 (OTP 22)**, always compiled. Newer API is opt-in via an
additive feature ladder, each rung pulling in the one below — so the enabled
set is always a contiguous prefix and every gated item carries exactly one
`cfg`:

| Feature      | OTP |
| ------------ | --- |
| *(floor)*    | 22  |
| `nif_2_16`   | 24  |
| `nif_2_17`   | 26  |
| `nif_2_18`   | 29  |

Enabling a rung means "I require at least this OTP". The symbols it adds are
resolved at load, and a BEAM older than the target fails the load rather than
misbehaving.

## Usage

There is no codegen here — you write the `nif_init` entry point and the load
callback yourself, and call `init()` from the load callback before any wrapper
is used. A complete, minimal NIF (plus an Erlang harness that loads and
exercises it) lives in [`smoke_test/`](smoke_test). The shape:

```rust
use enif_ffi::*;
use std::ffi::{c_int, c_void};

unsafe extern "C" fn nif_add(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    let args = unsafe { std::slice::from_raw_parts(argv, argc as usize) };
    let (mut a, mut b) = (0, 0);
    unsafe { get_int(env, args[0], &mut a) };
    unsafe { get_int(env, args[1], &mut b) };
    unsafe { make_int(env, a + b) }
}

unsafe extern "C" fn load(_env: *mut Env, _priv_data: *mut *mut c_void, _info: Term) -> c_int {
    match unsafe { enif_ffi::init() } {
        Ok(()) => 0,
        Err(_) => 1, // non-zero => fail the load; the VM stays up
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nif_init() -> *const Entry {
    // Build and return a 'static ErlNifEntry referencing your Func table
    // and the `load` callback above. See smoke_test/ for the full version.
    todo!()
}
```

## Platform

Unix only for now. Windows binds the API through a callback struct passed at
load rather than `dlsym`; it is a planned addition, and the function table is
already laid out in the canonical order that mechanism requires.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
