#![feature(placement_in_syntax, box_heap)]

#![cfg_attr(test, feature(test, plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
extern crate test;

#[cfg(test)]
extern crate rand;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate bincode;


pub mod utils;
pub mod awari;
pub mod ra;
pub mod models;

// include constants (see `build.rs`)
include!(concat!(env!("OUT_DIR"), "/size.rs"));
