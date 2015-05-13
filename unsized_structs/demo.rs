#![feature(unsized_structs)]

use std::{fmt, mem, slice};
use std::ops::{Index, Range, RangeFull};

macro_rules! show {
    ($e:expr) => {
        println!("> {}", stringify!($e));
        println!("{:?}\n", $e);
    }
}

fn main() {
    rpass();

    let array = [
        0, 1, 2, 3, 4,
        5, 6, 7, 8, 9,
        8, 7, 6, 5, 4,
    ];

    println!("// `array: [i32; 15]`");
    show!(array);

    let m;

    println!("// `m: &'array Mat<i32>`");
    show!(m = Mat::reshape(&array, (3, 5)));

    show!(m);

    show!(m[(1, 2)]);

    println!("// `&m[(1, ..)]: &'array Row<i32>`");
    show!(&m[(1, ..)]);

    println!("// `&m[(.., 2)]: &'array Col<i32>`");
    show!(&m[(.., 2)]);

    show!(&m[0]);

    show!(m[0][2]);

    show!(&m[(1..3, 1..4)]);
}

// TODO DRY, implementation work can be reduced by adding
// `impl<T> Deref for Mat<T> { type Target = SubMat<T>; }`, and deferring `Mat`'s methods to
// `SubMat`'s

/// A matrix stored in contiguous memory
pub unsized struct Mat<T> {
    data: *mut T,
    ncols: usize,
    nrows: usize,
}

impl<T> Mat<T> {
    /// Returns a matrix view of a slice
    pub fn reshape<'a>(slice: &'a [T], (nrows, ncols): (usize, usize)) -> &'a Mat<T> {
        assert_eq!(slice.len(), nrows * ncols);

        // NOTE The raw representation (`Mat`) gets coerced into a fat pointer (`&Mat`)
        // This is equivalent to `mem::transmute::<_, &[T]>(raw::Slice { .. })`
        Mat {
            data: slice.as_ptr() as *mut T,
            nrows: nrows,
            ncols: ncols,
        }
    }

    pub fn ncols(&self) -> usize {
        let Mat { ncols, .. } = self;

        ncols
    }

    pub fn nrows(&self) -> usize {
        let Mat { nrows, .. } = self;

        nrows
    }

    pub fn size(&self) -> (usize, usize) {
        // NOTE This is equivalent to `raw::Slice { len, .. } = self.repr()` where `Self = [T]`
        let Mat { nrows, ncols, .. } = self;

        (nrows, ncols)
    }

    fn data(&self) -> *mut T {
        let Mat { data, .. } = self;

        data
    }
}

impl<T> fmt::Debug for Mat<T> where T: fmt::Debug {
    // TODO `impl Deref<Target=SubMat> for Mat`, and defer this impl to `SubMat`'s `Debug` impl
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            // TODO use a row-by-row iterator
            for i in 0..self.nrows() {
                if i != 0 {
                    try!(write!(f, "\n"));
                }

                try!(write!(f, "{:?}", mem::transmute::<_, &[T]>(&self[i])))
            }

            Ok(())
        }
    }
}

/// Element indexing
impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &T {
        unsafe {
            &*self.data().offset((row * self.ncols() + col) as isize)
        }
    }
}

/// Row indexing
impl<T> Index<(usize, RangeFull)> for Mat<T> {
    type Output = Row<T>;

    fn index(&self, (row, _): (usize, RangeFull)) -> &Row<T> {
        unsafe {
            let (nrows, ncols) = self.size();

            assert!(row < nrows);

            let data = self.data().offset((row * ncols) as isize);

            mem::transmute(slice::from_raw_parts(data, ncols))
        }
    }
}

/// Row indexing (alternative form)
impl<T> Index<usize> for Mat<T> {
    type Output = Row<T>;

    fn index(&self, row: usize) -> &Row<T> {
        &self[(row, ..)]
    }
}

/// Column indexing
impl<T> Index<(RangeFull, usize)> for Mat<T> {
    type Output = Col<T>;

    fn index(&self, (_, col): (RangeFull, usize)) -> &Col<T> {
        unsafe {
            let (nrows, ncols) = self.size();

            assert!(col < ncols);

            let data = self.data().offset(col as isize);

            Col {
                data: data,
                len: nrows,
                stride: ncols,
            }
        }
    }
}

/// Matrix slicing
impl<T> Index<(Range<usize>, Range<usize>)> for Mat<T> {
    type Output = SubMat<T>;

    fn index(&self, (row, col): (Range<usize>, Range<usize>)) -> &SubMat<T> {
        let (nrows, ncols) = self.size();

        assert!(row.start <= row.end);
        assert!(row.end <= nrows);
        assert!(col.start <= col.end);
        assert!(col.end <= ncols);

        let stride = nrows;
        let data = unsafe {
            self.data().offset((row.start * stride + col.start) as isize)
        };

        SubMat {
            data: data,
            nrows: row.end - row.start,
            ncols: col.end - col.start,
            stride: nrows,
        }
    }
}

/// A view into the column of a matrix
pub unsized struct Col<T> {
    data: *mut T,
    len: usize,
    stride: usize,
}

impl<T> Col<T> {
    pub fn len(&self) -> usize {
        let Col { len, .. } = self;

        len
    }

    fn data(&self) -> *mut T {
        let Col { data, .. } = self;

        data
    }

    fn stride(&self) -> usize {
        let Col { stride, .. } = self;

        stride
    }
}

impl<T> fmt::Debug for Col<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Col(["));

        // TODO use iteration
        for i in 0..self.len() {
            if i != 0 {
                try!(write!(f, ", "));
            }

            try!(write!(f, "{:?}", self[i]))
        }

        write!(f, "])")
    }
}

/// Element indexing
impl<T> Index<usize> for Col<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        unsafe {
            assert!(i < self.len());

            &*self.data().offset((i * self.stride()) as isize)
        }
    }
}

/// A view into the row of a matrix
pub struct Row<T>([T]);

impl<T> fmt::Debug for Row<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "Row({:?})", mem::transmute::<_, &[T]>(self))
        }
    }
}

/// Element indexing
impl<T> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        unsafe {
            &mem::transmute::<_, &[T]>(self)[i]
        }
    }
}

/// A sub-matrix which may not be stored in contiguous memory
pub unsized struct SubMat<T> {
    data: *mut T,
    ncols: usize,
    nrows: usize,
    stride: usize,
}

impl<T> SubMat<T> {
    pub fn ncols(&self) -> usize {
        let SubMat { ncols, .. } = self;

        ncols
    }

    pub fn nrows(&self) -> usize {
        let SubMat { nrows, .. } = self;

        nrows
    }

    pub fn size(&self) -> (usize, usize) {
        let SubMat { nrows, ncols, .. } = self;

        (nrows, ncols)
    }

    fn data(&self) -> *mut T {
        let SubMat { data, .. } = self;

        data
    }

    fn stride(&self) -> usize {
        let SubMat { stride, .. } = self;

        stride
    }
}

impl<T> fmt::Debug for SubMat<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO implement this using iterators
        for r in 0..self.nrows() {
            if r == 0 {
                try!(write!(f, "["));
            } else {
                try!(write!(f, "\n["));
            }

            for c in 0..self.ncols() {
                if c != 0 {
                    try!(write!(f, ", "));
                }

                try!(write!(f, "{:?}", self[(r, c)]))
            }

            try!(write!(f, "]"));
        }

        Ok(())
    }
}

/// Element indexing
impl<T> Index<(usize, usize)> for SubMat<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &T {
        let (nrows, ncols) = self.size();

        assert!(row < nrows);
        assert!(col < ncols);

        let stride = self.stride();

        unsafe {
            &*self.data().offset((row * stride + col) as isize)
        }
    }
}

// run-pass tests
fn rpass() {
    unsized_size_of();
}

fn unsized_size_of() {
    let word = mem::size_of::<usize>();

    assert_eq!(mem::size_of::<&Mat<()>>(), 3 * word);
    assert_eq!(mem::size_of::<*mut Mat<()>>(), 3 * word);
    assert_eq!(mem::size_of::<Box<Mat<()>>>(), 3 * word);
}

// compile-fail tests
#[cfg(cfail)]
fn unsized_is_not_sized() -> Mat<()> {}
//~ error: the trait `core::marker::Sized` is not implemented for the type `Mat<()>`

// TODO
//
// - Add unsized struct field restrictions
// - `Mat` -> `&Mat`/`Box<Mat>` coercion should be unsafe
// - Test destructors, `impl Drop for Mat` and check dropping `Box<Mat>`/`Rc<Mat>` under valgrind
// - Test privacy, shouldn't be able to create fat pointers from outside the module where the
// unsized struct was defined
//
// FIXME
//
// - Accessing the fields of a fat pointer crashes in trans
// - `foo.deref().bar()` works but `foo.bar()` crashes where `bar` is *not* a method of `Foo`
// - Most likely doesn't work cross crate
