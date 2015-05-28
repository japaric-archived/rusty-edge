#![feature(filling_drop)]
#![feature(unsized_types)]

use std::ops::{Index, Range};
use std::{fmt, mem, slice};

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

    show!(drop(m));
}

/// In-memory representations
mod raw {
    #[allow(raw_pointer_derive)]
    #[derive(Debug)]
    pub struct Mat<T> {
        pub data: *mut T,
        pub col_ind: *const usize,
        pub row_ptr: *const usize,
        pub ncols: usize,
        pub nrows: usize,
    }

    pub struct Slice<T> {
        pub data: *mut T,
        pub indices: *const [usize],
        pub len: usize,
    }
}

// Sparse matrix stored in Compressed Row Storage (CRS) format
unsized type Mat<T> = raw::Mat<T>;

// Sparse row
unsized type Row<T> = raw::Slice<T>;

impl<T> Mat<T> {
    fn new<'a>(
        mut data: Box<[T]>,
        col_ind: Box<[usize]>,
        row_ptr: Box<[usize]>,
        ncols: usize,
    ) -> Box<Mat<T>> {
        unsafe {
            assert_eq!(Some(&0), row_ptr.first());
            assert_eq!(Some(&data.len()), row_ptr.last());
            assert_eq!(data.len(), col_ind.len());

            let nrows = row_ptr.len().checked_sub(1).unwrap();
            let data_ = data.as_mut_ptr();
            mem::forget(data);
            let col_ind_ = col_ind.as_ptr();
            mem::forget(col_ind);
            let row_ptr_ = row_ptr.as_ptr();
            mem::forget(row_ptr);

            mem::transmute(raw::Mat {
                data: data_, col_ind: col_ind_, row_ptr: row_ptr_, nrows: nrows, ncols: ncols,
            })
        }
    }

    fn index_raw(&self, (i, j): (usize, usize)) -> Option<*mut T> {
        unsafe {
            let raw::Mat { data, col_ind, row_ptr, nrows, ncols } = self.repr();

            assert!(i < nrows);
            assert!(j < ncols);

            for k in *row_ptr.offset(i as isize)..*row_ptr.offset(i as isize + 1) {
                let k = k as isize;

                if j == *col_ind.offset(k) {
                    return Some(data.offset(k))
                }
            }

            None
        }
    }

    fn repr(&self) -> raw::Mat<T> {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<T> fmt::Debug for Mat<T> where T: fmt::Debug + Zero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let raw::Mat { data, col_ind, row_ptr, nrows, ncols } = self.repr();

            for i in 0..nrows {
                if i != 0 {
                    try!(f.write_str("\n"))
                }

                try!(f.write_str("["));

                let _0 = T::zero();
                let mut k = *row_ptr.offset(i as isize) as isize;
                let mut nnz = *row_ptr.offset(i as isize + 1) as isize - k;
                for j in 0..ncols {
                    if j != 0 {
                        try!(f.write_str(", "))
                    }

                    if nnz != 0 && j == *col_ind.offset(k) {
                        try!(write!(f, "{:?}", *data.offset(k)));
                        k += 1;
                        nnz -= 1;
                    } else {
                        try!(write!(f, "{:?}", _0));
                    }
                }

                try!(f.write_str("]"));
            }

            Ok(())
        }
    }
}

impl<T> Drop for Mat<T> {
    fn drop(&mut self) {
        unsafe {
            let raw::Mat { data, col_ind, row_ptr, nrows, .. } = self.repr();

            if !data.is_null() && data as usize != mem::POST_DROP_USIZE {
                let nnz = *row_ptr.offset(nrows as isize);

                println!("freeing `data`");
                mem::drop(Vec::from_raw_parts(data, nnz, nnz));
                println!("freeing `col_ind`");
                mem::drop(Vec::from_raw_parts(col_ind as *mut usize, nnz, nnz));
                println!("freeing `row_ptr`");
                mem::drop(Vec::from_raw_parts(row_ptr as *mut usize, nrows + 1, nrows + 1));
            }
        }
    }
}

/// Element indexing
impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        unsafe {
            &*self.index_raw((i, j)).unwrap()
        }
    }
}

/// Row indexing
impl<T> Index<usize> for Mat<T> {
    type Output = Row<T>;

    fn index(&self, i: usize) -> &Row<T> {
        unsafe {
            let raw::Mat { data, col_ind, row_ptr, ncols, nrows } = self.repr();

            assert!(i < nrows);

            let i = i as isize;
            let offset = *row_ptr.offset(i);
            let nnz = *row_ptr.offset(i + 1) - offset;
            let offset = offset as isize;
            let data = data.offset(offset);
            let indices = slice::from_raw_parts(col_ind.offset(offset), nnz);

            mem::transmute(raw::Slice { data: data, indices: indices, len: ncols })
        }
    }
}

/// Row slicing
impl<T> Index<Range<usize>> for Mat<T> {
    type Output = Mat<T>;

    fn index(&self, r: Range<usize>) -> &Mat<T> {
        unsafe {
            assert!(r.start <= r.end);

            let raw::Mat { data, col_ind, row_ptr, ncols, nrows } = self.repr();

            assert!(r.end <= nrows);

            let row_ptr = row_ptr.offset(r.start as isize);
            let nrows = r.end - r.start;

            mem::transmute(raw::Mat {
                data: data, col_ind: col_ind, row_ptr: row_ptr, ncols: ncols, nrows: nrows
            })
        }
    }
}

impl<T> Row<T> {
    fn repr(&self) -> raw::Slice<T> {
        unsafe {
            mem::transmute(self)
        }
    }

    unsafe fn index_raw(&self, i: usize) -> Option<*mut T> {
        let raw::Slice { data, indices, len } = self.repr();

        assert!(i < len);

        if (*indices).contains(&i) {
            Some(data.offset(i as isize))
        } else {
            None
        }
    }
}

impl<T> fmt::Debug for Row<T> where T: fmt::Debug + Zero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let raw::Slice { data, indices, len } = self.repr();

            try!(f.write_str("Row(["));

            let _0 = T::zero();
            let mut k = 0;
            let mut nnz = (*indices).len();
            let indices = (*indices).as_ptr();
            for i in 0..len {
                if i != 0 {
                    try!(f.write_str(", "))
                }

                if nnz != 0 && i == *indices.offset(k) {
                    try!(write!(f, "{:?}", *data.offset(k)));
                    k += 1;
                    nnz -= 1;
                } else {
                    try!(write!(f, "{:?}", _0));
                }
            }

            f.write_str("])")
        }
    }
}

/// Element indexing
impl<T> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        unsafe {
            &*self.index_raw(i).unwrap()
        }
    }
}

trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> i32 {
        0
    }
}

fn drop<T>(_: T) {}
