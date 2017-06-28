extern crate awari;

use awari::awari::{Awari,N};
use awari::retrograde::analysis;

fn main() {
    println!("start");
    println!("N={}", N);
    let table = analysis();
    for n in 0..25 {
        println!("configurations with {} seeds", n);
        for u in Awari::iter_config(n) {
            let id = u.encode();
            println!("{:10}: {}", id, table[id].0);
        }
    }
}
