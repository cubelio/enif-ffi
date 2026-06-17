-module(smoke).
%% map_get/2 and map_size/1 are auto-imported BIFs; our NIF stubs shadow them
%% deliberately, so suppress the name-clash warnings.
-compile({no_auto_import, [map_get/2, map_size/1]}).
-export([add/2, mk_tuple/0, roundtrip/1, mk_atom/0, check_atom/1,
         mul64/2, halve/1, dup_bin/1, mk_map/0, map_get/2, map_size/1,
         triple/0, len/1, notify/0, minor/0, new_atom/0, tsize/1, test/0]).
-on_load(init/0).

init() ->
    erlang:load_nif("./smoke_nif", 0).

%% NIF stubs — replaced on load. Reaching these means the NIF failed to load.
add(_, _) -> erlang:nif_error(not_loaded).
mk_tuple() -> erlang:nif_error(not_loaded).
roundtrip(_) -> erlang:nif_error(not_loaded).
mk_atom() -> erlang:nif_error(not_loaded).
check_atom(_) -> erlang:nif_error(not_loaded).
mul64(_, _) -> erlang:nif_error(not_loaded).
halve(_) -> erlang:nif_error(not_loaded).
dup_bin(_) -> erlang:nif_error(not_loaded).
mk_map() -> erlang:nif_error(not_loaded).
map_get(_, _) -> erlang:nif_error(not_loaded).
map_size(_) -> erlang:nif_error(not_loaded).
triple() -> erlang:nif_error(not_loaded).
len(_) -> erlang:nif_error(not_loaded).
notify() -> erlang:nif_error(not_loaded).
minor() -> erlang:nif_error(not_loaded).
new_atom() -> erlang:nif_error(not_loaded).
tsize(_) -> erlang:nif_error(not_loaded).

test() ->
    %% scalars and terms
    7 = add(3, 4),
    0 = add(-5, 5),
    {ok, 42} = mk_tuple(),
    foo = roundtrip(foo),
    [1, 2, 3] = roundtrip([1, 2, 3]),
    <<"bin">> = roundtrip(<<"bin">>),
    hello = mk_atom(),
    true = check_atom(abc),
    false = check_atom(123),
    %% 64-bit integers and doubles
    1000000000000 = mul64(1000000, 1000000),
    1.5 = halve(3.0),
    %% binaries
    <<"abcabc">> = dup_bin(<<"abc">>),
    <<>> = dup_bin(<<>>),
    %% maps
    M = mk_map(),
    true = is_map(M),
    2 = map_size(M),
    {ok, 10} = map_get(M, 1),
    {ok, 20} = map_get(M, 2),
    error = map_get(M, 3),
    %% lists
    [first, second, third] = triple(),
    3 = len([a, b, c]),
    0 = len([]),
    %% send to self
    ok = notify(),
    receive
        pong -> ok
    after 1000 -> erlang:error(no_pong)
    end,
    %% version-gated functions: only call those the loaded build registered,
    %% which minor() (the compiled NIF minor version) tells us.
    Minor = minor(),
    if
        Minor >= 17 -> made217 = new_atom();
        true -> ok
    end,
    if
        Minor >= 18 ->
            S = tsize({a, b, c}),
            true = is_integer(S) andalso S > 0;
        true -> ok
    end,
    ok.
