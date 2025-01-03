# Rust-based Tic-Tac-Toe Library (lib-tic-tac-toe)

![Rust](https://github.com/ptdecker/lib-tic-tac-toe/workflows/Rust/badge.svg)

This is a Rust-based Tic-Tac-Toe library meant to be incorporated into various
Tic-Tac-Toe game implementations.

## no_std Compatibility

This library is designed to be compatible with no_std environments. It does not
require the Rust standard library (std) and depends only on the core allocation
(alloc) library for memory management tasks. This makes it suitable for use in
bare-metal, embedded, or other high-performance or resource-constrained
environments.

`RUST_LOG=debug cargo run` for prints

To start
`cargo run`

To start a game
`curl -X POST http://localhost:50051/game/start`

To get all game ids:
`curl -X GET http://localhost:50051/game/ids`

To get the status of a game
`curl -v -X GET http://localhost:50051/game/state/bc816ffe-9561-4c0c-89ae-c4bd04175c8c`

To make a move

```shell
curl -X POST http://localhost:50051/game/f5d7a789-1931-428b-92d2-9c6c4fa4c177/move \
 -H "Content-Type: application/json" \
 -d '{"space": 1}'
```
