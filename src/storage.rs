use std::marker::PhantomData;
use std::ops::{Deref,DerefMut,Drop};
use std::default::Default;


pub trait Backend<T>: Default {
    type Handle;

    fn get_handle(&self, usize) -> Self::Handle;
    fn deref_handle<'a>(&'a self, &'a Self::Handle) -> &'a T;
    fn deref_handle_mut<'a>(&'a mut self, &'a mut Self::Handle) -> &'a mut T;
    fn write_back(&mut self, &Self::Handle);
}


pub struct Storage<T, B> {
    backend: B,
    _marker: PhantomData<T>,
}

impl<T, B: Backend<T>> Storage<T, B> {
    #[inline]
    pub fn index(&self, i: usize) -> Ref<T, B> {
        Ref { handle: self.backend.get_handle(i),
              owner: &self.backend }
    }
    #[inline]
    pub fn index_mut(&mut self, i: usize) -> RefMut<T, B> {
        RefMut { handle: self.backend.get_handle(i),
                 owner: &mut self.backend }
    }
}

impl<T, B: Backend<T>> Default for Storage<T, B> {
    fn default() -> Self {
        Storage { backend: B::default(), _marker: PhantomData }
    }
}


pub struct Ref<'a, T, B: 'a + Backend<T>> {
    handle: B::Handle,
    owner: &'a B,
}


impl<'a, T, B: 'a + Backend<T>> Deref for Ref<'a, T, B> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        self.owner.deref_handle(&self.handle)
    }
}


pub struct RefMut<'a, T, B: 'a + Backend<T>> {
    handle: B::Handle,
    owner: &'a mut B,
}


impl<'a, T, B: 'a + Backend<T>> Deref for RefMut<'a, T, B> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        self.owner.deref_handle(&self.handle)
    }
}


impl<'a, T, B: 'a + Backend<T>> DerefMut for RefMut<'a, T, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.owner.deref_handle_mut(&mut self.handle)
    }
}


impl<'a, T, B: 'a + Backend<T>> Drop for RefMut<'a, T, B> {
    #[inline]
    fn drop(&mut self) {
        self.owner.write_back(&self.handle)
    }
}
