use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct DebuggableMutate<T: Debug> {
    data: UnsafeCell<T>,
}

unsafe impl<T: Debug> Sync for DebuggableMutate<T> {}

impl<T: Debug> DebuggableMutate<T> {
    #[inline]
    pub fn new(user_data: T) -> DebuggableMutate<T> {
        DebuggableMutate {
            data: UnsafeCell::new(user_data),
        }
    }

    #[inline]
    pub fn get(&self) -> &T {
        unsafe {
            &*self.data.get()
        }
    }

    #[inline]
    pub fn get_mut(&self) -> &mut T {
        unsafe {
            &mut *self.data.get()
        }
    }
}

impl<T: Debug> Deref for DebuggableMutate<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.get()
    }
}

impl<T: Debug> DerefMut for DebuggableMutate<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}