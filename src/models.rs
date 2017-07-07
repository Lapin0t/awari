use std::boxed::{Box,HEAP};
use std::fs::{File,OpenOptions};
use std::io::{Seek,SeekFrom};
use std::cell::RefCell;

use bincode::{deserialize_from,serialize_into,Bounded};

use NBOARDS;
use ra::State;
use storage::Backend;


/// Fully in-memory storage model.
pub struct NaiveRAM { data: Box<[State; NBOARDS]> }


impl Default for NaiveRAM {
    fn default() -> Self {
        NaiveRAM { data: HEAP <- [Default::default(); NBOARDS] }
    }
}


impl Backend<State> for NaiveRAM {
    type Handle = usize;

    #[inline]
    fn get_handle(&self, i: usize) -> usize { i }

    #[inline]
    fn deref_handle(&self, i: &usize) -> &State {
        &self.data[*i]
    }

    #[inline]
    fn deref_handle_mut(&mut self, i: &mut usize) -> &mut State {
        &mut self.data[*i]
    }

    #[inline]
    fn write_back(&mut self, _: &usize) {}
}

/// Fully on-disk storage model.
pub struct NaiveDisk(RefCell<File>);


impl Default for NaiveDisk {
    fn default() -> Self {
        NaiveDisk(RefCell::new(
            OpenOptions::new()
              .read(true)
              .write(true)
              .create(true)
              .truncate(true)
              .open("foobar.db").unwrap()
            ))
    }
}

impl Backend<State> for NaiveDisk {
    type Handle = (State, usize);

    fn get_handle(&self, i: usize) -> Self::Handle {
        let mut f = self.0.borrow_mut();
        f.seek(SeekFrom::Start(6 * i as u64)).unwrap();
        return (deserialize_from(&mut *f, Bounded(6))
                  .unwrap_or(Default::default()), i);
    }

    #[inline]
    fn deref_handle<'a>(&'a self, h: &'a Self::Handle) -> &'a State { &h.0 }

    #[inline]
    fn deref_handle_mut<'a>(&'a mut self, h: &'a mut Self::Handle) -> &'a mut State { &mut h.0 }

    fn write_back(&mut self, s: &Self::Handle) {
        let mut f = self.0.borrow_mut();
        f.seek(SeekFrom::Start(6 * s.1 as u64)).unwrap();
        serialize_into(&mut *f, &s.0, Bounded(6)).unwrap();
    }
}
