use std::env;
use std::option::Option;
use std::fs::File;
use std::io::{BufWriter,Write};
use std::mem::transmute;

extern crate awari;

use awari::{PITS,SEEDS,NBOARDS};
use awari::awari::Awari;
use awari::ra::analyze;
use awari::models;


fn main() {
    println!("PITS={} SEEDS={} NBOARDS={}", PITS, SEEDS, NBOARDS);

    let upto = match env::args().nth(1) {
        Option::Some(n) => n.parse::<usize>().unwrap(),
        Option::None => SEEDS,
    };

    let table = analyze::<models::NaiveRAM>(upto);

    if !env::args().any(|x| x == "--quiet") {
        let mut out = BufWriter::new(File::create("out").unwrap());
        for n in 0..upto+1 {
            //println!("configurations with {} seeds", n);
            for (c, u) in Awari::iter_config(n) {
                write!(&mut out, "{:10}: {}\n", c, table.index(c).value()).unwrap();
            }
        }
        let mut out = BufWriter::new(File::create("access").unwrap());
        let tmp = table.stats();
        for &i in tmp.iter() {
            unsafe {
                out.write(&transmute::<usize,[u8;8]>(i)).unwrap();
            }
        }
    }
    println!("memory accesses: {}", table.stats().len());
}
