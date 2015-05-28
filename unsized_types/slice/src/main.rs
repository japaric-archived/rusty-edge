#![feature(alloc)]
#![feature(filling_drop)]
#![feature(unsized_types)]

use std::ops::{Index, Range};
use std::{fmt, mem, ptr};

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
    show!(slice = (&array).coerce_ref());

    show!(slice);

    println!("// `slice` is a fat pointer");
    show!(mem::size_of_val(&slice));

    println!("// In-memory representation of `slice`");
    println!("// `slice.repr(): raw::Slice<i32>`");
    show!(slice.repr());

    println!("// Element indexing");
    show!(slice[3]);

    println!("// Slicing");
    show!(&slice[1..3]);

    let boxed;
    println!("// Equivalent to `let boxed: Box<[i32]> = Box::new([4, 5, 6, 7])`");
    println!("// `boxed: `Box<Slice<i32>>");
    show!(boxed = Box::new([4, 5, 6, 7]).coerce_box());

    show!(drop(boxed));
}

mod raw {
    #[allow(raw_pointer_derive)]
    #[derive(Debug)]
    pub struct Slice<T> {
        pub data: *mut T,
        pub len: usize,
    }
}

unsized type Slice<T> = raw::Slice<T>;

impl<T> Slice<T> {
    fn repr(&self) -> raw::Slice<T> {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<T> fmt::Debug for Slice<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            try!(f.write_str("["));

            let raw::Slice { data, len } = self.repr();
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
        unsafe {
            let raw::Slice { data, len } = self.repr();

            if !data.is_null() && data as usize != mem::POST_DROP_USIZE {
                for i in 0..len {
                    ptr::read(&*data.offset(i as isize));
                }

                let data = data as *mut u8;
                let len = len * mem::size_of::<T>();
                let align = mem::min_align_of::<T>();

                println!("deallocating {} bytes starting at {:?}", len, data);
                std::rt::heap::deallocate(data, len, align);
            }
        }
    }
}

impl<T> Index<Range<usize>> for Slice<T> {
    type Output = Slice<T>;

    fn index(&self, r: Range<usize>) -> &Slice<T> {
        unsafe {
            assert!(r.start <= r.end);

            let raw::Slice { data, len } = self.repr();

            assert!(r.end <= len);

            let data = data.offset(r.start as isize);
            let len = r.end - r.start;

            mem::transmute(raw::Slice { data: data, len: len })
        }
    }
}

impl<T> Index<usize> for Slice<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        unsafe {
            let raw::Slice { data, len } = self.repr();

            assert!(i < len);

            &*data.offset(i as isize)
        }
    }
}

// The standard library version of these traits control which types can be unsized to DST
trait Unsize<T: ?Sized> {}

// However these traits are not currently implementable by users, and what they actually do is
// hard-coded in the compiler. But these methods are an approximation of what the compiler magic
// does
trait CoerceUnsized<T: ?Sized>: Unsize<T> {
    fn coerce_ref(&self) -> &T;
    fn coerce_mut(&mut self) -> &mut T;
    fn coerce_box(self: Box<Self>) -> Box<T>;
}

// As you may notice, implementing these traits generically for all fixed-size arrays will require
// type level integers: `impl<T, usize N> Unsize<Slice<T>> for [T; N] {}`
impl<T> Unsize<Slice<T>> for [T; 4] {}

impl<T> CoerceUnsized<Slice<T>> for [T; 4] {
    fn coerce_ref(&self) -> &Slice<T> {
        unsafe {
            let data: *const _ = self;
            mem::transmute(raw::Slice { data: data as *mut T, len: 4 })
        }
    }

    fn coerce_mut(&mut self) -> &mut Slice<T> {
        unsafe {
            let data: *mut _ = self;
            mem::transmute(raw::Slice { data: data, len: 4 })
        }
    }

    fn coerce_box(mut self: Box<Self>) -> Box<Slice<T>> {
        unsafe {
            let data: *mut _ = &mut *self;
            mem::forget(self);
            mem::transmute(raw::Slice { data: data, len: 4 })
        }
    }
}

fn drop<T>(_: T) {}
