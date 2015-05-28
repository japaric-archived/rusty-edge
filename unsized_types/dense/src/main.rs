#![feature(unsized_types)]

use std::ops::{Deref, Index};
use std::{fmt, mem};

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
    println!("// `m.repr(): raw::Mat<i32>`");
    show!(m.repr());

    println!("// Element at the intersection of the second row and the third column");
    show!(m[(1, 2)]);

    println!("// First row");
    println!("// `&m[0]: &'array Row<i32>`");
    show!(&m[0]);

    println!("// Third element of the first row");
    show!(m[0][3]);

    println!("// Third column");
    println!("// `&m[(.., 2)]: &'array Col<i32>`");
    show!(&m[(.., 2)]);

    let sm;
    println!("// Submatrix from 2nd to 3rd row and from 2nd to 4th column");
    println!("// `sm: &'array strided::Mat<i32>`");
    show!(sm = &m[(1..3, 1..4)]);

    show!(sm);

    println!("// `sm` is another type of fat pointer");
    show!(mem::size_of_val(&sm));

    println!("// In memory-representation of `sm`");
    println!("// `sm.repr(): strided::raw::Mat<i32>`");
    show!(sm.repr());
}

mod raw {
    #[allow(raw_pointer_derive)]
    #[derive(Debug)]
    pub struct Mat<T> {
        pub data: *mut T,
        pub ncols: usize,
        pub nrows: usize,
    }
}

mod strided {
    use std::ops::{Index, Range, RangeFull};
    use std::{fmt, mem, slice};

    use Row;

    pub mod raw {
        #[allow(raw_pointer_derive)]
        #[derive(Debug)]
        pub struct Mat<T> {
            pub data: *mut T,
            pub ncols: usize,
            pub nrows: usize,
            pub stride: usize,
        }

        pub struct Slice<T> {
            pub data: *mut T,
            pub len: usize,
            pub stride: usize,
        }
    }

    // A view into the column of a matrix
    pub unsized type Col<T> = raw::Slice<T>;

    // A strided matrix
    pub unsized type Mat<T> = raw::Mat<T>;

    impl<T> Col<T> {
        fn repr(&self) -> raw::Slice<T> {
            unsafe {
                mem::transmute(self)
            }
        }
    }

    impl<T> fmt::Debug for Col<T> where T: fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            unsafe {
                let raw::Slice { data, len, stride } = self.repr();

                try!(f.write_str("Col(["));

                // NB(japaric) this should use iterators, but I don't want to make example bigger
                for i in 0..len {
                    if i != 0 {
                        try!(f.write_str(", "));
                    }

                    try!(write!(f, "{:?}", *data.offset((i * stride) as isize)))
                }

                f.write_str("])")
            }
        }
    }

    impl<T> Mat<T> {
        pub fn repr(&self) -> raw::Mat<T> {
            unsafe {
                mem::transmute(self)
            }
        }
    }

    impl<T> fmt::Debug for Mat<T> where T: fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            unsafe {
                let raw::Mat { data, nrows, ncols, stride } = self.repr();

                // NB(japaric) this should use iterators, but I don't want to make example bigger
                for i in 0..nrows {
                    if i != 0 {
                        try!(f.write_str("\n"));
                    }

                    let row = slice::from_raw_parts(data.offset((i * stride) as isize), ncols);

                    try!(write!(f, "{:?}", row))
                }

                Ok(())
            }
        }
    }

    impl<T> Index<(Range<usize>, Range<usize>)> for Mat<T> {
        type Output = Mat<T>;

        fn index(&self, (row, col): (Range<usize>, Range<usize>)) -> &Mat<T> {
            unsafe {
                let raw::Mat { data, nrows, ncols, stride } = self.repr();

                assert!(row.start <= row.end);
                assert!(row.end <= nrows);
                assert!(col.start <= col.end);
                assert!(col.end <= ncols);

                let data = data.offset((row.start * stride + col.start) as isize);
                let nrows = row.end - row.start;
                let ncols = col.end - col.start;

                mem::transmute(raw::Mat { data: data, nrows: nrows, ncols: ncols, stride: stride })
            }
        }
    }

    impl<T> Index<(RangeFull, usize)> for Mat<T> {
        type Output = Col<T>;

        fn index(&self, (_, col): (RangeFull, usize)) -> &Col<T> {
            unsafe {
                let raw::Mat { data, nrows, ncols, stride  } = self.repr();

                assert!(col < ncols);

                let data = data.offset(col as isize);

                mem::transmute(raw::Slice { data: data, len: nrows, stride: stride })
            }
        }
    }

    impl<T> Index<(usize, usize)> for Mat<T> {
        type Output = T;

        fn index(&self, (row, col): (usize, usize)) -> &T {
            unsafe {
                let raw::Mat { data, nrows, ncols, stride } = self.repr();

                assert!(row < nrows && col < ncols);

                &*data.offset((row * stride + col) as isize)
            }
        }
    }

    impl<T> Index<usize> for Mat<T> {
        type Output = Row<T>;

        fn index(&self, row: usize) -> &Row<T> {
            unsafe {
                let raw::Mat { data, nrows, ncols, .. } = self.repr();

                assert!(row < nrows);

                let data = data.offset((row * ncols) as isize);

                mem::transmute(slice::from_raw_parts(data, ncols))
            }
        }
    }
}

// A dense matrix stored in contiguous memory
unsized type Mat<T> = raw::Mat<T>;

// A view into the row of a matrix
#[derive(Debug)]
pub struct Row<T>([T]);

impl<T> Mat<T> {
    fn reshape<'a>(slice: &'a [T], (nrows, ncols): (usize, usize)) -> &'a Mat<T> {
        unsafe {
            assert_eq!(slice.len(), nrows * ncols);

            let data = slice.as_ptr() as *mut T;

            mem::transmute(raw::Mat { data: data, nrows: nrows, ncols: ncols })
        }
    }

    fn repr(&self) -> raw::Mat<T> {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<T> fmt::Debug for Mat<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

/// A contiguous matrix is also a strided matrix with `stride = ncols`
impl<T> Deref for Mat<T> {
    type Target = strided::Mat<T>;

    fn deref(&self) -> &strided::Mat<T> {
        unsafe {
            let raw::Mat { data, nrows, ncols } = self.repr();

            mem::transmute(strided::raw::Mat {
                data: data, nrows: nrows, ncols: ncols, stride: ncols,
            })
        }
    }
}

impl<T> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        unsafe {
            &mem::transmute::<_, &[T]>(self)[i]
        }
    }
}
