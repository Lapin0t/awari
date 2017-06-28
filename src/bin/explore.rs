#![feature(placement_in_syntax,box_heap)]

use std::boxed::{Box,HEAP};

extern crate awari;

use awari::awari::{MAX_CODE,Awari,START_SEEDS,SIZE};

fn explore(start: Awari) -> Box<[bool; MAX_CODE+1]> {
    let mut tbl = HEAP <- [false; MAX_CODE+1];
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
    for i in 0..SIZE {
        start[i] = START_SEEDS as u8;
    }
    let tbl = explore(start);
    for i in 0..MAX_CODE+1 {
        println!("{}: {}", i, tbl[i]);
    }
}
