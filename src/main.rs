#![feature(plugin)]
#![feature(conservative_impl_trait)]
#![feature(box_syntax)]
#![feature(placement_in_syntax, box_heap)]
#![feature(associated_consts)]

#![cfg_attr(test, plugin(quickcheck_macros))]
#![allow(dead_code)]  // TODO: maybe remove this when everything is done

#[cfg(test)]
extern crate quickcheck;

#[macro_use]
extern crate log;


mod utils;
mod awari;
mod retrograde;

#[cfg(test)]
mod tests;


use awari::Awari;

fn main() {
/*    for u in awari::Board4::iter_config(3) {
        println!("\n\n===== board ===={:?}", u);
        for v in u.predecessors() {
            println!("\n{:?}", v);
        }
    }*/
    println!("start");
    let table = retrograde::analysis();
    for n in 0..4 {
        println!("configuration with {} seeds", n);
        for u in awari::Board4::iter_config(n) {
            let id = u.encode();
            println!("board: {:4}, score: {}", id, table[id].0);
        }
    }
}
