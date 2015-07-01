#![allow(warnings)]

#![feature(box_raw)]
#![feature(core)]
#![feature(filling_drop)]
#![feature(raw)]
#![feature(unsized_types)]

mod mat;
mod row;
mod vector;

use std::mem;

macro_rules! show {
    ($e:expr) => {
        println!("> {}", stringify!($e));
        println!("{:?}\n", $e);
    }
}

fn main() {
    let data;
    show!(data = Box::new([1, 2, 3, 4, 5, 6, 7, 8]));

    let col_ind;
    show!(col_ind = Box::new([0, 1, 1, 3, 2, 3, 4, 5]));

    let row_ptr;
    show!(row_ptr = Box::new([0, 2, 4, 7, 8]));

    let ncols;
    show!(ncols = 6);

    println!("// Create an owned sparse matrix (stored in CRS format)");
    println!("// `m: Box<Mat<i32>>`");
    let m;
    show!(m = Mat::new(data, col_ind, row_ptr, ncols));

    println!("// 4-by-6 matrix");
    show!(m);

    println!("// `Box<Mat<i32>>` is a fat pointer");
    show!(mem::size_of_val(&m));

    println!("// In-memory representation of `Box<Mat<i32>>`");
    println!("// `m.repr(): FatPtr<i32, mat::Info>`");
    show!(m.repr());

    println!("// Element at the intersection of the second row and the fourth column");
    show!(m[(1, 3)]);

    println!("// Third row");
    println!("// `&m[2]: &'m Row<i32>`");
    show!(&m[2]);

    println!("// Second element of the first row");
    show!(m[0][1]);

    println!("// Submatrix from 2nd row to 3rd row (row slicing)");
    println!("// `&m[1..3]: &'m Mat<i32>`");
    show!(&m[1..3]);

    show!(mem::drop(m));
}

/// Sparse matrix stored in Compressed Row Storage (CRS) format
unsized type Mat<T>;

/// Sparse row vector
#[derive(Debug)]
struct Row<T>(Vector<T>);

/// Sparse vector
unsized type Vector<T>;
