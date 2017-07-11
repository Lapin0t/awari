use std::boxed::{Box,HEAP};
use std::fs::File;
use std::io::{Seek,SeekFrom,Read,Write};
use std::cell::RefCell;

use tempfile::tempfile;

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
        NaiveDisk(RefCell::new(tempfile().expect("couldn't create temporary file")))
    }
}

impl Backend<State> for NaiveDisk {
    type Handle = (State, usize);

    fn get_handle(&self, i: usize) -> Self::Handle {
        let mut f = self.0.borrow_mut();
        f.seek(SeekFrom::Start(2 * i as u64)).unwrap();

        let mut buf = [0; 2];
        f.read(&mut buf).unwrap();
        
        return (State::deserialize(buf), i);
    }

    #[inline]
    fn deref_handle<'a>(&'a self, h: &'a Self::Handle) -> &'a State { &h.0 }

    #[inline]
    fn deref_handle_mut<'a>(&'a mut self, h: &'a mut Self::Handle) -> &'a mut State { &mut h.0 }

    fn write_back(&mut self, s: &Self::Handle) {
        let mut f = self.0.borrow_mut();
        f.seek(SeekFrom::Start(2 * s.1 as u64)).unwrap();
        f.write(&s.0.serialize()).unwrap();
    }
}
