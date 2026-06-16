#!/usr/bin/env bash
# Build the smoke NIF, load it into a real BEAM, and run the assertions.
# Extra args are forwarded to cargo (e.g. --features ... via -F to enif-ffi is
# not wired here; the floor surface is enough for the smoke test).
set -euo pipefail

ERL_BIN="${ERL_BIN:-/opt/erlang/27/bin}"
DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$DIR"

echo "== building cdylib =="
cargo build "$@"

SO="$(find target -name 'libsmoke.so' -print -quit)"
[ -n "$SO" ] || { echo "libsmoke.so not found" >&2; exit 1; }
cp "$SO" ./libsmoke.so

echo "== compiling erlang harness =="
"$ERL_BIN/erlc" smoke.erl

echo "== running in BEAM ($("$ERL_BIN/erl" -noshell -eval 'io:format("~s/OTP-~s",[erlang:system_info(nif_version),erlang:system_info(otp_release)]), halt()')) =="
"$ERL_BIN/erl" -noshell -pa . -eval \
  'case catch smoke:test() of
       ok -> io:format("SMOKE OK~n"), halt(0);
       E  -> io:format("SMOKE FAIL: ~p~n", [E]), halt(1)
   end'
