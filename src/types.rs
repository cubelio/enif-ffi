//! Raw C ABI types and constants mirroring `erl_nif.h` and `erl_drv_nif.h`.
//!
//! Direct `#[repr(C)]` transcriptions — no logic, no safety wrappers. Prefixes
//! are dropped per the crate naming convention; names that would shadow a Rust
//! keyword or a `std` prelude item take a trailing underscore (`Option_`).
//!
//! Each item ends with a `[`ErlNif…`](…) — NIF x.y — OTP z` line naming the C
//! entity and the release that introduced it; struct fields and enum variants
//! added in a later release carry their own inline note. The crate floor is
//! NIF 2.15 (OTP 22); anything newer is gated behind its rung.

use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::marker::{PhantomData, PhantomPinned};
use std::ops::BitOr;

// `SysIOVec` is platform-divergent; its definition lives in the active platform
// module. Re-export it here so it sits with the rest of the type mirror and
// reaches the crate root through `lib.rs`'s `pub use types::*`.
#[cfg(unix)]
pub use crate::unix::SysIOVec;
#[cfg(windows)]
pub use crate::windows::SysIOVec;

// ---------------------------------------------------------------------------
// Library version
// ---------------------------------------------------------------------------

/// The major NIF version this build targets. Always 2.
///
/// [`ERL_NIF_MAJOR_VERSION`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_MAJOR_VERSION) — NIF 0.1 — OTP R13B03
pub const MAJOR_VERSION: c_int = 2;

/// The highest NIF minor this build targets, set from the enabled feature rung.
/// Reported to the BEAM in the library [`Entry`].
///
/// [`ERL_NIF_MINOR_VERSION`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_MINOR_VERSION) — NIF 0.1 — OTP R13B03
#[cfg(not(feature = "nif_2_16"))]
pub const MINOR_VERSION: c_int = 15;
#[cfg(all(feature = "nif_2_16", not(feature = "nif_2_17")))]
pub const MINOR_VERSION: c_int = 16;
#[cfg(all(feature = "nif_2_17", not(feature = "nif_2_18")))]
pub const MINOR_VERSION: c_int = 17;
#[cfg(feature = "nif_2_18")]
pub const MINOR_VERSION: c_int = 18;

/// The minimum ERTS the library declares it requires, tracking the enabled
/// feature rung. The BEAM refuses to load the library on an older runtime.
///
/// [`ERL_NIF_MIN_ERTS_VERSION`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_MIN_ERTS_VERSION) — NIF 2.14 — OTP 21
#[cfg(not(feature = "nif_2_16"))]
pub const MIN_ERTS_VERSION: &CStr = c"erts-10.4";
#[cfg(all(feature = "nif_2_16", not(feature = "nif_2_17")))]
pub const MIN_ERTS_VERSION: &CStr = c"erts-12.0";
#[cfg(feature = "nif_2_17")]
pub const MIN_ERTS_VERSION: &CStr = c"erts-14.0";

/// The VM variant the library is built for (`"beam.vanilla"`).
///
/// [`ERL_NIF_VM_VARIANT`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_VM_VARIANT) — NIF 2.1 — OTP R14B02
pub const VM_VARIANT: &CStr = c"beam.vanilla";

// ---------------------------------------------------------------------------
// Core term type
// ---------------------------------------------------------------------------

/// A tagged machine word representing any Erlang term, opaque to the NIF library.
///
/// [`ERL_NIF_TERM`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM) — NIF 0.1 — OTP R13B03
pub type Term = usize;

// ---------------------------------------------------------------------------
// Opaque environment
// ---------------------------------------------------------------------------

/// Per-call or process-independent NIF environment.
///
/// Always used as `*mut Env`. Never constructed directly.
///
/// [`ErlNifEnv`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifEnv) — NIF 0.1 — OTP R13B03
#[repr(C)]
pub struct Env {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

// ---------------------------------------------------------------------------
// Function and entry descriptors
// ---------------------------------------------------------------------------

/// Describes one NIF: Erlang name, arity, function pointer, flags.
///
/// `flags` is `0` for a regular NIF, or [`DIRTY_JOB_CPU_BOUND`] /
/// [`DIRTY_JOB_IO_BOUND`] (cast to `c_uint`) for a dirty NIF; the `flags` field
/// itself was added in NIF 2.7 (OTP 17.3).
///
/// [`ErlNifFunc`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifFunc) — NIF 0.1 — OTP R13B03
#[repr(C)]
pub struct Func {
    pub name: *const c_char,
    pub arity: c_uint,
    pub fptr: unsafe extern "C" fn(env: *mut Env, argc: c_int, argv: *const Term) -> Term,
    pub flags: c_uint,
}

/// The library descriptor returned by `nif_init()`, extended in later versions.
/// All tail fields through `min_erts` are always present; the BEAM reads only
/// what its version knows.
///
/// [`ErlNifEntry`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifEntry) — NIF 0.1 — OTP R13B03
#[repr(C)]
pub struct Entry {
    pub major: c_int,
    pub minor: c_int,
    pub name: *const c_char,
    pub num_of_funcs: c_int,
    pub funcs: *mut Func,
    pub load: Option<unsafe extern "C" fn(*mut Env, *mut *mut c_void, Term) -> c_int>,
    pub reload: Option<unsafe extern "C" fn(*mut Env, *mut *mut c_void, Term) -> c_int>,
    pub upgrade:
        Option<unsafe extern "C" fn(*mut Env, *mut *mut c_void, *mut *mut c_void, Term) -> c_int>,
    pub unload: Option<unsafe extern "C" fn(*mut Env, *mut c_void)>,
    /// NIF 2.1 (OTP R14B02).
    pub vm_variant: *const c_char,
    /// NIF 2.7 (OTP 17.3) — unused, set to 0 or 1.
    pub options: c_uint,
    /// NIF 2.12 (OTP 20.0) — must equal `size_of::<ResourceTypeInit>()`.
    pub sizeof_resource_type_init: usize,
    /// NIF 2.14 (OTP 21.0) — minimum ERTS version string.
    pub min_erts: *const c_char,
}

// ---------------------------------------------------------------------------
// Binary
// ---------------------------------------------------------------------------

/// An inspected binary: byte count and data pointer. The `ref_bin`/`_spare`
/// fields are internal to the BEAM.
///
/// [`ErlNifBinary`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifBinary) — NIF 0.1 — OTP R13B03
#[repr(C)]
pub struct Binary {
    pub size: usize,
    pub data: *mut u8,
    ref_bin: *mut c_void,
    _spare: [*mut c_void; 2],
}

// ---------------------------------------------------------------------------
// Pid and Port
// ---------------------------------------------------------------------------

/// A local process identifier.
///
/// [`ErlNifPid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifPid) — NIF 2.0 — OTP R14A
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pid {
    pub pid: Term,
}

/// A port identifier.
///
/// [`ErlNifPort`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifPort) — NIF 2.11 — OTP 19
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Port {
    pub port_id: Term,
}

// ---------------------------------------------------------------------------
// Monitor
// ---------------------------------------------------------------------------

/// A process monitor handle (the C type is also `ErlDrvMonitor`). 32 bytes,
/// opaque; pass only by pointer.
///
/// [`ErlNifMonitor`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifMonitor) — NIF 2.12 — OTP 20
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Monitor(pub [u8; 32]);

// ---------------------------------------------------------------------------
// Resource type
// ---------------------------------------------------------------------------

/// An opaque resource type handle.
///
/// [`ErlNifResourceType`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifResourceType) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct ResourceType {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// Callback table for resource type registration.
///
/// `members` must equal the number of callback fields provided, counting from
/// the start: 1 = dtor, 2 = +stop, 3 = +down, 4 = +dyncall; `dyncall` was added
/// in NIF 2.16 (OTP 24). The field is always present — set `members` to bound
/// what the target BEAM understands.
///
/// [`ErlNifResourceTypeInit`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifResourceTypeInit) — NIF 2.12 — OTP 20
#[repr(C)]
pub struct ResourceTypeInit {
    pub dtor: Option<unsafe extern "C" fn(*mut Env, *mut c_void)>,
    pub stop: Option<unsafe extern "C" fn(*mut Env, *mut c_void, Event, c_int)>,
    pub down: Option<unsafe extern "C" fn(*mut Env, *mut c_void, *mut Pid, *mut Monitor)>,
    pub members: c_int,
    pub dyncall: Option<unsafe extern "C" fn(*mut Env, *mut c_void, *mut c_void)>,
}

/// Flags passed to resource type registration. Combine with `|`:
/// `ResourceFlags::CREATE | ResourceFlags::TAKEOVER`.
///
/// [`ErlNifResourceFlags`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifResourceFlags) — NIF 1.0 — OTP R13B04
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ResourceFlags(pub c_int);

impl ResourceFlags {
    /// Create a new resource type.
    pub const CREATE: Self = Self(1);
    /// Take over from an old NIF library during upgrade.
    pub const TAKEOVER: Self = Self(2);
}

impl BitOr for ResourceFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

// ---------------------------------------------------------------------------
// OS event handle (for enif_select)
// ---------------------------------------------------------------------------

/// An OS event handle.
///
/// [`ErlNifEvent`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifEvent) — NIF 2.12 — OTP 20
#[cfg(unix)]
pub type Event = c_int;
#[cfg(windows)]
pub type Event = *mut c_void;

// ---------------------------------------------------------------------------
// Map iterator
// ---------------------------------------------------------------------------

/// Starting position for a map iterator.
///
/// [`ErlNifMapIteratorEntry`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifMapIteratorEntry) — NIF 2.6 — OTP 17
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapIteratorEntry {
    First = 1,
    Last = 2,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MapIteratorFlat {
    ks: *mut Term,
    vs: *mut Term,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MapIteratorHash {
    wstack: *mut c_void,
    kv: *mut Term,
}

#[repr(C)]
union MapIteratorUnion {
    flat: MapIteratorFlat,
    hash: MapIteratorHash,
}

/// Map iteration cursor. All fields but `map` are internal to the BEAM. Created
/// by `map_iterator_create`, destroyed by `map_iterator_destroy`; must not be
/// moved after init.
///
/// [`ErlNifMapIterator`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifMapIterator) — NIF 2.6 — OTP 17
#[repr(C)]
pub struct MapIterator {
    pub map: Term,
    size: usize,
    idx: usize,
    u: MapIteratorUnion,
    _spare: [*mut c_void; 2],
}

// ---------------------------------------------------------------------------
// Term type tag
// ---------------------------------------------------------------------------

/// The canonical term types returned by `term_type`.
///
/// The C header reserves a `-1` sentinel and may add new types, so an
/// unrecognized code must never be transmuted into this enum.
///
/// [`ErlNifTermType`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTermType) — NIF 2.15 — OTP 22
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TermType {
    Atom = 1,
    Bitstring = 2,
    Float = 3,
    Fun = 4,
    Integer = 5,
    List = 6,
    Map = 7,
    Pid = 8,
    Port = 9,
    Reference = 10,
    Tuple = 11,
}

impl TermType {
    /// Map a raw `term_type` return code to a known variant, or `None` for any
    /// code outside the canonical `1..=11` (an unknown/future term type).
    pub fn from_raw(code: c_int) -> Option<Self> {
        match code {
            1 => Some(Self::Atom),
            2 => Some(Self::Bitstring),
            3 => Some(Self::Float),
            4 => Some(Self::Fun),
            5 => Some(Self::Integer),
            6 => Some(Self::List),
            7 => Some(Self::Map),
            8 => Some(Self::Pid),
            9 => Some(Self::Port),
            10 => Some(Self::Reference),
            11 => Some(Self::Tuple),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Character encoding
// ---------------------------------------------------------------------------

/// Encoding for reading/writing atom and string names.
///
/// [`ErlNifCharEncoding`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifCharEncoding) — NIF 1.0 — OTP R13B04
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CharEncoding {
    Latin1 = 1,
    /// NIF 2.17 (OTP 26.0).
    #[cfg(feature = "nif_2_17")]
    Utf8 = 2,
}

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

/// A time value in BEAM time units.
///
/// [`ErlNifTime`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTime) — NIF 2.10 — OTP 18.3
pub type Time = i64;

/// Sentinel returned by time functions on error.
///
/// [`ERL_NIF_TIME_ERROR`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TIME_ERROR) — NIF 2.10 — OTP 18.3
pub const TIME_ERROR: Time = i64::MIN;

/// Time unit for `monotonic_time` etc.
///
/// [`ErlNifTimeUnit`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTimeUnit) — NIF 2.10 — OTP 18.3
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TimeUnit {
    Second = 0,
    Millisecond = 1,
    Microsecond = 2,
    Nanosecond = 3,
}

// ---------------------------------------------------------------------------
// Unique integer flags
// ---------------------------------------------------------------------------

/// Flags for `make_unique_integer`. Combine with `|`.
///
/// [`ErlNifUniqueInteger`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifUniqueInteger) — NIF 2.11 — OTP 19
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UniqueInteger(pub c_int);

impl UniqueInteger {
    /// Return a positive integer only.
    pub const POSITIVE: Self = Self(1 << 0);
    /// Return a strictly monotonic integer.
    pub const MONOTONIC: Self = Self(1 << 1);
}

impl BitOr for UniqueInteger {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

// ---------------------------------------------------------------------------
// Hash
// ---------------------------------------------------------------------------

/// Hash algorithm for `hash`.
///
/// [`ErlNifHash`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifHash) — NIF 2.12 — OTP 20
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hash {
    InternalHash = 1,
    Phash2 = 2,
}

// ---------------------------------------------------------------------------
// Select (I/O event multiplexing)
// ---------------------------------------------------------------------------

/// Input flags for `select`. Combine with `|`:
/// `SelectFlags::READ | SelectFlags::CUSTOM_MSG`. Defined in `erl_drv_nif.h`;
/// the individual flags carry their own introduction versions.
///
/// [`ErlNifSelectFlags`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifSelectFlags) — NIF 2.12 — OTP 20
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SelectFlags(pub c_int);

impl SelectFlags {
    /// NIF 2.12 (OTP 20.0).
    pub const READ: Self = Self(1 << 0);
    /// NIF 2.12 (OTP 20.0).
    pub const WRITE: Self = Self(1 << 1);
    /// NIF 2.12 (OTP 20.0).
    pub const STOP: Self = Self(1 << 2);
    /// NIF 2.15 (OTP 22.0).
    pub const CANCEL: Self = Self(1 << 3);
    /// NIF 2.15 (OTP 22.0).
    pub const CUSTOM_MSG: Self = Self(1 << 4);
    /// NIF 2.16 (OTP 24.0).
    #[cfg(feature = "nif_2_16")]
    pub const ERROR: Self = Self(1 << 5);
}

impl BitOr for SelectFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// `select`'s stop callback ran on the calling thread.
///
/// [`ERL_NIF_SELECT_STOP_CALLED`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_STOP_CALLED) — NIF 2.12 — OTP 20
pub const SELECT_STOP_CALLED: c_int = 1 << 0;
/// `select`'s stop callback was scheduled to run on another thread.
///
/// [`ERL_NIF_SELECT_STOP_SCHEDULED`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_STOP_SCHEDULED) — NIF 2.12 — OTP 20
pub const SELECT_STOP_SCHEDULED: c_int = 1 << 1;
/// `select` was given an invalid event object.
///
/// [`ERL_NIF_SELECT_INVALID_EVENT`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_INVALID_EVENT) — NIF 2.12 — OTP 20
pub const SELECT_INVALID_EVENT: c_int = 1 << 2;
/// `select` failed to add the event to the poll set.
///
/// [`ERL_NIF_SELECT_FAILED`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_FAILED) — NIF 2.12 — OTP 20
pub const SELECT_FAILED: c_int = 1 << 3;
/// A pending read `select` was cancelled.
///
/// [`ERL_NIF_SELECT_READ_CANCELLED`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_READ_CANCELLED) — NIF 2.15 — OTP 22
pub const SELECT_READ_CANCELLED: c_int = 1 << 4;
/// A pending write `select` was cancelled.
///
/// [`ERL_NIF_SELECT_WRITE_CANCELLED`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_WRITE_CANCELLED) — NIF 2.15 — OTP 22
pub const SELECT_WRITE_CANCELLED: c_int = 1 << 5;
/// A pending error `select` was cancelled.
///
/// [`ERL_NIF_SELECT_ERROR_CANCELLED`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_ERROR_CANCELLED) — NIF 2.16 — OTP 24
#[cfg(feature = "nif_2_16")]
pub const SELECT_ERROR_CANCELLED: c_int = 1 << 6;
/// The requested `select` mode is not supported for the event object.
///
/// [`ERL_NIF_SELECT_NOTSUP`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_NOTSUP) — NIF 2.16 — OTP 24
#[cfg(feature = "nif_2_16")]
pub const SELECT_NOTSUP: c_int = 1 << 7;

// ---------------------------------------------------------------------------
// binary_to_term options
// ---------------------------------------------------------------------------

/// Safe decoding for `binary_to_term`: reject encoded atoms that don't already
/// exist.
///
/// [`ERL_NIF_BIN2TERM_SAFE`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_BIN2TERM_SAFE) — NIF 2.11 — OTP 19
pub const BIN2TERM_SAFE: c_uint = 0x2000_0000;

// ---------------------------------------------------------------------------
// System info
// ---------------------------------------------------------------------------

/// BEAM system information (the C type is also `ErlDrvSysInfo`).
///
/// [`ErlNifSysInfo`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifSysInfo) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct SysInfo {
    pub driver_major_version: c_int,
    pub driver_minor_version: c_int,
    pub erts_version: *mut c_char,
    pub otp_release: *mut c_char,
    pub thread_support: c_int,
    pub smp_support: c_int,
    pub async_threads: c_int,
    pub scheduler_threads: c_int,
    pub nif_major_version: c_int,
    pub nif_minor_version: c_int,
    pub dirty_scheduler_support: c_int,
}

// ---------------------------------------------------------------------------
// NIF options (enif_set_option) — NIF 2.17 (OTP 26.0)
// ---------------------------------------------------------------------------

/// Option key for `set_option`. The trailing underscore avoids shadowing the
/// `std` prelude `Option`.
///
/// [`ErlNifOption`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifOption) — NIF 2.17 — OTP 26
#[cfg(feature = "nif_2_17")]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Option_ {
    DelayHalt = 1,
    OnHalt = 2,
    /// NIF 2.17 (OTP 27.0) — added within the 2.17 line; passing it to an
    /// OTP-26 runtime is a caller error.
    OnUnloadThread = 3,
}

// ---------------------------------------------------------------------------
// Thread type (return values from enif_thread_type)
// ---------------------------------------------------------------------------

/// Not a scheduler thread.
///
/// [`ERL_NIF_THR_UNDEFINED`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_THR_UNDEFINED) — NIF 2.11 — OTP 19
pub const THR_UNDEFINED: c_int = 0;
/// Normal BEAM scheduler thread.
///
/// [`ERL_NIF_THR_NORMAL_SCHEDULER`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_THR_NORMAL_SCHEDULER) — NIF 2.11 — OTP 19
pub const THR_NORMAL_SCHEDULER: c_int = 1;
/// Dirty CPU scheduler thread.
///
/// [`ERL_NIF_THR_DIRTY_CPU_SCHEDULER`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_THR_DIRTY_CPU_SCHEDULER) — NIF 2.11 — OTP 19
pub const THR_DIRTY_CPU_SCHEDULER: c_int = 2;
/// Dirty I/O scheduler thread.
///
/// [`ERL_NIF_THR_DIRTY_IO_SCHEDULER`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_THR_DIRTY_IO_SCHEDULER) — NIF 2.11 — OTP 19
pub const THR_DIRTY_IO_SCHEDULER: c_int = 3;

// ---------------------------------------------------------------------------
// Dirty scheduler flags
// ---------------------------------------------------------------------------
// The two `ERL_NIF_DIRTY_JOB_*` constants. They apply to both the
// `enif_schedule_nif` `flags` argument and the [`Func::flags`] field (the
// latter is `c_uint`, so cast there); a regular NIF is just `0`.

/// Run on a dirty CPU scheduler.
///
/// [`ERL_NIF_DIRTY_JOB_CPU_BOUND`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_DIRTY_JOB_CPU_BOUND) — NIF 2.6 — OTP 17
pub const DIRTY_JOB_CPU_BOUND: c_int = 1;
/// Run on a dirty I/O scheduler.
///
/// [`ERL_NIF_DIRTY_JOB_IO_BOUND`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_DIRTY_JOB_IO_BOUND) — NIF 2.6 — OTP 17
pub const DIRTY_JOB_IO_BOUND: c_int = 2;

// ---------------------------------------------------------------------------
// I/O queue and iovec
// ---------------------------------------------------------------------------

/// An opaque I/O queue handle.
///
/// [`ErlNifIOQueue`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifIOQueue) — NIF 2.12 — OTP 20.1
#[repr(C)]
pub struct IOQueue {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// I/O queue creation options.
///
/// [`ErlNifIOQueueOpts`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifIOQueueOpts) — NIF 2.12 — OTP 20.1
pub type IOQueueOpts = c_int;

/// Normal I/O queue mode.
///
/// [`ERL_NIF_IOQ_NORMAL`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_IOQ_NORMAL) — NIF 2.12 — OTP 20.1
pub const IOQ_NORMAL: IOQueueOpts = 1;

/// A scatter/gather I/O vector.
///
/// [`ErlNifIOVec`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifIOVec) — NIF 2.12 — OTP 20.1
#[repr(C)]
pub struct IOVec {
    pub iovcnt: c_int,
    pub size: usize,
    pub iov: *mut SysIOVec,
    ref_bins: *mut *mut c_void,
    flags: c_int,
    small_iov: [SysIOVec; 16],
    small_ref_bin: [*mut c_void; 16],
}

// ---------------------------------------------------------------------------
// Opaque thread-primitive handles
// ---------------------------------------------------------------------------

/// An opaque mutex handle.
///
/// [`ErlNifMutex`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifMutex) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct Mutex {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque condition variable handle.
///
/// [`ErlNifCond`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifCond) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct Cond {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque read-write lock handle.
///
/// [`ErlNifRWLock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifRWLock) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct RWLock {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// A thread identifier (in C, `struct ErlDrvTid_ *`).
///
/// [`ErlNifTid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTid) — NIF 1.0 — OTP R13B04
pub type Tid = *mut c_void;

/// A thread-specific data key.
///
/// [`ErlNifTSDKey`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTSDKey) — NIF 1.0 — OTP R13B04
pub type TSDKey = c_int;

/// Thread creation options.
///
/// [`ErlNifThreadOpts`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifThreadOpts) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct ThreadOpts {
    pub suggested_stack_size: c_int,
}
