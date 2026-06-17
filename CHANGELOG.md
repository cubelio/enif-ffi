# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/cubelio/enif-ffi/compare/0.1.0...HEAD
[0.1.0]: https://github.com/cubelio/enif-ffi/releases/tag/0.1.0
