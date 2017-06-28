#![feature(placement_in_syntax,box_heap)]

use std::boxed::{Box,HEAP};

extern crate awari;

use awari::{NBOARDS,START_SEEDS,FPITS};
use awari::awari::Awari;

fn explore(start: Awari) -> Box<[bool; NBOARDS]> {
    let mut tbl = HEAP <- [false; NBOARDS];
    let mut stack = vec![start];
    while let Some(u) = stack.pop() {
        for (v, _) in u.successors() {
            let id = v.encode();
            if !tbl[id] {
                tbl[id] = true;
                stack.push(v);
            }
        }
    }
    return tbl;
}

fn main() {
    println!("blabla");
    let mut start = Awari::new();
    for i in 0..FPITS {
        start[i] = START_SEEDS as u8;
    }
    let tbl = explore(start);
    for i in 0..NBOARDS {
        println!("{}: {}", i, tbl[i]);
    }
}
