use std::marker::PhantomData;
use std::ops::{Deref,DerefMut,Drop};
use std::default::Default;
use std::fmt::Debug;
use std::cell::RefCell;


pub trait Backend<T>: Default {
    type Handle;

    fn get_handle(&self, usize) -> Self::Handle;
    fn deref_handle<'a>(&'a self, &'a Self::Handle) -> &'a T;
    fn deref_handle_mut<'a>(&'a mut self, &'a mut Self::Handle) -> &'a mut T;
    fn write_back(&mut self, &Self::Handle);
}


pub trait Stats: Default {
    type Output: Debug;
    
    fn record(&mut self, usize);
    fn report(&self) -> Self::Output;
}


impl Stats for usize {
    type Output = usize;

    fn record(&mut self, _: usize) { *self += 1; }
    fn report(&self) -> usize { *self }
}


pub struct Storage<T, B, S=usize> {
    backend: B,
    stats: RefCell<S>,
    _marker: PhantomData<T>,
}

impl<T, B: Backend<T>, S: Stats> Storage<T, B, S> {
    pub fn get_stats(&self) -> S::Output {
        self.stats.borrow().report()
    }

    #[inline]
    pub fn index(&self, i: usize) -> Ref<T, B> {
        self.stats.borrow_mut().record(i);
        Ref { handle: self.backend.get_handle(i),
              owner: &self.backend }
    }
    #[inline]
    pub fn index_mut(&mut self, i: usize) -> RefMut<T, B> {
        self.stats.borrow_mut().record(i);
        RefMut { handle: self.backend.get_handle(i),
                 owner: &mut self.backend }
    }
}

impl<T, B: Backend<T>, S: Stats> Default for Storage<T, B, S> {
    fn default() -> Self {
        Storage { backend: B::default(),
                  stats: RefCell::new(S::default()),
                  _marker: PhantomData }
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
