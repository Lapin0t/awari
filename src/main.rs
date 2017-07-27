use std::env;

#[macro_use] extern crate log;
extern crate env_logger;
extern crate awari;

use awari::{PITS,SEEDS,NBOARDS};
use awari::ra::{Table,NaiveRAM};


fn main() {
    env_logger::init().unwrap();

    info!("[pits:{},seeds:{},nboards:{}]", PITS, SEEDS, NBOARDS);

    let mut table = NaiveRAM::new(env::args().nth(1).unwrap());
    table.build();
}
