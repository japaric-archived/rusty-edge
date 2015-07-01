use std::marker::Unsized;
use std::ops::Index;
use std::raw::FatPtr;
use std::{fat_ptr, fmt, mem, slice};

#[derive(Clone, Copy)]
pub struct Info {
    /// The indices of the non-zero elements
    pub indices: *const usize,
    /// The length of the vector
    pub len: usize,
    /// The number of non-zero elements
    pub nnz: usize,
}

impl<T> ::Vector<T> {
    fn repr(&self) -> FatPtr<T, Info> {
        fat_ptr::repr(self)
    }
}

impl<T> fmt::Debug for ::Vector<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let FatPtr { data, info } = self.repr();

            try!(f.write_str("["));

            let mut k = 0;
            let mut nnz = info.nnz;
            for i in 0..info.len {
                if i != 0 {
                    try!(f.write_str(", "))
                }

                if nnz != 0 && i == *info.indices.offset(k) {
                    try!(write!(f, "{:?}", *data.offset(k)));
                    k += 1;
                    nnz -= 1;
                } else {
                    try!(write!(f, "_"));
                }
            }

            f.write_str("]")
        }
    }
}

/// Element indexing
impl<T> Index<usize> for ::Vector<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        unsafe {
            let FatPtr { data, info } = self.repr();

            for (&j, k) in slice::from_raw_parts(info.indices, info.nnz).iter().zip(0..) {
                if j > i {
                    break
                } else if j == i {
                    return &*data.offset(k)
                }
            }

            panic!("element not set");
        }
    }
}

impl<T> Unsized for ::Vector<T> {
    type Data = T;
    type Info = Info;

    fn size_of_val(info: Info) -> usize {
        info.nnz * mem::size_of::<T>()
    }
}
