use std::marker::Unsized;
use std::raw::FatPtr;
use std::{fat_ptr, fmt, mem};

#[derive(Clone, Copy)]
pub struct Info {
    pub len: usize,
    pub stride: usize,
}

impl<T> ::strided::Col<T> {
    pub fn repr(&self) -> FatPtr<T, Info> {
        fat_ptr::repr(self)
    }
}

impl<T> fmt::Debug for ::strided::Col<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let FatPtr { data, info } = self.repr();

            try!(f.write_str("Col(["));

            // NB(japaric) this should use iterators, but I don't want to make example bigger
            for i in 0..info.len {
                if i != 0 {
                    try!(f.write_str(", "));
                }

                try!(write!(f, "{:?}", *data.offset((i * info.stride) as isize)))
            }

            f.write_str("])")
        }
    }
}

impl<T> Unsized for ::strided::Col<T> {
    type Data = T;
    type Info = Info;

    fn size_of_val(info: Info) -> usize {
        info.len * info.stride * mem::size_of::<T>()
    }
}
