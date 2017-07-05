use std::boxed::{Box,HEAP};
use std::fs::{File,OpenOptions};
use std::io::{Seek,SeekFrom};
use std::cell::RefCell;
use std::option::Option;

use bincode::{deserialize_from,serialize_into,Bounded};

use {NBOARDS,SEEDS};
use ra::{State,Storage};


pub struct NaiveRam(Box<[State; NBOARDS]>);


impl Storage for NaiveRam {
    fn new() -> Self {
        NaiveRam(HEAP <- [State::Unstable(-(SEEDS as i8), 0); NBOARDS])
    }

    fn pre_row_hook(&mut self, _: usize) {}

    fn update(&mut self, i: usize, up: i8, sat_lvl: i8) -> Option<i8> {
        self.0[i].update(up, sat_lvl)
    }

    fn value(&self, i: usize) -> i8 {
        self.0[i].value()
    }

    fn try_stabilize(&mut self, i: usize) -> Option<i8> {
        self.0[i].try_stabilize()
    }

    fn set(&mut self, i: usize, s: State) {
        self.0[i] = s;
    }
}


pub struct NaiveDisk(RefCell<File>);


impl NaiveDisk {
    fn offset(i: usize) -> u64 { 6 * i as u64 }

    fn get(&self, i: usize) -> State {
        let mut f = self.0.borrow_mut();
        f.seek(SeekFrom::Start(NaiveDisk::offset(i))).unwrap();
        return deserialize_from(&mut *f, Bounded(6)).unwrap();
    }
}

impl Storage for NaiveDisk {
    fn new() -> Self {
        NaiveDisk(RefCell::new(OpenOptions::new()
          .read(true)
          .write(true)
          .create(true)
          .truncate(true)
          .open("foobar.db").unwrap()))

        // trick to allocate disk space (write on last bytes)
        //file.seek(SeekFrom::Start(NaiveDisk::offset(NBOARDS - 1))).unwrap();
        //serialize_into(&mut file, &State::Stable(0), Infinite).unwrap();

        //return NaiveDisk(RefCell::new(file));
    }

    fn pre_row_hook(&mut self, _: usize) {}

    fn set(&mut self, i: usize, s: State) {
        //println!("setting board {}", i);
        let mut f = self.0.borrow_mut();
        f.seek(SeekFrom::Start(NaiveDisk::offset(i))).unwrap();
        serialize_into(&mut *f, &s, Bounded(6)).unwrap();
        //f.flush().unwrap();
    }

    fn update(&mut self, i: usize, up: i8, sat_lvl: i8) -> Option<i8> {
        let mut s = self.get(i);
        let r = s.update(up, sat_lvl);
        self.set(i, s);
        return r;
    }

    fn try_stabilize(&mut self, i: usize) -> Option<i8> {
        let mut s = self.get(i);
        let r = s.try_stabilize();
        self.set(i, s);
        return r;
    }

    fn value(&self, i: usize) -> i8 {
        self.get(i).value()
    }
}
