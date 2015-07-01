use std::marker::Unsized;
use std::ops::{Index, Range};
use std::raw::FatPtr;
use std::{fat_ptr, fmt, mem, ptr, slice};

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Debug)]
struct Info {
    /// Column indices
    pub col_ind: *const usize,
    /// Number of columns
    pub ncols: usize,
    /// Number of non-zero elements
    pub nnz: usize,
    /// Number of rows
    pub nrows: usize,
    /// Row offsets
    pub row_ptr: *const usize,
}

impl<T> ::Mat<T> {
    pub fn new<'a>(
        mut elems: Box<[T]>,
        col_ind: Box<[usize]>,
        row_ptr: Box<[usize]>,
        ncols: usize,
    ) -> Box<::Mat<T>> {
            let nnz = elems.len();

            assert_eq!(Some(&0), row_ptr.first());
            assert_eq!(Some(&nnz), row_ptr.last());
            assert_eq!(nnz, col_ind.len());

            let nrows = row_ptr.len().checked_sub(1).unwrap();
            let data = elems.as_mut_ptr();
            mem::forget(elems);
            let col_ind_ = col_ind.as_ptr();
            mem::forget(col_ind);
            let row_ptr_ = row_ptr.as_ptr();
            mem::forget(row_ptr);

        unsafe {
            Box::from_raw(fat_ptr::new(FatPtr {
                data: data,
                info: Info {
                    col_ind: col_ind_,
                    ncols: ncols,
                    nnz: nnz,
                    nrows: nrows,
                    row_ptr: row_ptr_,
                }
            }))
        }
    }

    pub fn repr(&self) -> FatPtr<T, Info> {
        fat_ptr::repr(self)
    }

    pub fn nrows(&self) -> usize {
        self.repr().info.nrows
    }
}

impl<T> fmt::Debug for ::Mat<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first = true;
        for i in 0..self.nrows() {
            if is_first {
                is_first = false;
            } else {
                try!(f.write_str("\n"))
            }

            try!(self[i].0.fmt(f))
        }

        Ok(())
    }
}

impl<T> Drop for ::Mat<T> {
    fn drop(&mut self) {
        let FatPtr { data, info } = self.repr();

        if !data.is_null() && data as usize != mem::POST_DROP_USIZE {
            unsafe {
                println!("dropping contents of the `data` pointer");
                for x in slice::from_raw_parts(data, info.nnz) {
                    ptr::read(x);
                }

                println!("dropping `col_ind`");
                mem::drop(Box::from_raw(slice::from_raw_parts_mut(info.col_ind as *mut usize, info.nnz)));

                println!("dropping `row_ptr`");
                mem::drop(Box::from_raw(slice::from_raw_parts_mut(info.row_ptr as *mut usize, info.nrows + 1)));
            }
        }
    }
}

/// Element indexing
impl<T> Index<(usize, usize)> for ::Mat<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        unsafe {
            let FatPtr { data, info } = self.repr();

            assert!(i < info.nrows);
            assert!(j < info.ncols);

            for k in *info.row_ptr.offset(i as isize)..*info.row_ptr.offset(i as isize + 1) {
                let k = k as isize;

                if j == *info.col_ind.offset(k) {
                    return &*data.offset(k)
                }
            }

            panic!("element not set");
        }
    }
}

/// Row indexing
impl<T> Index<usize> for ::Mat<T> {
    type Output = ::Row<T>;

    fn index(&self, i: usize) -> &::Row<T> {
        unsafe {
            let FatPtr { data, info } = self.repr();

            assert!(i < info.nrows);

            let i = i as isize;
            let offset = *info.row_ptr.offset(i);
            let nnz = *info.row_ptr.offset(i + 1) - offset;
            let offset = offset as isize;
            let data = data.offset(offset);

            let v: *mut ::Vector<T> = fat_ptr::new(FatPtr {
                data: data,
                info: ::vector::Info {
                    indices: info.col_ind.offset(offset),
                    nnz: nnz,
                    len: info.ncols,
                }
            });

            mem::transmute(v)
        }
    }
}

/// Row slicing
impl<T> Index<Range<usize>> for ::Mat<T> {
    type Output = ::Mat<T>;

    fn index(&self, Range { start, end }: Range<usize>) -> &::Mat<T> {
        unsafe {
            let FatPtr { data, info } = self.repr();

            assert!(start <= end);
            assert!(end <= info.nrows);

            let row_ptr = info.row_ptr.offset(start as isize);
            let nnz = *row_ptr.offset(start as isize + 1) - *row_ptr;

            &*fat_ptr::new(FatPtr {
                data: data,
                info: Info {
                    col_ind: info.col_ind,
                    row_ptr: row_ptr,
                    nnz: nnz,
                    ncols: info.ncols,
                    nrows: end - start,
                }
            })
        }
    }
}

impl<T> Unsized for ::Mat<T> {
    type Data = T;
    type Info = Info;

    fn size_of_val(info: Info) -> usize {
        info.nnz * mem::size_of::<T>()
    }
}
