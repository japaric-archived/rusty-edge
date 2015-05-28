# `dense`

This crate implements a dense matrix (stored in [row-major order]) with Python/NumPy-like slicing
sugar using the `Index` trait and the range syntax (`a..b`).

[row-major order]: https://en.wikipedia.org/wiki/Row-major_order

(For convenience the output looks like a REPL, but it's just a normal Rust program)

[Source code](/unsized_types/dense/src/main.rs)

Here's the output of `cargo run`

``` rust
// `array: [i32; 15]`
> array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4]
()

// `m: &'array Mat<i32>`
> m = Mat::reshape(&array, (3, 5))
()

// `m` is a 3-by-5 matrix (3 rows, 5 columns)
> m
[0, 1, 2, 3, 4]
[5, 6, 7, 8, 9]
[8, 7, 6, 5, 4]

// `m` is a fat pointer
> mem::size_of_val(&m)
24

// In memory-representation of `m`
// `m.repr(): raw::Mat<i32>`
> m.repr()
Mat { data: 0x7ffdcdac97dc, ncols: 5, nrows: 3 }

// Element at the intersection of the second row and the third column
> m[(1, 2)]
7

// First row
// `&m[0]: &'array Row<i32>`
> &m[0]
Row([0, 1, 2, 3, 4])

// Third element of the first row
> m[0][3]
3

// Third column
// `&m[(.., 2)]: &'array Col<i32>`
> &m[(.., 2)]
Col([2, 7, 6])

// Submatrix from 2nd to 3rd row and from 2nd to 4th column
// `sm: &'array strided::Mat<i32>`
> sm = &m[(1..3, 1..4)]
()

> sm
[6, 7, 8]
[7, 6, 5]

// `sm` is another type of fat pointer
> mem::size_of_val(&sm)
32

// In memory-representation of `sm`
// `sm.repr(): strided::raw::Mat<i32>`
> sm.repr()
Mat { data: 0x7ffdcdac97f4, ncols: 3, nrows: 2, stride: 5 }
```

This is how the same operations look like in Python/NumPy for comparison:

``` python
>>> import numpy

>>> m = numpy.array([[0, 1, 2, 3, 4], [5, 6, 7, 8, 9], [8, 7, 6, 5, 4]])

>>> m
array([[0, 1, 2, 3, 4],
       [5, 6, 7, 8, 9],
       [8, 7, 6, 5, 4]])

>>> m[1, 2]
7

>>> m[0]
array([0, 1, 2, 3, 4])

>>> m[0][3]
3

>>> m[:, 2]
array([2, 7, 6])

>>> m[1:3, 1:4]
array([[6, 7, 8],
       [7, 6, 5]])
```
