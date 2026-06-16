-module(smoke).
-export([add/2, mk_tuple/0, roundtrip/1, mk_atom/0, check_atom/1, test/0]).
-on_load(init/0).

init() ->
    erlang:load_nif("./libsmoke", 0).

%% NIF stubs — replaced on load. Reaching these means the NIF failed to load.
add(_, _) -> erlang:nif_error(not_loaded).
mk_tuple() -> erlang:nif_error(not_loaded).
roundtrip(_) -> erlang:nif_error(not_loaded).
mk_atom() -> erlang:nif_error(not_loaded).
check_atom(_) -> erlang:nif_error(not_loaded).

test() ->
    7 = add(3, 4),
    0 = add(-5, 5),
    {ok, 42} = mk_tuple(),
    foo = roundtrip(foo),
    [1, 2, 3] = roundtrip([1, 2, 3]),
    <<"bin">> = roundtrip(<<"bin">>),
    hello = mk_atom(),
    true = check_atom(abc),
    false = check_atom(123),
    ok.
