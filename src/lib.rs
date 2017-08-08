#![feature(placement_in_syntax,box_heap,nonzero)]

#![cfg_attr(test, feature(test, plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate test;
#[cfg(test)] extern crate rand;
#[macro_use] extern crate slog;
extern crate libc;
extern crate core;
extern crate tempfile;


pub mod utils;
pub mod awari;
pub mod ra;
mod storage;

// include constants (see `build.rs`)
include!(concat!(env!("OUT_DIR"), "/params.rs"));
