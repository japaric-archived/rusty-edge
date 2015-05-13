# `unsized_structs`

I added an `unsized struct` feature to ([my fork] of) `rustc` that lets users create their own
unsized types (from scratch, not just newtypes over `[T]` or `str`), and then used that feature
and the `Index` trait to add Python-like slicing syntax to a matrix data structure:

[my fork]: https://github.com/japaric/rust/commits/unsized

[Source code](/unsized_structs/demo.rs).
[Travis output](https://travis-ci.org/japaric/rusty-edge)

(The output looks like a REPL, but it's just a normal Rust program)

``` rust
// `array: [i32; 15]`
> array
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4]

// `m: &'array Mat<i32>`
> m = Mat::reshape(&array, (3, 5))
()

// 3-by-5 matrix (3 rows, 5 columns)
> m
[0, 1, 2, 3, 4]
[5, 6, 7, 8, 9]
[8, 7, 6, 5, 4]

// Element at the intersection of the second row and the third column
> m[(1, 2)]
7

// Second row
// `&m[(1, ..)]: &'array Row<i32>`
> &m[(1, ..)]
Row([5, 6, 7, 8, 9])

// Third column
// `&m[(.., 2)]: &'array Col<i32>`
> &m[(.., 2)]
Col([2, 7, 6])

// First row (alternative syntax)
> &m[0]
Row([0, 1, 2, 3, 4])

// Third element of the first row
> m[0][2]
2

// Submatrix from 2nd to 3rd row and from 2nd to 4th column
> &m[(1..3, 1..4)]
[4, 5, 6]
[7, 8, 9]
```

This is how the same thing looks like in Python:

``` python
>>> import numpy

>>> A = numpy.array([[0, 1, 2, 3, 4], [5, 6, 7, 8, 9], [8, 7, 6, 5, 4]])

>>> A
array([[0, 1, 2, 3, 4],
       [5, 6, 7, 8, 9],
       [8, 7, 6, 5, 4]])

>>> A[1, 2]
7

>>> A[1, :]
array([5, 6, 7, 8, 9])

>>> A[:, 2]
array([2, 7, 6])

>>> A[0]
array([0, 1, 2, 3, 4])

>>> A[0][2]
2

>>> A[1:3, 1:4]
array([[6, 7, 8],
       [7, 6, 5]])
```

---

This is one of my planned [compiler changes] aimed at improving the ergonomics of doing
numerical/scientific computing with Rust.

[compiler changes]: https://github.com/japaric/linalg.rs#improving-operator-sugar

For more details about the feature check my [WIP RFC].

[WIP RFC]: https://github.com/japaric/rfcs/blob/unsized/text/0000-unsized-structs.md

If you want to try out the feature, you can use one of my unstested (as in not `make check`ed)
stage1 tarballs (Linux x86_64 only, see [top README]), otherwise you'll have to compile
[this branch] by yourself.

[top README](/README.md)
[this branch]: https://github.com/japaric/rust/commits/unsized
