//! Raw C ABI types and constants mirroring `erl_nif.h` and `erl_drv_nif.h`.
//!
//! Direct `#[repr(C)]` transcriptions — no logic, no safety wrappers. Prefixes
//! are dropped per the crate naming convention; names that would shadow a Rust
//! keyword or a `std` prelude item take a trailing underscore (`Option_`).
//!
//! Each item notes the NIF version and OTP release that introduced it. The
//! crate floor is NIF 2.15 (OTP 22); anything newer is gated behind its rung.

use std::ffi::{CStr, c_char, c_int, c_uint, c_void};
use std::marker::{PhantomData, PhantomPinned};
use std::ops::BitOr;

// ---------------------------------------------------------------------------
// Library version
// ---------------------------------------------------------------------------

/// `ERL_NIF_MAJOR_VERSION`. Always 2.
pub const MAJOR_VERSION: c_int = 2;

/// `ERL_NIF_MINOR_VERSION` — the highest NIF minor this build targets, set from
/// the enabled feature rung. Reported to the BEAM in the library [`Entry`].
#[cfg(not(feature = "nif_2_16"))]
pub const MINOR_VERSION: c_int = 15;
#[cfg(all(feature = "nif_2_16", not(feature = "nif_2_17")))]
pub const MINOR_VERSION: c_int = 16;
#[cfg(all(feature = "nif_2_17", not(feature = "nif_2_18")))]
pub const MINOR_VERSION: c_int = 17;
#[cfg(feature = "nif_2_18")]
pub const MINOR_VERSION: c_int = 18;

/// `ERL_NIF_MIN_ERTS_VERSION` — the minimum ERTS the library declares it
/// requires, tracking the enabled feature rung. The BEAM refuses to load the
/// library on an older runtime.
#[cfg(not(feature = "nif_2_16"))]
pub const MIN_ERTS_VERSION: &CStr = c"erts-10.4";
#[cfg(all(feature = "nif_2_16", not(feature = "nif_2_17")))]
pub const MIN_ERTS_VERSION: &CStr = c"erts-12.0";
#[cfg(feature = "nif_2_17")]
pub const MIN_ERTS_VERSION: &CStr = c"erts-14.0";

/// `ERL_NIF_VM_VARIANT`. NIF 2.1 (OTP R14B02).
pub const VM_VARIANT: &CStr = c"beam.vanilla";

// ---------------------------------------------------------------------------
// Core term type
// ---------------------------------------------------------------------------

/// `ERL_NIF_TERM` — a tagged machine word, opaque to the NIF library.
/// NIF 1.0 (OTP R13B04).
pub type Term = usize;

/// `THE_NON_VALUE` — the BEAM's "no value" marker. No valid term is ever `0`,
/// so it doubles as an absent-term sentinel.
pub const NON_VALUE: Term = 0;

// ---------------------------------------------------------------------------
// Opaque environment
// ---------------------------------------------------------------------------

/// `ErlNifEnv` — per-call or process-independent NIF environment.
///
/// Always used as `*mut Env`. Never constructed directly. NIF 1.0 (OTP R13B04).
#[repr(C)]
pub struct Env {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

// ---------------------------------------------------------------------------
// Function and entry descriptors
// ---------------------------------------------------------------------------

/// `ErlNifFunc` — describes one NIF: Erlang name, arity, function pointer, flags.
/// NIF 1.0 (OTP R13B04); `flags` added in NIF 2.7 (OTP 17.3).
#[repr(C)]
pub struct Func {
    pub name: *const c_char,
    pub arity: c_uint,
    pub fptr: unsafe extern "C" fn(env: *mut Env, argc: c_int, argv: *const Term) -> Term,
    pub flags: c_uint,
}

/// [`Func::flags`]: run on a dirty CPU scheduler. NIF 2.7 (OTP 17.3).
pub const FUNC_DIRTY_CPU: c_uint = 1;
/// [`Func::flags`]: run on a dirty I/O scheduler. NIF 2.7 (OTP 17.3).
pub const FUNC_DIRTY_IO: c_uint = 2;

/// `ErlNifEntry` — the library descriptor returned by `nif_init()`.
/// NIF 1.0 (OTP R13B04), extended in later versions. All tail fields through
/// `min_erts` are always present; the BEAM reads only what its version knows.
#[repr(C)]
pub struct Entry {
    pub major: c_int,
    pub minor: c_int,
    pub name: *const c_char,
    pub num_of_funcs: c_int,
    pub funcs: *mut Func,
    pub load: Option<unsafe extern "C" fn(*mut Env, *mut *mut c_void, Term) -> c_int>,
    pub reload: Option<unsafe extern "C" fn(*mut Env, *mut *mut c_void, Term) -> c_int>,
    pub upgrade: Option<unsafe extern "C" fn(*mut Env, *mut *mut c_void, *mut *mut c_void, Term) -> c_int>,
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

/// `ErlNifBinary` — inspected binary: byte count and data pointer. The
/// `ref_bin`/`_spare` fields are internal to the BEAM. NIF 1.0 (OTP R13B04).
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

/// `ErlNifPid` — local process identifier. NIF 2.0 (OTP R14A).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pid {
    pub pid: Term,
}

/// `ErlNifPort` — port identifier. NIF 2.11 (OTP 19.0).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Port {
    pub port_id: Term,
}

// ---------------------------------------------------------------------------
// Monitor
// ---------------------------------------------------------------------------

/// `ErlNifMonitor` (= `ErlDrvMonitor`) — process monitor handle. 32 bytes,
/// opaque; pass only by pointer. NIF 2.12 (OTP 20.0).
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Monitor(pub [u8; 32]);

// ---------------------------------------------------------------------------
// Resource type
// ---------------------------------------------------------------------------

/// `ErlNifResourceType` — opaque resource type handle. NIF 2.0 (OTP R14A).
#[repr(C)]
pub struct ResourceType {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// `ErlNifResourceTypeInit` — callback table for resource type registration.
///
/// `members` must equal the number of callback fields provided, counting from
/// the start: 1 = dtor, 2 = +stop, 3 = +down, 4 = +dyncall. NIF 2.12 (OTP
/// 20.0); `dyncall` added in NIF 2.16 (OTP 24.0). The field is always present —
/// set `members` to bound what the target BEAM understands.
#[repr(C)]
pub struct ResourceTypeInit {
    pub dtor: Option<unsafe extern "C" fn(*mut Env, *mut c_void)>,
    pub stop: Option<unsafe extern "C" fn(*mut Env, *mut c_void, Event, c_int)>,
    pub down: Option<unsafe extern "C" fn(*mut Env, *mut c_void, *mut Pid, *mut Monitor)>,
    pub members: c_int,
    pub dyncall: Option<unsafe extern "C" fn(*mut Env, *mut c_void, *mut c_void)>,
}

/// `ErlNifResourceFlags` — passed to resource type registration. Combine with
/// `|`: `ResourceFlags::CREATE | ResourceFlags::TAKEOVER`. NIF 2.0 (OTP R14A).
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

/// `ErlNifEvent` — OS event handle. NIF 2.12 (OTP 20.0).
#[cfg(unix)]
pub type Event = c_int;
#[cfg(windows)]
pub type Event = *mut c_void;

// ---------------------------------------------------------------------------
// Map iterator
// ---------------------------------------------------------------------------

/// `ErlNifMapIteratorEntry` — starting position for a map iterator.
/// NIF 2.6 (OTP R17).
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

/// `ErlNifMapIterator` — map iteration cursor. All fields but `map` are internal
/// to the BEAM. Created by `map_iterator_create`, destroyed by
/// `map_iterator_destroy`; must not be moved after init. NIF 2.6 (OTP R17).
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

/// `ErlNifTermType` — the canonical term types returned by `term_type`.
///
/// The C header reserves a `-1` sentinel and may add new types, so an
/// unrecognized code must never be transmuted into this enum. NIF 2.15 (OTP
/// 22.0).
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

/// `ErlNifCharEncoding` — encoding for reading/writing atom and string names.
/// NIF 1.0 (OTP R13B04).
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

/// `ErlNifTime` — time value in BEAM time units. NIF 2.10 (OTP 18.3).
pub type Time = i64;

/// `ERL_NIF_TIME_ERROR` — sentinel returned by time functions on error.
/// NIF 2.10 (OTP 18.3).
pub const TIME_ERROR: Time = i64::MIN;

/// `ErlNifTimeUnit` — time unit for `monotonic_time` etc. NIF 2.10 (OTP 18.3).
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

/// `ErlNifUniqueInteger` — flags for `make_unique_integer`. Combine with `|`.
/// NIF 2.11 (OTP 19.0).
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

/// `ErlNifHash` — hash algorithm for `hash`. NIF 2.12 (OTP 20.0).
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hash {
    InternalHash = 1,
    Phash2 = 2,
}

// ---------------------------------------------------------------------------
// Select (I/O event multiplexing)
// ---------------------------------------------------------------------------

/// `ErlNifSelectFlags` — input flags for `select`. Combine with `|`:
/// `SelectFlags::READ | SelectFlags::CUSTOM_MSG`. Defined in `erl_drv_nif.h`.
/// NIF 2.12 (OTP 20.0); `CANCEL`/`CUSTOM_MSG` in NIF 2.15 (OTP 22.0); `ERROR`
/// in NIF 2.16 (OTP 24.0).
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

/// Return bit from `select`. NIF 2.12 (OTP 20.0).
pub const SELECT_STOP_CALLED: c_int = 1 << 0;
/// NIF 2.12 (OTP 20.0).
pub const SELECT_STOP_SCHEDULED: c_int = 1 << 1;
/// NIF 2.12 (OTP 20.0).
pub const SELECT_INVALID_EVENT: c_int = 1 << 2;
/// NIF 2.12 (OTP 20.0).
pub const SELECT_FAILED: c_int = 1 << 3;
/// NIF 2.15 (OTP 22.0).
pub const SELECT_READ_CANCELLED: c_int = 1 << 4;
/// NIF 2.15 (OTP 22.0).
pub const SELECT_WRITE_CANCELLED: c_int = 1 << 5;
/// NIF 2.16 (OTP 24.0).
#[cfg(feature = "nif_2_16")]
pub const SELECT_ERROR_CANCELLED: c_int = 1 << 6;
/// NIF 2.16 (OTP 24.0).
#[cfg(feature = "nif_2_16")]
pub const SELECT_NOTSUP: c_int = 1 << 7;

// ---------------------------------------------------------------------------
// binary_to_term options
// ---------------------------------------------------------------------------

/// Safe decoding for `binary_to_term`: reject encoded atoms that don't already
/// exist. NIF 2.11 (OTP 19.0).
pub const BIN2TERM_SAFE: c_uint = 0x2000_0000;

// ---------------------------------------------------------------------------
// System info
// ---------------------------------------------------------------------------

/// `ErlNifSysInfo` (= `ErlDrvSysInfo`) — BEAM system information.
/// NIF 1.0 (OTP R13B04).
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

/// `ErlNifOption` — option key for `set_option`. Trailing underscore avoids
/// shadowing the `std` prelude `Option`. NIF 2.17 (OTP 26.0).
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

/// Not a scheduler thread. NIF 2.11 (OTP 19.0).
pub const THR_UNDEFINED: c_int = 0;
/// Normal BEAM scheduler thread. NIF 2.11 (OTP 19.0).
pub const THR_NORMAL_SCHEDULER: c_int = 1;
/// Dirty CPU scheduler thread. NIF 2.11 (OTP 19.0).
pub const THR_DIRTY_CPU_SCHEDULER: c_int = 2;
/// Dirty I/O scheduler thread. NIF 2.11 (OTP 19.0).
pub const THR_DIRTY_IO_SCHEDULER: c_int = 3;

// ---------------------------------------------------------------------------
// Schedule NIF flags (enif_schedule_nif)
// ---------------------------------------------------------------------------

/// Run on a normal scheduler. NIF 2.7 (OTP 17.3).
pub const DIRTY_JOB_NORMAL: c_int = 0;
/// Run on a dirty CPU scheduler. NIF 2.7 (OTP 17.3).
pub const DIRTY_JOB_CPU_BOUND: c_int = 1;
/// Run on a dirty I/O scheduler. NIF 2.7 (OTP 17.3).
pub const DIRTY_JOB_IO_BOUND: c_int = 2;

// ---------------------------------------------------------------------------
// I/O queue and iovec
// ---------------------------------------------------------------------------

/// `ErlNifIOQueue` — opaque I/O queue handle. NIF 2.13 (OTP 20.1).
#[repr(C)]
pub struct IOQueue {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// `ErlNifIOQueueOpts` — I/O queue creation options. NIF 2.13 (OTP 20.1).
pub type IOQueueOpts = c_int;

/// Normal I/O queue mode. NIF 2.13 (OTP 20.1).
pub const IOQ_NORMAL: IOQueueOpts = 1;

/// `SysIOVec` — iovec on Unix; matches `struct iovec`. NIF 2.13 (OTP 20.1).
#[cfg(unix)]
#[repr(C)]
pub struct SysIOVec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

/// `ErlNifIOVec` — scatter/gather I/O vector. NIF 2.13 (OTP 20.1).
#[cfg(unix)]
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

/// `ErlNifMutex` — opaque mutex handle. NIF 1.0 (OTP R13B04).
#[repr(C)]
pub struct Mutex {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// `ErlNifCond` — opaque condition variable handle. NIF 1.0 (OTP R13B04).
#[repr(C)]
pub struct Cond {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// `ErlNifRWLock` — opaque read-write lock handle. NIF 1.0 (OTP R13B04).
#[repr(C)]
pub struct RWLock {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// `ErlNifTid` — thread identifier (in C, `struct ErlDrvTid_ *`).
/// NIF 1.0 (OTP R13B04).
pub type Tid = *mut c_void;

/// `ErlNifTSDKey` — thread-specific data key. NIF 1.0 (OTP R13B04).
pub type TSDKey = c_int;

/// `ErlNifThreadOpts` — thread creation options. NIF 1.0 (OTP R13B04).
#[repr(C)]
pub struct ThreadOpts {
    pub suggested_stack_size: c_int,
}
