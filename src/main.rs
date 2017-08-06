#[macro_use] extern crate log;
extern crate env_logger;
extern crate awari;

use awari::{PITS,SEEDS,NBOARDS};
use awari::ra::{build,MMaped};


fn main() {
    env_logger::init().unwrap();

    info!("{{\"pits\":{},\"seeds\":{},\"nboards\":{}}}", PITS, SEEDS, NBOARDS);

    let mut table = MMaped::new(".").unwrap();
    build(&mut table);
}
