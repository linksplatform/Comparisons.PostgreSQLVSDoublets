use {
    crate::Benched,
    std::ops::{Deref, DerefMut, Drop},
};

pub struct Fork<'a, S: Benched>(&'a mut S);

impl<'a, S: Benched> Fork<'a, S> {
    pub fn new(storage: &'a mut S) -> Self {
        Fork(storage)
    }
}

impl<S: Benched> Deref for Fork<'_, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<S: Benched> DerefMut for Fork<'_, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<S: Benched> Drop for Fork<'_, S> {
    fn drop(&mut self) {
        let _ = self.drop_storage();
    }
}
