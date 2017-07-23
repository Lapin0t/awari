use std::boxed::{Box,HEAP};
use std::ops::{Index,IndexMut};
use std::convert::Into;
use std::cell::RefCell;
use std::string::String;
use std::path::PathBuf;

use NBOARDS;
use ra::{State,Table};



pub struct NaiveRAM {
    data: Box<[State; NBOARDS]>,
    stats: RefCell<Vec<usize>>,
    wd: PathBuf,
}

impl Index<usize> for NaiveRAM {
    type Output = State;
    fn index(&self, i: usize) -> &State {
        self.stats.borrow_mut().push(i);
        self.data.index(i)
    }
}

impl IndexMut<usize> for NaiveRAM {
    fn index_mut(&mut self, i: usize) -> &mut State {
        self.stats.borrow_mut().push(i);
        self.data.index_mut(i)
    }
}

impl Table for NaiveRAM {
    fn new<T: Into<PathBuf>>(wd: T) -> Self {
        NaiveRAM { data: HEAP <- [Default::default(); NBOARDS],
                   stats: RefCell::new(Vec::new()),
                   wd: wd.into() }
    }

    fn pre_hook(&mut self, _: usize) {}
    fn post_hook(&mut self, _: usize) {}

    fn finish_hook(&mut self) {
        info!("number of memory accesses: {}", self.stats.borrow().len())
    }
}


//pub struct Hybrid<S> {}
