extern crate awari;

use awari::{PITS,SEEDS,NBOARDS};
use awari::awari::Awari;
use awari::ra::{analyze,Storage};
use awari::storage::NaiveRam;

fn main() {
    println!("PITS={} SEEDS={} NBOARDS={}", PITS, SEEDS, NBOARDS);
    let table = analyze::<NaiveRam>(SEEDS);
    for n in 0..SEEDS+1 {
        println!("configurations with {} seeds", n);
        for u in Awari::iter_config(n) {
            let id = u.encode();
            println!("{:10}: {}", id, table.get(id).value());
        }
    }
}
