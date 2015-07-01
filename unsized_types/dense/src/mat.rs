use std::marker::Unsized;
use std::ops::Deref;
use std::raw::FatPtr;
use std::{fat_ptr, fmt, mem};

#[derive(Clone, Copy, Debug)]
struct Info {
    pub ncols: usize,
    pub nrows: usize,
}

impl<T> ::Mat<T> {
    pub fn reshape<'a>(slice: &'a [T], (nrows, ncols): (usize, usize)) -> &'a ::Mat<T> {
        assert_eq!(slice.len(), nrows * ncols);

        unsafe {
            &*fat_ptr::new(FatPtr {
                data: slice.as_ptr() as *mut T,
                info: Info {
                    ncols: ncols,
                    nrows: nrows,
                }
            })
        }
    }

    pub fn repr(&self) -> FatPtr<T, Info> {
        fat_ptr::repr(self)
    }
}

impl<T> fmt::Debug for ::Mat<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

/// A contiguous matrix is also a strided matrix with `stride = ncols`
impl<T> Deref for ::Mat<T> {
    type Target = ::strided::Mat<T>;

    fn deref(&self) -> &::strided::Mat<T> {
        let FatPtr { data, info } = self.repr();

        unsafe {
            &*fat_ptr::new(FatPtr {
                data: data,
                info: ::strided::mat::Info {
                    nrows: info.nrows,
                    ncols: info.ncols,
                    stride: info.ncols,
                }
            })
        }
    }
}

impl<T> Unsized for ::Mat<T> {
    type Data = T;
    type Info = Info;

    fn size_of_val(info: Info) -> usize {
        info.nrows * info.ncols * mem::size_of::<T>()
    }
}
