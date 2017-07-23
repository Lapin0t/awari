use std::env;
use std::option::Option;

#[macro_use] extern crate log;
extern crate env_logger;
extern crate awari;

use awari::{PITS,SEEDS,NBOARDS};
use awari::ra::{Table,NaiveRAM};


fn main() {
    env_logger::init().unwrap();

    info!("PITS={} SEEDS={} NBOARDS={}", PITS, SEEDS, NBOARDS);

    let upto = match env::args().nth(1) {
        Option::Some(n) => n.parse::<usize>().unwrap(),
        Option::None => SEEDS,
    };

    let mut table = NaiveRAM::new(".");
    table.build(upto);
}
