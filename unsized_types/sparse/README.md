# `sparse`

This crate implements a sparse matrix using the Compressed Row Storage ([CRS]) format. The owned
version of the sparse matrix (`Box<Mat>`) owns three slices which are freed when its dropped.

[CRS]: http://netlib.org/linalg/html_templates/node91.html

(For convenience the output looks like a REPL, but it's just a normal Rust program)

[Source code](/unsized_types/sparse/src/main.rs)

Here's the output of `cargo run` under valgrind

``` rust
> data = Box::new([1, 2, 3, 4, 5, 6, 7, 8])
()

> col_ind = Box::new([0, 1, 1, 3, 2, 3, 4, 5])
()

> row_ptr = Box::new([0, 2, 4, 7, 8])
()

> ncols = 6
()

// Create an owned sparse matrix (stored in CRS format)
// `m: Box<Mat<i32>>`
> m = Mat::new(data, col_ind, row_ptr, ncols)
()

// 4-by-6 matrix
> m
[1, 2, 0, 0, 0, 0]
[0, 3, 0, 4, 0, 0]
[0, 0, 5, 6, 7, 0]
[0, 0, 0, 0, 0, 8]

// `Box<Mat<i32>>` is a fat pointer
> mem::size_of_val(&m)
40

// In-memory representation of `Box<Mat<i32>>`
> m.repr()
Mat { data: 0x6826020, col_ind: 0x6820080, row_ptr: 0x681d060, ncols: 6, nrows: 4 }

// Element at the intersection of the second row and the fourth column
> m[(1, 3)]
4

// Third row
// `&m[2]: &'m Row<i32>`
> &m[2]
Row([0, 0, 5, 6, 7, 0])

// Second element of the first row
> m[0][1]
2

// Submatrix from 2nd row to 3rd row (row slicing)
// `&m[1..3]: &'m Mat<i32>`
> &m[1..3]
[0, 3, 0, 4, 0, 0]
[0, 0, 5, 6, 7, 0]

> drop(m)
freeing `data`
freeing `col_ind`
freeing `row_ptr`
()

==13590==
==13590== HEAP SUMMARY:
==13590==     in use at exit: 0 bytes in 0 blocks
==13590==   total heap usage: 27 allocs, 27 frees, 2,856 bytes allocated
==13590==
==13590== All heap blocks were freed -- no leaks are possible
```
