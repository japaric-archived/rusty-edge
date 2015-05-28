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
> slice = (&array).coerce_ref()
()

> slice
[0, 1, 2, 3]

// `slice` is a fat pointer
> mem::size_of_val(&slice)
16

// In-memory representation of `slice`
// `slice.repr(): raw::Slice<i32>`
> slice.repr()
Slice { data: 0xfff000428, len: 4 }

// Element indexing
> slice[3]
3

// Slicing
> &slice[1..3]
[1, 2]

// Equivalent to `let boxed: Box<[i32]> = Box::new([4, 5, 6, 7])`
// `boxed: `Box<Slice<i32>>
> boxed = Box::new([4, 5, 6, 7]).coerce_box()
()

> drop(boxed)
deallocating 16 bytes starting at 0x682b000
()

==27071==
==27071== HEAP SUMMARY:
==27071==     in use at exit: 0 bytes in 0 blocks
==27071==   total heap usage: 25 allocs, 25 frees, 2,728 bytes allocated
==27071==
==27071== All heap blocks were freed -- no leaks are possible
```
