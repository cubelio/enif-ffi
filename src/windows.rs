//! Windows-specific items: the load-time callback-table handoff and the
//! platform `SysIOVec`.
//!
//! Windows NIFs cannot resolve `enif_*` via `dlsym`; the BEAM passes a callback
//! table to `nif_init`, which [`init`] stores.

use std::ffi::{c_char, c_void};

use crate::ffi::{Api, API};

/// `TWinDynNifCallbacks` — the callback table the BEAM passes to `nif_init` on
/// Windows, where the `enif_*` symbols are not resolvable via `dlsym`.
///
/// Its leading fields are the `enif_*` function pointers in canonical order —
/// exactly `Api` — followed by an `erts_alc_test` slot the NIF API never
/// uses. Only the leading `Api` is read.
#[repr(C)]
pub struct TWinDynNifCallbacks {
    api: Api,
    erts_alc_test: *mut c_void,
}

/// Store the BEAM-supplied callback table (Windows). Idempotent.
///
/// Call exactly once from the `nif_init` entry point, before any other function
/// in this crate (the `nif_init!` macro does this for you).
///
/// # Safety
///
/// `callbacks` must be the pointer the BEAM passed to `nif_init`, valid for the
/// duration of the call.
pub unsafe fn init(callbacks: *const TWinDynNifCallbacks) {
    if API.get().is_some() {
        return;
    }
    // The function pointers are the leading `api` field; copy them out. A
    // newer BEAM may have appended further fields (plus erts_alc_test); we read
    // only our prefix.
    let api = unsafe { std::ptr::read(std::ptr::addr_of!((*callbacks).api)) };
    let _ = API.set(api);
}

/// A single base/length I/O segment.
///
/// The element type of an [`IOVec`](crate::IOVec), as produced by
/// [`inspect_iovec`](crate::inspect_iovec). On Windows the fields are swapped and
/// `iov_len` is 32-bit so the struct can be cast directly to a `WSABUF`.
///
/// [`SysIOVec`](https://www.erlang.org/doc/apps/erts/erl_nif.html#SysIOVec) — NIF 2.12 — OTP 20.1
#[repr(C)]
pub struct SysIOVec {
    pub iov_len: std::ffi::c_ulong,
    pub iov_base: *mut c_char,
}
