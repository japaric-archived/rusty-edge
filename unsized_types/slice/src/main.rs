#![feature(box_raw)]
#![feature(core)]
#![feature(filling_drop)]
#![feature(raw)]
#![feature(unsized_types)]

use std::marker::Unsized;
use std::ops::{Index, Range};
use std::raw::FatPtr;
use std::{fat_ptr, fmt, mem, ptr, slice};

macro_rules! show {
    ($e:expr) => {
        println!("> {}", stringify!($e));
        println!("{:?}\n", $e);
    }
}

fn main() {
    let array;
    show!(array = [0, 1, 2, 3]);

    let slice;
    println!("// Equivalent to `let slice: &[i32] = &array`");
    println!("// `slice: &'array Slice<i32>`");
    show!(slice = coerce_ref(&array));

    show!(slice);

    println!("// `slice` is a fat pointer");
    show!(mem::size_of_val(&slice));

    println!("// In-memory representation of `slice`");
    println!("// `slice.repr(): FatPtr<i32, usize>`");
    show!(slice.repr());

    println!("// Element indexing");
    show!(slice[3]);

    println!("// Slicing");
    show!(&slice[1..3]);

    let boxed;
    println!("// Equivalent to `let boxed: Box<[i32]> = Box::new([..])`");
    println!("// `boxed: `Box<Slice<Box<i32>>>");
    show!(boxed = coerce_box(Box::new([Box::new(4), Box::new(5), Box::new(6), Box::new(7)])));

    show!(drop(boxed));
}

unsized type Slice<T>;

impl<T> Slice<T> {
    fn repr(&self) -> FatPtr<T, usize> {
        fat_ptr::repr(self)
    }
}

impl<T> fmt::Debug for Slice<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            try!(f.write_str("["));

            let FatPtr { data, info: len } = self.repr();
            for i in 0..len {
                if i != 0 {
                    try!(write!(f, ", "));
                }

                try!(write!(f, "{:?}", *data.offset(i as isize)))
            }

            f.write_str("]")
        }
    }
}

impl<T> Drop for Slice<T> {
    fn drop(&mut self) {
        let FatPtr { data, info: len } = self.repr();

        if !data.is_null() && data as usize != mem::POST_DROP_USIZE {
            println!("dropping contents of `Slice`");

            unsafe {
                for x in slice::from_raw_parts(data, len) {
                        ptr::read(x);
                }
            }
        }
    }
}

impl<T> Index<Range<usize>> for Slice<T> {
    type Output = Slice<T>;

    fn index(&self, Range { start, end }: Range<usize>) -> &Slice<T> {
        let FatPtr { data, info: len } = self.repr();

        assert!(start <= end);
        assert!(end <= len);

        unsafe {
            &*fat_ptr::new(FatPtr {
                data: data.offset(start as isize),
                info: end - start,
            })
        }
    }
}

impl<T> Index<usize> for Slice<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        let FatPtr { data, info: len } = self.repr();

        assert!(i < len);

        unsafe {
            &*data.offset(i as isize)
        }
    }
}

impl<T> Unsized for Slice<T> {
    type Data = T;
    /// The length of the slice
    type Info = usize;

    fn size_of_val(len: usize) -> usize {
        len * mem::size_of::<T>()
    }
}

// The standard library version of these traits control which types can be unsized to DST
trait Unsize<T: ?Sized + Unsized> {
    fn unsized_info() -> T::Info;
}

// However these traits are not currently implementable by users, and what they actually do is
// hard-coded in the compiler. But these methods are an approximation of what the compiler magic
// does
trait CoerceUnsized<T: ?Sized + Unsized>: Unsize<T> {}

impl<T> Unsize<Slice<T>> for [T; 4] {
    fn unsized_info() -> usize {
        4
    }
}

/// Coerces from &SizedTy to &UnsizedTy
fn coerce_ref<S, U: ?Sized>(sized: &S) -> &U where
    S: Unsize<U>,
    U: Unsized,
{
    let data = sized as *const S as *mut U::Data;
    let info: U::Info = S::unsized_info();

    unsafe {
        &*fat_ptr::new(FatPtr {
            data: data,
            info: info,
        })
    }
}

/// Coerces from Box<SizedTy> to Box<UnsizedTy>
fn coerce_box<S, U: ?Sized>(sized: Box<S>) -> Box<U> where
    S: Unsize<U>,
    U: Unsized,
{
    let data = &*sized as *const S as *mut U::Data;
    mem::forget(sized);
    let info: U::Info = S::unsized_info();

    unsafe {
        Box::from_raw(fat_ptr::new(FatPtr {
            data: data,
            info: info,
        }))
    }
}
