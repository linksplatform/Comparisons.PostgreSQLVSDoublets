use std::ops::{Deref, DerefMut};

/// Compatibility wrapper around backends whose SQL clients use a mutex.
/// Shared reads lock the client, as required by the Doublets `Send + Sync` API.
pub struct Exclusive<T>(T);

impl<T> Exclusive<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn get(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Exclusive<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Exclusive<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
