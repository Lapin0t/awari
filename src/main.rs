extern crate awari;

use awari::{PITS,SEEDS,NBOARDS};
use awari::awari::Awari;
use awari::retrograde::analysis;

fn main() {
    println!("PITS={} SEEDS={} NBOARDS={}", PITS, SEEDS, NBOARDS);
    let table = analysis();
    for n in 0..25 {
        println!("configurations with {} seeds", n);
        for u in Awari::iter_config(n) {
            let id = u.encode();
            println!("{:10}: {}", id, table[id].0);
        }
    }
}
