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

/// The major NIF API version this build targets.
///
/// Always 2; the major number has not changed since the modern NIF API. Written
/// into the library [`Entry`] reported to the BEAM at load.
///
/// [`ERL_NIF_MAJOR_VERSION`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_MAJOR_VERSION) — NIF 0.1 — OTP R13B03
pub const MAJOR_VERSION: c_int = 2;

/// The highest NIF minor version this build targets.
///
/// Set from the enabled feature rung (15, 16, 17, or 18) and written into the
/// library [`Entry`]. The BEAM refuses to load the library if its own NIF
/// version is lower than this.
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

/// The minimum ERTS version the library requires.
///
/// An `erts-X.Y` string tracking the enabled feature rung, written into the
/// [`Entry`]. The BEAM refuses to load the library on an older runtime.
///
/// [`ERL_NIF_MIN_ERTS_VERSION`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_MIN_ERTS_VERSION) — NIF 2.14 — OTP 21
#[cfg(not(feature = "nif_2_16"))]
pub const MIN_ERTS_VERSION: &CStr = c"erts-10.4";
#[cfg(all(feature = "nif_2_16", not(feature = "nif_2_17")))]
pub const MIN_ERTS_VERSION: &CStr = c"erts-12.0";
#[cfg(feature = "nif_2_17")]
pub const MIN_ERTS_VERSION: &CStr = c"erts-14.0";

/// The VM variant the library is built for.
///
/// Always `"beam.vanilla"`. Written into the [`Entry`]; the BEAM checks it
/// against the running emulator at load.
///
/// [`ERL_NIF_VM_VARIANT`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_VM_VARIANT) — NIF 2.1 — OTP R14B02
pub const VM_VARIANT: &CStr = c"beam.vanilla";

// ---------------------------------------------------------------------------
// Core term type
// ---------------------------------------------------------------------------

/// Any Erlang term, as an opaque tagged word.
///
/// A pointer-sized tagged machine word whose bit layout is private to the
/// runtime. A NIF must only ever inspect or build terms through the `enif_*`
/// functions, never by interpreting the integer directly.
///
/// [`ERL_NIF_TERM`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM) — NIF 0.1 — OTP R13B03
pub type Term = usize;

// ---------------------------------------------------------------------------
// Opaque environment
// ---------------------------------------------------------------------------

/// A NIF environment that owns terms.
///
/// Always handled as `*mut Env` and never constructed by a NIF. A call
/// environment is passed into each NIF and lives for the duration of the call; a
/// process-independent one from [`alloc_env`](crate::alloc_env) lives until [`free_env`](crate::free_env). Every
/// term is bound to some environment.
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

/// The descriptor for a single exported NIF.
///
/// Pairs an Erlang function `name` and `arity` with the C function pointer
/// `fptr`. `flags` is `0` for a regular NIF, or [`DIRTY_JOB_CPU_BOUND`] /
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

/// The library descriptor the BEAM reads at load.
///
/// Built and returned by `nif_init`, it lists the module's functions and the
/// load/upgrade/unload callbacks, plus version and metadata fields. All tail
/// fields through `min_erts` are always present; an older BEAM simply reads
/// fewer of them.
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
    /// The VM variant the library was built for.
    ///
    /// Set to [`VM_VARIANT`]; the BEAM rejects a mismatch at load.
    ///
    /// NIF 2.1 (OTP R14B02).
    pub vm_variant: *const c_char,
    /// Options word signalling which tail fields are populated.
    ///
    /// Set to `1` to indicate that `sizeof_resource_type_init` is present,
    /// otherwise `0`.
    ///
    /// NIF 2.7 (OTP 17.3).
    pub options: c_uint,
    /// Size of [`ResourceTypeInit`], for forward-compatible resources.
    ///
    /// Must equal `size_of::<ResourceTypeInit>()` so the BEAM knows how much of
    /// the callback struct this build understands. Read only when `options` is
    /// `1`.
    ///
    /// NIF 2.12 (OTP 20.0).
    pub sizeof_resource_type_init: usize,
    /// The minimum ERTS version the library requires.
    ///
    /// Set to [`MIN_ERTS_VERSION`]; the BEAM refuses to load on an older runtime.
    ///
    /// NIF 2.14 (OTP 21).
    pub min_erts: *const c_char,
}

// ---------------------------------------------------------------------------
// Binary
// ---------------------------------------------------------------------------

/// A binary's bytes, as a size and data pointer.
///
/// `size` and `data` describe the bytes; the `ref_bin`/`_spare` fields are
/// BEAM-internal bookkeeping and must be left untouched. Filled by
/// [`inspect_binary`](crate::inspect_binary) for borrowing, or [`alloc_binary`](crate::alloc_binary) for an owned, mutable
/// binary.
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
/// Wraps the pid term in `pid`. Obtained from [`self_`](crate::self_), [`get_local_pid`](crate::get_local_pid), or
/// [`whereis_pid`](crate::whereis_pid), turned back into a term with [`make_pid`](crate::make_pid), and may be flagged
/// undefined with [`set_pid_undefined`](crate::set_pid_undefined).
///
/// [`ErlNifPid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifPid) — NIF 2.0 — OTP R14A
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pid {
    pub pid: Term,
}

/// A port identifier.
///
/// Wraps the port term in `port_id`. Obtained from [`get_local_port`](crate::get_local_port) or
/// [`whereis_port`](crate::whereis_port).
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

/// A process-monitor handle.
///
/// A 32-byte opaque value — the C type is also `ErlDrvMonitor` — always passed by
/// pointer and never interpreted. Produced by [`monitor_process`](crate::monitor_process), ordered with
/// [`compare_monitors`](crate::compare_monitors), and turned into a term with [`make_monitor_term`](crate::make_monitor_term).
///
/// [`ErlNifMonitor`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifMonitor) — NIF 2.12 — OTP 20
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct Monitor(pub [u8; 32]);

// ---------------------------------------------------------------------------
// Resource type
// ---------------------------------------------------------------------------

/// A handle to a registered resource type.
///
/// An opaque value returned by [`open_resource_type`](crate::open_resource_type),
/// [`open_resource_type_x`](crate::open_resource_type_x), or `init_resource_type`
/// and passed to [`alloc_resource`](crate::alloc_resource) and
/// [`get_resource`](crate::get_resource). Always handled as `*mut ResourceType`.
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

/// Create-or-take-over flags for registering a resource type.
///
/// Combine with `|`, e.g. `ResourceFlags::CREATE | ResourceFlags::TAKEOVER`.
/// Passed to [`open_resource_type`](crate::open_resource_type) and friends, which report through their
/// `tried` out-parameter which operation actually occurred.
///
/// [`ErlNifResourceFlags`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifResourceFlags) — NIF 1.0 — OTP R13B04
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ResourceFlags(pub c_int);

impl ResourceFlags {
    /// Register the name as a new resource type.
    ///
    /// Combine with [`ResourceFlags::TAKEOVER`] to accept either a fresh
    /// registration or a takeover of an existing one.
    ///
    /// [`ERL_NIF_RT_CREATE`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_RT_CREATE) — NIF 1.0 — OTP R13B04
    pub const CREATE: Self = Self(1);
    /// Take over an existing resource type during a code upgrade.
    ///
    /// Lets the new library inherit a type registered by the version it replaces.
    /// Combine with [`ResourceFlags::CREATE`] to allow either.
    ///
    /// [`ERL_NIF_RT_TAKEOVER`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_RT_TAKEOVER) — NIF 1.0 — OTP R13B04
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

/// An OS event handle for use with [`select`](crate::select).
///
/// A file descriptor on Unix (`c_int`) or a `HANDLE` on Windows (`*mut c_void`).
/// Registered for readiness notifications through [`select`](crate::select), with a resource
/// owning its lifetime.
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
    /// Start iterating at the first entry.
    ///
    /// [`ERL_NIF_MAP_ITERATOR_FIRST`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_MAP_ITERATOR_FIRST) — NIF 2.6 — OTP 17
    First = 1,
    /// Start iterating at the last entry.
    ///
    /// [`ERL_NIF_MAP_ITERATOR_LAST`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_MAP_ITERATOR_LAST) — NIF 2.6 — OTP 17
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

/// A cursor for iterating a map's entries.
///
/// Only `map` is public; the remaining fields are BEAM-internal. Initialized by
/// [`map_iterator_create`](crate::map_iterator_create) and released by [`map_iterator_destroy`](crate::map_iterator_destroy), it must not
/// be moved after initialization. Iteration order is unspecified.
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
    /// Matches an atom.
    ///
    /// [`ERL_NIF_TERM_TYPE_ATOM`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_ATOM) — NIF 2.15 — OTP 22
    Atom = 1,
    /// Matches a bitstring (binaries included).
    ///
    /// [`ERL_NIF_TERM_TYPE_BITSTRING`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_BITSTRING) — NIF 2.15 — OTP 22
    Bitstring = 2,
    /// Matches a float.
    ///
    /// [`ERL_NIF_TERM_TYPE_FLOAT`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_FLOAT) — NIF 2.15 — OTP 22
    Float = 3,
    /// Matches a fun.
    ///
    /// [`ERL_NIF_TERM_TYPE_FUN`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_FUN) — NIF 2.15 — OTP 22
    Fun = 4,
    /// Matches an integer.
    ///
    /// [`ERL_NIF_TERM_TYPE_INTEGER`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_INTEGER) — NIF 2.15 — OTP 22
    Integer = 5,
    /// Matches a list (the empty list included).
    ///
    /// [`ERL_NIF_TERM_TYPE_LIST`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_LIST) — NIF 2.15 — OTP 22
    List = 6,
    /// Matches a map.
    ///
    /// [`ERL_NIF_TERM_TYPE_MAP`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_MAP) — NIF 2.15 — OTP 22
    Map = 7,
    /// Matches a pid.
    ///
    /// [`ERL_NIF_TERM_TYPE_PID`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_PID) — NIF 2.15 — OTP 22
    Pid = 8,
    /// Matches a port.
    ///
    /// [`ERL_NIF_TERM_TYPE_PORT`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_PORT) — NIF 2.15 — OTP 22
    Port = 9,
    /// Matches a reference.
    ///
    /// [`ERL_NIF_TERM_TYPE_REFERENCE`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_REFERENCE) — NIF 2.15 — OTP 22
    Reference = 10,
    /// Matches a tuple.
    ///
    /// [`ERL_NIF_TERM_TYPE_TUPLE`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TERM_TYPE_TUPLE) — NIF 2.15 — OTP 22
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

/// Encoding for reading and writing atom and string bytes.
///
/// Selects how the byte buffer in functions like [`make_atom`](crate::make_atom), [`get_string`](crate::get_string),
/// and [`make_string`](crate::make_string) is interpreted. `Latin1` is one byte per character;
/// `Utf8` (NIF 2.17) is variable-width.
///
/// [`ErlNifCharEncoding`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifCharEncoding) — NIF 1.0 — OTP R13B04
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CharEncoding {
    /// Latin-1 (ISO-8859-1).
    ///
    /// [`ERL_NIF_LATIN1`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_LATIN1) — NIF 1.0 — OTP R13B04
    Latin1 = 1,
    /// UTF-8.
    ///
    /// [`ERL_NIF_UTF8`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_UTF8) — NIF 2.17 — OTP 26
    #[cfg(feature = "nif_2_17")]
    Utf8 = 2,
}

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

/// A time value in BEAM time units.
///
/// A signed 64-bit count whose unit is given by an accompanying [`TimeUnit`].
/// Monotonic times are frequently negative; see [`monotonic_time`](crate::monotonic_time).
///
/// [`ErlNifTime`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTime) — NIF 2.10 — OTP 18.3
pub type Time = i64;

/// Sentinel returned by the time functions on error.
///
/// Equal to `i64::MIN`. Returned when a time value cannot be represented in the
/// requested [`TimeUnit`].
///
/// [`ERL_NIF_TIME_ERROR`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_TIME_ERROR) — NIF 2.10 — OTP 18.3
pub const TIME_ERROR: Time = i64::MIN;

/// The unit of a [`Time`] value.
///
/// Selects the unit passed to or returned by [`monotonic_time`](crate::monotonic_time), [`time_offset`](crate::time_offset),
/// and [`convert_time_unit`](crate::convert_time_unit).
///
/// [`ErlNifTimeUnit`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTimeUnit) — NIF 2.10 — OTP 18.3
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TimeUnit {
    /// Seconds.
    ///
    /// [`ERL_NIF_SEC`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SEC) — NIF 2.10 — OTP 18.3
    Second = 0,
    /// Milliseconds.
    ///
    /// [`ERL_NIF_MSEC`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_MSEC) — NIF 2.10 — OTP 18.3
    Millisecond = 1,
    /// Microseconds.
    ///
    /// [`ERL_NIF_USEC`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_USEC) — NIF 2.10 — OTP 18.3
    Microsecond = 2,
    /// Nanoseconds.
    ///
    /// [`ERL_NIF_NSEC`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_NSEC) — NIF 2.10 — OTP 18.3
    Nanosecond = 3,
}

// ---------------------------------------------------------------------------
// Unique integer flags
// ---------------------------------------------------------------------------

/// Flags shaping the result of [`make_unique_integer`](crate::make_unique_integer). Combine with `|`.
///
/// The same options as `erlang:unique_integer/1`. With no flags the integer is
/// merely unique and may be negative; the flags constrain it to be positive
/// and/or strictly monotonic.
///
/// [`ErlNifUniqueInteger`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifUniqueInteger) — NIF 2.11 — OTP 19
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UniqueInteger(pub c_int);

impl UniqueInteger {
    /// Return a positive integer only.
    ///
    /// [`ERL_NIF_UNIQUE_POSITIVE`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_UNIQUE_POSITIVE) — NIF 2.11 — OTP 19
    pub const POSITIVE: Self = Self(1 << 0);
    /// Return a strictly monotonic integer.
    ///
    /// [`ERL_NIF_UNIQUE_MONOTONIC`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_UNIQUE_MONOTONIC) — NIF 2.11 — OTP 19
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

/// The hash algorithm selector for [`hash`](crate::hash).
///
/// Chooses between a fast non-portable internal hash and the portable `phash2`
/// that matches `erlang:phash2/1`.
///
/// [`ErlNifHash`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifHash) — NIF 2.12 — OTP 20
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hash {
    /// Non-portable internal hash; fast, but may change between ERTS versions.
    ///
    /// [`ERL_NIF_INTERNAL_HASH`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_INTERNAL_HASH) — NIF 2.12 — OTP 20
    InternalHash = 1,
    /// Portable `phash2` hash, matching `erlang:phash2/1`.
    ///
    /// [`ERL_NIF_PHASH2`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_PHASH2) — NIF 2.12 — OTP 20
    Phash2 = 2,
}

// ---------------------------------------------------------------------------
// Select (I/O event multiplexing)
// ---------------------------------------------------------------------------

/// Mode flags for [`select`](crate::select). Combine with `|`.
///
/// For example `SelectFlags::READ | SelectFlags::CUSTOM_MSG`. Defined in
/// `erl_drv_nif.h`; each flag carries its own introduction version, as the
/// cancel, custom-message, and error modes were added after the base ones.
///
/// [`ErlNifSelectFlags`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifSelectFlags) — NIF 2.12 — OTP 20
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SelectFlags(pub c_int);

impl SelectFlags {
    /// Select for read readiness.
    ///
    /// [`ERL_NIF_SELECT_READ`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_READ) — NIF 2.12 — OTP 20
    pub const READ: Self = Self(1 << 0);
    /// Select for write readiness.
    ///
    /// [`ERL_NIF_SELECT_WRITE`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_WRITE) — NIF 2.12 — OTP 20
    pub const WRITE: Self = Self(1 << 1);
    /// Stop selecting on the event and trigger the resource stop callback.
    ///
    /// [`ERL_NIF_SELECT_STOP`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_STOP) — NIF 2.12 — OTP 20
    pub const STOP: Self = Self(1 << 2);
    /// Cancel a pending read or write select without stopping.
    ///
    /// [`ERL_NIF_SELECT_CANCEL`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_CANCEL) — NIF 2.15 — OTP 22
    pub const CANCEL: Self = Self(1 << 3);
    /// Deliver `msg` as a custom message instead of the default select message.
    ///
    /// [`ERL_NIF_SELECT_CUSTOM_MSG`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_CUSTOM_MSG) — NIF 2.15 — OTP 22
    pub const CUSTOM_MSG: Self = Self(1 << 4);
    /// Select for error conditions on the event.
    ///
    /// [`ERL_NIF_SELECT_ERROR`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_SELECT_ERROR) — NIF 2.16 — OTP 24
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

/// A snapshot of BEAM runtime information.
///
/// Filled by [`system_info`](crate::system_info) (the C type is also `ErlDrvSysInfo`): ERTS and OTP
/// version strings, the NIF and driver version numbers, scheduler counts, and the
/// SMP, thread, and dirty-scheduler support flags.
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

/// The key selecting which runtime option `set_option` sets.
///
/// Used by the [`set_option_delay_halt`](crate::set_option_delay_halt), [`set_option_on_halt`](crate::set_option_on_halt), and
/// [`set_option_on_unload_thread`](crate::set_option_on_unload_thread) wrappers. The trailing underscore avoids
/// shadowing the `std` prelude `Option`.
///
/// [`ErlNifOption`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifOption) — NIF 2.17 — OTP 26
#[cfg(feature = "nif_2_17")]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Option_ {
    /// Delay runtime-system halt until running NIF calls have returned.
    ///
    /// [`ERL_NIF_OPT_DELAY_HALT`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_OPT_DELAY_HALT) — NIF 2.17 — OTP 26
    DelayHalt = 1,
    /// Register an on-halt callback, run when the runtime system halts.
    ///
    /// [`ERL_NIF_OPT_ON_HALT`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_OPT_ON_HALT) — NIF 2.17 — OTP 26
    OnHalt = 2,
    /// Register an on-unload-thread callback. Added within the 2.17 line, so
    /// passing it to an OTP-26 runtime is a caller error.
    ///
    /// [`ERL_NIF_OPT_ON_UNLOAD_THREAD`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_OPT_ON_UNLOAD_THREAD) — NIF 2.17 — OTP 27
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
/// A FIFO of binary data used to stage output without copying. Created with
/// [`ioq_create`](crate::ioq_create) and destroyed with [`ioq_destroy`](crate::ioq_destroy); always handled as
/// `*mut IOQueue`.
///
/// [`ErlNifIOQueue`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifIOQueue) — NIF 2.12 — OTP 20.1
#[repr(C)]
pub struct IOQueue {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// Creation mode for an I/O queue.
///
/// The `opts` argument to [`ioq_create`](crate::ioq_create); the only defined value is
/// [`IOQ_NORMAL`].
///
/// [`ErlNifIOQueueOpts`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifIOQueueOpts) — NIF 2.12 — OTP 20.1
pub type IOQueueOpts = c_int;

/// Normal I/O queue mode.
///
/// [`ERL_NIF_IOQ_NORMAL`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ERL_NIF_IOQ_NORMAL) — NIF 2.12 — OTP 20.1
pub const IOQ_NORMAL: IOQueueOpts = 1;

/// A scatter/gather I/O vector.
///
/// `iovcnt` [`SysIOVec`] segments at `iov` spanning `size` total bytes; the
/// remaining fields are BEAM-internal. Produced from an iolist by
/// [`inspect_iovec`](crate::inspect_iovec) and consumed by [`ioq_enqv`](crate::ioq_enqv).
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
/// Created with [`mutex_create`](crate::mutex_create) and destroyed with [`mutex_destroy`](crate::mutex_destroy); always
/// handled as `*mut Mutex`.
///
/// [`ErlNifMutex`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifMutex) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct Mutex {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque condition variable handle.
///
/// Created with [`cond_create`](crate::cond_create) and destroyed with [`cond_destroy`](crate::cond_destroy); waited on
/// with [`cond_wait`](crate::cond_wait). Always handled as `*mut Cond`.
///
/// [`ErlNifCond`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifCond) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct Cond {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque read/write lock handle.
///
/// Created with [`rwlock_create`](crate::rwlock_create) and destroyed with [`rwlock_destroy`](crate::rwlock_destroy); always
/// handled as `*mut RWLock`.
///
/// [`ErlNifRWLock`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifRWLock) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct RWLock {
    _opaque: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// A thread identifier.
///
/// Returned by [`thread_self`](crate::thread_self), written by [`thread_create`](crate::thread_create), and compared with
/// [`equal_tids`](crate::equal_tids). An opaque pointer (in C, `struct ErlDrvTid_ *`).
///
/// [`ErlNifTid`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTid) — NIF 1.0 — OTP R13B04
pub type Tid = *mut c_void;

/// A thread-specific data key.
///
/// Created with [`tsd_key_create`](crate::tsd_key_create); each thread stores and reads its own pointer
/// for the key with [`tsd_set`](crate::tsd_set) and [`tsd_get`](crate::tsd_get).
///
/// [`ErlNifTSDKey`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifTSDKey) — NIF 1.0 — OTP R13B04
pub type TSDKey = c_int;

/// Options for creating a thread.
///
/// Allocated with [`thread_opts_create`](crate::thread_opts_create) and passed to [`thread_create`](crate::thread_create). The
/// single field `suggested_stack_size` is a stack-size hint, or `0` for the
/// default.
///
/// [`ErlNifThreadOpts`](https://www.erlang.org/doc/apps/erts/erl_nif.html#ErlNifThreadOpts) — NIF 1.0 — OTP R13B04
#[repr(C)]
pub struct ThreadOpts {
    pub suggested_stack_size: c_int,
}
