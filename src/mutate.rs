use std::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub(crate) struct DebuggableMutate<T: Debug> {
    data: UnsafeCell<T>,
}

unsafe impl<T: Debug> Sync for DebuggableMutate<T> {}

impl<T: Debug> DebuggableMutate<T> {
    #[inline]
    pub fn new(user_data: T) -> DebuggableMutate<T> {
        DebuggableMutate {
            data: UnsafeCell::new(user_data)
        }
    }

    #[inline]
    pub fn get(&self) -> &T {
        unsafe { &*self.data.get() }
    }

    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.data.get() }
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
