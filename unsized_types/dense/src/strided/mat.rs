use std::marker::Unsized;
use std::ops::{Index, Range, RangeFull};
use std::raw::FatPtr;
use std::{fat_ptr, fmt, mem, slice};

#[derive(Clone, Copy, Debug)]
pub struct Info {
    pub nrows: usize,
    pub ncols: usize,
    pub stride: usize,
}

impl<T> ::strided::Mat<T> {
    pub fn repr(&self) -> FatPtr<T, Info> {
        fat_ptr::repr(self)
    }
}

impl<T> fmt::Debug for ::strided::Mat<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let FatPtr { data, info } = self.repr();

            // NB(japaric) this should use iterators, but I don't want to make example bigger
            for i in 0..info.nrows {
                if i != 0 {
                    try!(f.write_str("\n"));
                }

                let data = data.offset((i * info.stride) as isize);
                let row = slice::from_raw_parts(data, info.ncols);

                try!(write!(f, "{:?}", row))
            }

            Ok(())
        }
    }
}

impl<T> Index<(Range<usize>, Range<usize>)> for ::strided::Mat<T> {
    type Output = ::strided::Mat<T>;

    fn index(&self, (row, col): (Range<usize>, Range<usize>)) -> &::strided::Mat<T> {
        let FatPtr { data, info } = self.repr();

        assert!(row.start <= row.end);
        assert!(row.end <= info.nrows);
        assert!(col.start <= col.end);
        assert!(col.end <= info.ncols);

        unsafe {
            &*fat_ptr::new(FatPtr {
                data: data.offset((row.start * info.stride + col.start) as isize),
                info: Info {
                    nrows: row.end - row.start,
                    ncols: col.end - col.start,
                    stride: info.stride,
                }
            })
        }
    }
}

impl<T> Index<(RangeFull, usize)> for ::strided::Mat<T> {
    type Output = ::strided::Col<T>;

    fn index(&self, (_, col): (RangeFull, usize)) -> &::strided::Col<T> {
        let FatPtr { data, info } = self.repr();

        assert!(col < info.ncols);

        unsafe {
            &*fat_ptr::new(FatPtr {
                data: data.offset(col as isize),
                info: ::strided::col::Info {
                    len: info.nrows,
                    stride: info.stride,
                }
            })
        }
    }
}

impl<T> Index<(usize, usize)> for ::strided::Mat<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &T {
        let FatPtr { data, info } = self.repr();

        assert!(row < info.nrows && col < info.ncols);

        unsafe {
            &*data.offset((row * info.stride + col) as isize)
        }
    }
}

impl<T> Index<usize> for ::strided::Mat<T> {
    type Output = ::Row<T>;

    fn index(&self, row: usize) -> &::Row<T> {
        unsafe {
            let FatPtr { data, info } = self.repr();

            assert!(row < info.nrows);

            let data = data.offset((row * info.stride) as isize);

            mem::transmute(slice::from_raw_parts(data, info.ncols))
        }
    }
}

impl<T> Unsized for ::strided::Mat<T> {
    type Data = T;
    type Info = Info;

    fn size_of_val(info: Info) -> usize {
        info.nrows * info.stride * mem::size_of::<T>()
    }
}
