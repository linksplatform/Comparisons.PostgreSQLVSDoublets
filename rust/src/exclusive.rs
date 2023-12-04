use std::cell::UnsafeCell;

pub struct Exclusive<T>(UnsafeCell<T>);

impl<T> Exclusive<T> {
    pub fn new(t: T) -> Self {
        Exclusive(UnsafeCell::new(t))
    }

    pub fn get(&mut self) -> &mut T {
        self.0.get_mut()
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn assume_exclusive(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl<T> Sync for Exclusive<T> {}
