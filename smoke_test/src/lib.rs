//! A minimal NIF written directly against `enif-ffi`, used to prove the raw
//! binding actually loads and runs inside the BEAM (cargo only type-checks it).
//!
//! It is also a worked example of a consumer: the function table, the
//! `ErlNifEntry` builder, and `enif_ffi::nif_init!` to define the entry
//! point. The same source compiles on Unix and Windows.

#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::{c_int, c_uint, c_void};
use std::mem::MaybeUninit;

use enif_ffi::*;

// ---------------------------------------------------------------------------
// NIF implementations
// ---------------------------------------------------------------------------

/// add(A, B) -> A + B. Exercises get_int / make_int.
unsafe extern "C" fn nif_add(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 2 {
        return unsafe { make_badarg(env) };
    }
    let args = unsafe { std::slice::from_raw_parts(argv, 2) };
    let (mut a, mut b): (c_int, c_int) = (0, 0);
    if unsafe { get_int(env, args[0], &mut a) } == 0
        || unsafe { get_int(env, args[1], &mut b) } == 0
    {
        return unsafe { make_badarg(env) };
    }
    unsafe { make_int(env, a.wrapping_add(b)) }
}

/// mk_tuple() -> {ok, 42}. Exercises make_atom / make_int / make_tuple2.
unsafe extern "C" fn nif_mk_tuple(env: *mut Env, _argc: c_int, _argv: *const Term) -> Term {
    let ok = unsafe { make_atom(env, c"ok".as_ptr()) };
    let n = unsafe { make_int(env, 42) };
    unsafe { make_tuple2(env, ok, n) }
}

/// roundtrip(T) -> T. Passes an arbitrary term straight back.
unsafe extern "C" fn nif_roundtrip(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 1 {
        return unsafe { make_badarg(env) };
    }
    unsafe { *argv }
}

/// mk_atom() -> hello. Exercises make_atom.
unsafe extern "C" fn nif_mk_atom(env: *mut Env, _argc: c_int, _argv: *const Term) -> Term {
    unsafe { make_atom(env, c"hello".as_ptr()) }
}

/// check_atom(T) -> boolean(). Exercises is_atom and the true/false atoms.
unsafe extern "C" fn nif_check_atom(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 1 {
        return unsafe { make_badarg(env) };
    }
    let t = unsafe { *argv };
    let name = if unsafe { is_atom(env, t) } != 0 {
        c"true"
    } else {
        c"false"
    };
    unsafe { make_atom(env, name.as_ptr()) }
}

/// mul64(A, B) -> A * B. Exercises get_int64 / make_int64 with 64-bit values.
unsafe extern "C" fn nif_mul64(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 2 {
        return unsafe { make_badarg(env) };
    }
    let args = unsafe { std::slice::from_raw_parts(argv, 2) };
    let (mut a, mut b): (i64, i64) = (0, 0);
    if unsafe { get_int64(env, args[0], &mut a) } == 0
        || unsafe { get_int64(env, args[1], &mut b) } == 0
    {
        return unsafe { make_badarg(env) };
    }
    unsafe { make_int64(env, a.wrapping_mul(b)) }
}

/// halve(F) -> F / 2.0. Exercises get_double / make_double.
unsafe extern "C" fn nif_halve(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 1 {
        return unsafe { make_badarg(env) };
    }
    let mut f = 0.0_f64;
    if unsafe { get_double(env, *argv, &mut f) } == 0 {
        return unsafe { make_badarg(env) };
    }
    unsafe { make_double(env, f / 2.0) }
}

/// dup_bin(B) -> <<B/binary, B/binary>>. Exercises inspect_binary / alloc_binary
/// / make_binary, and the `Binary` handle (filled via MaybeUninit since its
/// bookkeeping fields are private).
unsafe extern "C" fn nif_dup_bin(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 1 {
        return unsafe { make_badarg(env) };
    }
    let mut inb = MaybeUninit::<Binary>::uninit();
    if unsafe { inspect_binary(env, *argv, inb.as_mut_ptr()) } == 0 {
        return unsafe { make_badarg(env) };
    }
    let inb = unsafe { inb.assume_init() };

    let mut out = MaybeUninit::<Binary>::uninit();
    if unsafe { alloc_binary(inb.size * 2, out.as_mut_ptr()) } == 0 {
        return unsafe { make_badarg(env) };
    }
    let mut out = unsafe { out.assume_init() };
    unsafe {
        std::ptr::copy_nonoverlapping(inb.data, out.data, inb.size);
        std::ptr::copy_nonoverlapping(inb.data, out.data.add(inb.size), inb.size);
    }
    unsafe { make_binary(env, &mut out) }
}

/// mk_map() -> #{1 => 10, 2 => 20}. Exercises make_new_map / make_map_put.
unsafe extern "C" fn nif_mk_map(env: *mut Env, _argc: c_int, _argv: *const Term) -> Term {
    let mut m = unsafe { make_new_map(env) };
    for (k, v) in [(1, 10), (2, 20)] {
        let kt = unsafe { make_int(env, k) };
        let vt = unsafe { make_int(env, v) };
        unsafe { make_map_put(env, m, kt, vt, &mut m) };
    }
    m
}

/// map_get(M, K) -> {ok, V} | error. Exercises is_map / get_map_value.
unsafe extern "C" fn nif_map_get(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 2 {
        return unsafe { make_badarg(env) };
    }
    let args = unsafe { std::slice::from_raw_parts(argv, 2) };
    if unsafe { is_map(env, args[0]) } == 0 {
        return unsafe { make_badarg(env) };
    }
    let mut v: Term = 0;
    if unsafe { get_map_value(env, args[0], args[1], &mut v) } == 0 {
        return unsafe { make_atom(env, c"error".as_ptr()) };
    }
    let ok = unsafe { make_atom(env, c"ok".as_ptr()) };
    unsafe { make_tuple2(env, ok, v) }
}

/// map_size(M) -> N. Exercises get_map_size.
unsafe extern "C" fn nif_map_size(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 1 {
        return unsafe { make_badarg(env) };
    }
    let mut n: usize = 0;
    if unsafe { get_map_size(env, *argv, &mut n) } == 0 {
        return unsafe { make_badarg(env) };
    }
    unsafe { make_int(env, n as c_int) }
}

/// triple() -> [first, second, third]. Exercises make_list_from_array.
unsafe extern "C" fn nif_triple(env: *mut Env, _argc: c_int, _argv: *const Term) -> Term {
    let arr = [
        unsafe { make_atom(env, c"first".as_ptr()) },
        unsafe { make_atom(env, c"second".as_ptr()) },
        unsafe { make_atom(env, c"third".as_ptr()) },
    ];
    unsafe { make_list_from_array(env, arr.as_ptr(), arr.len() as c_uint) }
}

/// len(L) -> N. Exercises get_list_length.
unsafe extern "C" fn nif_len(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 1 {
        return unsafe { make_badarg(env) };
    }
    let mut n: c_uint = 0;
    if unsafe { get_list_length(env, *argv, &mut n) } == 0 {
        return unsafe { make_badarg(env) };
    }
    unsafe { make_int(env, n as c_int) }
}

/// notify() -> ok, having sent the atom `pong` to the calling process.
/// Exercises self_ / send (msg_env NULL: the message lives in the caller env).
unsafe extern "C" fn nif_notify(env: *mut Env, _argc: c_int, _argv: *const Term) -> Term {
    let mut pid = MaybeUninit::<Pid>::uninit();
    unsafe { self_(env, pid.as_mut_ptr()) };
    let pid = unsafe { pid.assume_init() };
    let msg = unsafe { make_atom(env, c"pong".as_ptr()) };
    unsafe { send(env, &pid, std::ptr::null_mut(), msg) };
    unsafe { make_atom(env, c"ok".as_ptr()) }
}

/// minor() -> the NIF minor version this NIF was built against (MINOR_VERSION,
/// which is itself feature-gated). The harness reads this to decide which
/// version-gated functions below it may call against the loaded build.
unsafe extern "C" fn nif_minor(env: *mut Env, _argc: c_int, _argv: *const Term) -> Term {
    unsafe { make_int(env, MINOR_VERSION) }
}

/// new_atom() -> made217. Exercises make_new_atom (NIF 2.17); only registered
/// when built with nif_2_17.
#[cfg(feature = "nif_2_17")]
unsafe extern "C" fn nif_new_atom(env: *mut Env, _argc: c_int, _argv: *const Term) -> Term {
    let mut a: Term = 0;
    if unsafe { make_new_atom(env, c"made217".as_ptr(), &mut a, CharEncoding::Latin1) } == 0 {
        return unsafe { make_badarg(env) };
    }
    a
}

/// tsize(T) -> byte size of T's term storage. Exercises term_size (NIF 2.18);
/// only registered when built with nif_2_18.
#[cfg(feature = "nif_2_18")]
unsafe extern "C" fn nif_tsize(env: *mut Env, argc: c_int, argv: *const Term) -> Term {
    if argc != 1 {
        return unsafe { make_badarg(env) };
    }
    let n = unsafe { term_size(*argv) };
    unsafe { make_int(env, n as c_int) }
}

// ---------------------------------------------------------------------------
// Load callback
// ---------------------------------------------------------------------------

// `nif_init!` resolves the enif_* table before this runs, so there is nothing
// to do here; a real NIF would use this hook for per-load setup.
unsafe extern "C" fn load(_env: *mut Env, _priv_data: *mut *mut c_void, _info: Term) -> c_int {
    0
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

// Generates the platform-correct `nif_init` (no-arg on Unix, a callbacks
// pointer on Windows), resolves the table, then calls `build_entry` — so this
// file is identical on every platform.
enif_ffi::nif_init!(build_entry);

fn build_entry() -> *const Entry {
    let mut funcs = vec![
        Func {
            name: c"add".as_ptr(),
            arity: 2,
            fptr: nif_add,
            flags: 0,
        },
        Func {
            name: c"mk_tuple".as_ptr(),
            arity: 0,
            fptr: nif_mk_tuple,
            flags: 0,
        },
        Func {
            name: c"roundtrip".as_ptr(),
            arity: 1,
            fptr: nif_roundtrip,
            flags: 0,
        },
        Func {
            name: c"mk_atom".as_ptr(),
            arity: 0,
            fptr: nif_mk_atom,
            flags: 0,
        },
        Func {
            name: c"check_atom".as_ptr(),
            arity: 1,
            fptr: nif_check_atom,
            flags: 0,
        },
        Func {
            name: c"mul64".as_ptr(),
            arity: 2,
            fptr: nif_mul64,
            flags: 0,
        },
        Func {
            name: c"halve".as_ptr(),
            arity: 1,
            fptr: nif_halve,
            flags: 0,
        },
        Func {
            name: c"dup_bin".as_ptr(),
            arity: 1,
            fptr: nif_dup_bin,
            flags: 0,
        },
        Func {
            name: c"mk_map".as_ptr(),
            arity: 0,
            fptr: nif_mk_map,
            flags: 0,
        },
        Func {
            name: c"map_get".as_ptr(),
            arity: 2,
            fptr: nif_map_get,
            flags: 0,
        },
        Func {
            name: c"map_size".as_ptr(),
            arity: 1,
            fptr: nif_map_size,
            flags: 0,
        },
        Func {
            name: c"triple".as_ptr(),
            arity: 0,
            fptr: nif_triple,
            flags: 0,
        },
        Func {
            name: c"len".as_ptr(),
            arity: 1,
            fptr: nif_len,
            flags: 0,
        },
        Func {
            name: c"notify".as_ptr(),
            arity: 0,
            fptr: nif_notify,
            flags: 0,
        },
        Func {
            name: c"minor".as_ptr(),
            arity: 0,
            fptr: nif_minor,
            flags: 0,
        },
    ];
    // Version-gated functions are registered only when built at their rung; the
    // harness gates its calls on minor() so it never invokes an absent stub.
    #[cfg(feature = "nif_2_17")]
    funcs.push(Func {
        name: c"new_atom".as_ptr(),
        arity: 0,
        fptr: nif_new_atom,
        flags: 0,
    });
    #[cfg(feature = "nif_2_18")]
    funcs.push(Func {
        name: c"tsize".as_ptr(),
        arity: 1,
        fptr: nif_tsize,
        flags: 0,
    });
    let funcs = funcs.leak();
    let num = funcs.len() as c_int;
    let funcs_ptr = funcs.as_mut_ptr();

    let entry = Box::leak(Box::new(Entry {
        major: MAJOR_VERSION,
        minor: MINOR_VERSION,
        name: c"smoke".as_ptr(),
        num_of_funcs: num,
        funcs: funcs_ptr,
        load: Some(load),
        reload: None,
        upgrade: None,
        unload: None,
        vm_variant: VM_VARIANT.as_ptr(),
        options: 1,
        sizeof_resource_type_init: std::mem::size_of::<ResourceTypeInit>(),
        min_erts: MIN_ERTS_VERSION.as_ptr(),
    }));
    entry as *const Entry
}
