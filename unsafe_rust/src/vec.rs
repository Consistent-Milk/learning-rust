use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We are not ready to handle ZST");
        Vec {
            ptr: (NonNull::dangling()),
            cap: (0),
            len: (0),
        }
    }
}

impl<T> Vec<T> {
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap: usize = 2 * self.cap;

            let new_layout: Layout = Layout::array::<T>(new_cap).unwrap();

            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr: *mut u8 = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout: Layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr: *mut u8 = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.cap = new_cap;
    }
}

impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;

            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }
}
