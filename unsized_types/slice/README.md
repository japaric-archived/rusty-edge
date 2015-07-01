# `slice`

This crate re-implements the built-in `[T]` type without relying on compiler magic for the
destructor. Unsize coercions are implemented in a non-generic way because a generic implementation
requires type level integers: `impl<T, usize N> Unsize<Slice<T>> for [T; N] {}`.

(For convenience the output looks like a REPL, but it's just a normal Rust program)

[Source code](/unsized_types/sparse/src/main.rs)

Here's the output of `cargo run` under valgrind

``` rust
> array = [0, 1, 2, 3]
()

// Equivalent to `let slice: &[i32] = &array`
// `slice: &'array Slice<i32>`
> slice = coerce_ref(&array)
()

> slice
[0, 1, 2, 3]

// `slice` is a fat pointer
> mem::size_of_val(&slice)
16

// In-memory representation of `slice`
// `slice.repr(): FatPtr<i32, usize>`
> slice.repr()
FatPtr { data: 0xfff000208, info: 4 }

// Element indexing
> slice[3]
3

// Slicing
> &slice[1..3]
[1, 2]

// Equivalent to `let boxed: Box<[i32]> = Box::new([..])`
// `boxed: `Box<Slice<Box<i32>>>
> boxed =
    coerce_box(Box::new([Box::new(4), Box::new(5), Box::new(6), Box::new(7)]))
()

> drop(boxed)
dropping contents of `Slice`
()

==25013==
==25013== HEAP SUMMARY:
==25013==     in use at exit: 0 bytes in 0 blocks
==25013==   total heap usage: 29 allocs, 29 frees, 2,776 bytes allocated
```
