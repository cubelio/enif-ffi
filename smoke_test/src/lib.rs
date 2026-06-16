//! A minimal NIF written directly against `enif-ffi`, used to prove the raw
//! binding actually loads and runs inside the BEAM (cargo only type-checks it).
//!
//! This is also a worked example of what a consumer must hand-write without a
//! codegen layer: the `nif_init` entry, the `ErlNifEntry`, the function table,
//! and a `load` callback that calls [`enif_ffi::init`].

use std::ffi::{c_int, c_void};

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

// ---------------------------------------------------------------------------
// Load callback — resolves the enif_* table
// ---------------------------------------------------------------------------

unsafe extern "C" fn load(_env: *mut Env, _priv_data: *mut *mut c_void, _info: Term) -> c_int {
    match unsafe { enif_ffi::init() } {
        Ok(()) => 0,
        Err(_) => 1, // non-zero => fail the load, BEAM stays up
    }
}

// ---------------------------------------------------------------------------
// Entry point — the symbol the BEAM dlsym's
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn nif_init() -> *const Entry {
    let funcs = Box::leak(Box::new([
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
    ]));
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
        sizeof_resource_type_init: size_of::<ResourceTypeInit>(),
        min_erts: MIN_ERTS_VERSION.as_ptr(),
    }));
    entry as *const Entry
}
