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
