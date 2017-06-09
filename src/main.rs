#![feature(plugin)]
#![feature(conservative_impl_trait)]
#![feature(box_syntax)]
#![feature(placement_in_syntax, placement_new_protocol, box_heap)]

#![cfg_attr(test, plugin(quickcheck_macros))]
#![allow(dead_code)]  // TODO: maybe remove this when everything is done


#[cfg(test)]
extern crate quickcheck;


mod utils;
mod awari;
mod retrograde;

#[cfg(test)]
mod tests;


fn main() {
    println!("start");
    let table = retrograde::analysis();
    println!("test: {}", table[0].0);
    println!("done");
}
