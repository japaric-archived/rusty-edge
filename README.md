[![Build Status][status]](https://travis-ci.org/japaric/rusty-edge)

[status]: https://travis-ci.org/japaric/rusty-edge.svg?branch=master

# `rusty-edge`

Rust nightly channel + **backward compatible** bleeding edge features

## Features

- ~~[`unsized_structs`](/unsized_structs)~~ Superseded by [`unsized_types`](/unsized_types): Define
  your own unsized types (like `[T]`, `str`). Increases the usability of the `Index`/`Deref`
  traits, and lets you harness the power of re-borrow semantics. (RFC pending)

- Multi argument indexing: `A[i, j]` works and it's just sugar over `A[(i, j)]` (RFC pending)

- Overloaded augmented assignments: e.g. use the `AddAssign` trait to overload the expression
  `a += b`. [RFC](https://github.com/rust-lang/rfcs/pull/953)

- Overloaded indexed assignments: Use the `IndexAssign` trait to overload the expression
  `a[b] = c`. [RFC](https://github.com/rust-lang/rfcs/pull/1129)

## [Tarballs]

[Tarballs]: https://www.dropbox.com/sh/hz03qag74f3p6ol/AADVTj8mTTMk-phlj0ZqiiQna?dl=0

**Untested** (not `make check` ed) tarballs for x86_64-unknown-linux-gnu. Use at your own risk! Expect
ICEs.

## Demos

Some features contain a demo that showcases their use. If you don't want to mess with untested
tarballs, but are interested in knowing more about the features, you can look at [Travis' output],
and read the [source code](/unsized_structs/demo.rs) of each demo.

[Travis' output]: https://travis-ci.org/japaric/rusty-edge

## License

All the scripts, code snippets in this repository are licensed under the MIT license.

See LICENSE-MIT for more details.
