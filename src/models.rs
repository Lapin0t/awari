use std::boxed::{Box,HEAP};
use std::fs::{File,OpenOptions};
use std::io::{Read,Write,Seek,SeekFrom};

use super::{NBOARDS,SEEDS};
use ra::{State,Storage};


pub struct NaiveRam(Box<[State; NBOARDS]>);


impl Storage for NaiveRam {
    fn new() -> Self {
        NaiveRam(HEAP <- [State::Unstable(-(SEEDS as i8), 0); NBOARDS])
    }

    fn pre_row_hook(&mut self, _: usize) {}

    fn get(&self, i: usize) -> State {
        self.0[i]
    }
    
    fn set(&mut self, i: usize, s: State) {
        self.0[i] = s;
    }
}


pub struct NaiveDisk(&mut File);


impl Storage for NaiveDisk {
    fn new() -> Self {
        NaiveDisk(OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open("foobar.db")?)
    }

    fn pre_row_hook(&self, _: usize) {}

    fn get(&self, i: usize) -> State {
        deserialize_from()
    }
}
