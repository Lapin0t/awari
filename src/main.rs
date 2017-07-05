use std::env;
use std::option::Option;

extern crate awari;

use awari::{PITS,SEEDS,NBOARDS};
use awari::awari::Awari;
use awari::ra::{analyze,Storage};
use awari::models;


fn main() {
    println!("PITS={} SEEDS={} NBOARDS={}", PITS, SEEDS, NBOARDS);

    let upto = match env::args().nth(1) {
        Option::Some(n) => n.parse::<usize>().unwrap(),
        Option::None => SEEDS
    };

    let table = analyze::<models::NaiveDisk>(upto);
    for n in 0..upto+1 {
        println!("configurations with {} seeds", n);
        for u in Awari::iter_config(n) {
            let id = u.encode();
            println!("{:10}: {}", id, table.value(id));
        }
    }
}
