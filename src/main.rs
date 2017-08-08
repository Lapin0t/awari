#[macro_use] extern crate slog;
extern crate slog_json;
extern crate awari;

use slog::Drain;
use std::sync::Mutex;
use std::io::BufWriter;
use std::fs::OpenOptions;

use awari::{PITS,SEEDS,NBOARDS};
use awari::ra::{Driver,MMaped};


fn main() {
    let root = slog::Logger::root(
        Mutex::new(slog_json::Json::default(
            OpenOptions::new()
              .create(true)
              .write(true)
              .truncate(true)
              .open("log")
              .unwrap()
              )).map(slog::Fuse),
        o!()
    );

    info!(root, "params"; "pits" => PITS, "seeds" => SEEDS, "nboards" => NBOARDS);

    let mut driver = Driver { table: MMaped::new("tmp", root.new(o!())).unwrap(),
                              logger: root };
    driver.run();
}
