#![feature(core)]
#![feature(raw)]
#![feature(unsized_types)]

mod mat;
mod row;
mod strided;

use std::mem;

macro_rules! show {
    ($e:expr) => {
        println!("> {}", stringify!($e));
        println!("{:?}\n", $e);
    }
}

fn main() {
    let array;

    println!("// `array: [i32; 15]`");
    show!(array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4]);

    let m;
    println!("// `m: &'array Mat<i32>`");
    show!(m = Mat::reshape(&array, (3, 5)));

    println!("// `m` is a 3-by-5 matrix (3 rows, 5 columns)");
    show!(m);

    println!("// `m` is a fat pointer");
    show!(mem::size_of_val(&m));

    println!("// In memory-representation of `m`");
    println!("// `m.repr(): FatPtr<T, mat::Info>`");
    show!(m.repr());

    println!("// Element at the intersection of the second row and the third column");
    show!(m[(1, 2)]);

    println!("// First row");
    println!("// `&m[0]: &'array Row<i32>`");
    show!(&m[0]);

    println!("// Third element of the first row");
    show!(m[0][3]);

    println!("// Third column");
    println!("// `&m[(.., 2)]: &'array strided::Col<i32>`");
    show!(&m[(.., 2)]);

    let sm;
    println!("// Submatrix from 2nd to 3rd row and from 2nd to 4th column");
    println!("// `sm: &'array strided::Mat<i32>`");
    show!(sm = &m[(1..3, 1..4)]);

    show!(sm);

    println!("// `sm` is another type of fat pointer");
    show!(mem::size_of_val(&sm));

    println!("// In memory-representation of `sm`");
    println!("// `sm.repr(): FatPtr<T, strided::mat::Info>`");
    show!(sm.repr());
}

/// A dense matrix stored in contiguous memory
unsized type Mat<T>;

/// A view into the row of a matrix
#[derive(Debug)]
pub struct Row<T>([T]);
