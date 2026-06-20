# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-06-20

### Fixed

- `Monitor` now matches the C `ErlNifMonitor`/`ErlDrvMonitor` layout on every
  pointer width. Its length tracks `sizeof(void*) * 4` (16 bytes on 32-bit, 32
  on 64-bit) rather than a hardcoded `[u8; 32]`, and the spurious
  `#[repr(C, align(8))]` override is dropped so the type has the bare
  `char`-array alignment of 1. On 32-bit the old type was too short, so
  `compare_monitors` and the resource `down` callback could read past the bytes
  the BEAM wrote. A `const` assert now pins the size and alignment to the C ABI
  so the layout cannot silently drift. The 64-bit layout is unchanged.

### Documentation

- Every `enif_*` wrapper in `api.rs` now documents its parameters and return
  value. Each gains an `### Arguments` list (covering every parameter, including
  `env`), a `### Returns` section (omitted for `()`-returning functions), and a
  `### Reference` section carrying the upstream link and the NIF/OTP versions.
- Added a crates.io version badge to the README.

## [0.1.0] - 2026-06-17

Initial public release — the entire surface below is new.

### Added

- Raw, 1:1, all-`unsafe` FFI bindings to the Erlang NIF API (`erl_nif`), with
  every C prefix (`enif_`, `ERL_NIF_`, `ErlNif`) dropped and the whole surface
  re-exported flat to the crate root.
- NIF version floor 2.15 (OTP 22), always compiled, with an additive feature
  ladder, each rung pulling in the one below: `nif_2_16` (OTP 24), `nif_2_17`
  (OTP 26), `nif_2_18` (OTP 29).
- Unix and Windows support, chosen at compile time: `dlsym` symbol resolution
  on Unix, the BEAM-supplied callback table on Windows.
- `nif_init!` macro generating the platform-correct entry point, so a NIF's
  source is identical on both targets.

[0.2.0]: https://github.com/cubelio/enif-ffi/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/cubelio/enif-ffi/releases/tag/0.1.0
