use std::boxed::{Box,HEAP};
use std::convert::AsRef;
use std::path::Path;
use std::collections::HashMap;
use std::mem::uninitialized;
use std::fs::File;
use std::io::{Read,Write,Seek,SeekFrom};
use tempfile::tempfile_in;

use NBOARDS;
use ra::{State,Table};


pub struct NaiveRAM {
    data: Box<[State; NBOARDS]>,
    stats: Vec<usize>,
}

impl Table for NaiveRAM {
    fn new<T: AsRef<Path>>(_: T) -> Self {
        NaiveRAM { data: HEAP <- [Default::default(); NBOARDS],
                   stats: Vec::new() }
    }

    fn insert(&mut self, i: usize, v: State) {
        self.stats.push(i);
        self.data[i] = v;
    }

    fn index_mut(&mut self, i: usize) -> &mut State {
        self.stats.push(i);
        &mut self.data[i]
    }

    fn pre_hook(&mut self, _: usize) {}
    fn post_hook(&mut self, _: usize) {}

    fn finish_hook(&mut self) {
        info!("number of memory accesses: {}", self.stats.len())
    }
}


const BLK_SHIFT: usize = 16;
const BLK_LEN: usize = (1 << BLK_SHIFT);
const MAX_BLKS: usize = 1 << (20 - BLK_SHIFT); // max 1GB
const DQ: usize = 4;

pub struct Hybrid {
    fd: File,
    cache: HashMap<usize,[State; BLK_LEN]>,
    dqi: usize,
    dq: [usize; DQ],
}

impl Hybrid {
    fn ensure(&mut self, blk: usize, ) {
        match self.cache.get(&blk) {
            Option::Some(_) => {},
            Option::None => {
                let mut buf: [u8; BLK_LEN * 2] = unsafe { uninitialized() };

                if self.cache.len() >= MAX_BLKS {
                    let line = self.cache.remove(&self.dqi).unwrap();
                    self.fd.seek(SeekFrom::Start((self.dqi << BLK_SHIFT + 1) as u64)).unwrap();
                    for (i, s) in line.iter().enumerate() {
                        s.serialize(&mut buf[2*i..2*i+2]);
                    }
                    self.fd.write(&buf).unwrap();
                }

                debug_assert!(self.cache.len() < MAX_BLKS);

                let mut line: [State; BLK_LEN] = unsafe { uninitialized() };

                self.fd.seek(SeekFrom::Start((blk << BLK_SHIFT + 1) as u64)).unwrap();
                info!("blk: {}", blk);
                self.fd.read_exact(&mut buf).unwrap();

                for (i, s) in line.iter_mut().enumerate() {
                    *s = State::deserialize(&buf[2*i..2*i+2]);
                }
                self.cache.insert(blk, line);
            }
        }
        self.dq[self.dqi] = blk;
        self.dqi = (self.dqi + 1) & (DQ - 1);
    }
}


impl Table for Hybrid {
    fn new<T: AsRef<Path>>(wd: T) -> Self {
        let fd = tempfile_in(wd).unwrap();
        fd.set_len(1 + 2 * NBOARDS as u64).unwrap();
        Hybrid { fd: fd,
                 cache: HashMap::with_capacity(MAX_BLKS),
                 dqi: 0,
                 dq: [0; DQ] }
    }

    fn insert(&mut self, n: usize, s: State) {
        let blk = n >> BLK_SHIFT;
        self.ensure(blk);
        self.cache.get_mut(&blk).unwrap()[n & (BLK_LEN - 1)] = s;
    }

    fn index_mut(&mut self, n: usize) -> &mut State {
        let blk = n >> BLK_SHIFT;
        self.ensure(blk);
        let line = self.cache.get_mut(&blk).unwrap();
        return &mut line[n & (BLK_LEN - 1)];
    }

    fn pre_hook(&mut self, _: usize) {}
    fn post_hook(&mut self, _: usize) {}
    fn finish_hook(&mut self) {}

}
