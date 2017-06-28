#![feature(plugin)]
#![feature(box_syntax)]
#![feature(placement_in_syntax, box_heap)]

#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;


pub mod utils;
pub mod awari;
pub mod retrograde;
