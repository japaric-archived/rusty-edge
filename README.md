# `rusty-edge`

Rust nightly channel + bleeding edge features

## Features

- [`unsized_structs`][/unsized_structs]: Define your own unsized structs (like `[T]`, `str`).
  Increases the usability of the `Index`/`Deref` traits, and lets you harness the power of
  re-borrow semantics.

[Other planned features](https://github.com/japaric/linalg.rs#improving-operator-sugar)

## [Tarballs]

[Tarballs]: https://www.dropbox.com/sh/hz03qag74f3p6ol/AADVTj8mTTMk-phlj0ZqiiQna?dl=0

**Untested** debug-enabled stage1 tarballs for x86_64-unknown-linux-gnu. Use at your own risk! Expect
ICEs and no support.

## Demos

Each feature contains a demo that showcases the feature. If you don't want to mess with untested
tarballs, but are interested in knowing more about the features, you can look at [Travis' output],
and read the [source code][/unsized_structs/demo.rs] of each demo.

[Travis' output]:

## License

All the scripts, code snippets in this repository are licensed under the MIT license.

See LICENSE-MIT for more details.
