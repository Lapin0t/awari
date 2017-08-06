use std::boxed::{Box,HEAP};
use std::path::Path;
use std::convert::AsRef;
use std::mem::{size_of,uninitialized};
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::io;
use std::ops::{Index,IndexMut};
use std::ptr;
use libc;

use NBOARDS;
use ra::{State,Table};


pub struct NaiveRAM {
    data: Box<[State; NBOARDS]>,
    stats: Vec<usize>,
}


impl NaiveRAM {
    pub fn new() -> Self {
        NaiveRAM {
            data: HEAP <- unsafe { uninitialized() },
            stats: Vec::new()
        }
    }
}


impl Table for NaiveRAM {
    fn insert(&mut self, i: usize, v: State) {
        self.stats.push(i);
        unsafe {
            ptr::write(&mut self[i], v);
        }
    }

    fn pre_hook(&mut self, _: usize) {}
    fn post_hook(&mut self, _: usize) {}

    fn finish_hook(&mut self) {
        info!("number of memory accesses: {}", self.stats.len())
    }
}

impl Index<usize> for NaiveRAM {
    type Output = State;
    fn index(&self, i: usize) -> &State {
        &self.data[i]
    }
}

impl IndexMut<usize> for NaiveRAM {
    fn index_mut(&mut self, i: usize) -> &mut State {
        self.stats.push(i);
        &mut self.data[i]
    }
}


pub struct MMaped {
    ptr: *mut State,
    len: usize,
}

impl MMaped {
    pub fn new<T: AsRef<Path>>(wd: T) -> io::Result<Self> {
        let size = size_of::<State>() * NBOARDS;
        let fd = OpenOptions::new()
                   .read(true)
                   .write(true)
                   .create(true)
                   .open(wd.as_ref().join("table_mmap"))?;
        fd.set_len(size as u64)?;

        let ptr = unsafe {
            libc::mmap(ptr::null_mut(), size as libc::size_t,
                       libc::PROT_READ | libc::PROT_WRITE,
                       libc::MAP_SHARED, fd.as_raw_fd(), 0)
        };

        if ptr == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        } else {
            return Ok(MMaped { ptr: ptr as *mut State, len: size });
        }
    }
}

impl Table for MMaped {
    fn insert(&mut self, i: usize, s: State) {
        unsafe {
            ptr::write(self.ptr.offset(i as isize), s);
        }
    }

    fn pre_hook(&mut self, _: usize) {}
    fn post_hook(&mut self, _: usize) {}
    fn finish_hook(&mut self) {}
}

impl Index<usize> for MMaped {
    type Output = State;

    fn index(&self, i: usize) -> &State {
        debug_assert!(i < self.len);
        unsafe { &*self.ptr.offset(i as isize) }
    }
}

impl IndexMut<usize> for MMaped {
    fn index_mut(&mut self, i: usize) -> &mut State {
        debug_assert!(i < self.len);
        unsafe { &mut *self.ptr.offset(i as isize) }
    }
}

impl Drop for MMaped {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.len);
        }
    }
}
