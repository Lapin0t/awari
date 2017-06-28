use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;


fn binom(k: usize, n: usize) -> usize {
    if n < k {
        return 0;
    }
    let mut p = 1;
    for i in 0..k {
        p *= n - i;
        p /= i + 1;
    }
    return p;
}


fn main() {
    println!("cargo:rerun-if-env-changed=AWARI_PITS");
    println!("cargo:rerun-if-env-changed=AWARI_SEEDS");

    let board = match env::var("AWARI_PITS") {
        Ok(v) => v.parse::<usize>().unwrap(),
        Err(_) => 6,
    };
        
    let fboard = 2 * board;

    let seeds = match env::var("AWARI_SEEDS") {
        Ok(v) => v.parse::<usize>().unwrap(),
        Err(_) => 4,
    };

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("size.rs");
    let mut file = File::create(&path).unwrap();
    
    writeln!(&mut file, "pub const PITS: usize = {};", board).unwrap();
    writeln!(&mut file, "pub const FPITS: usize = {};", fboard).unwrap();
    writeln!(&mut file, "pub const START_SEEDS: usize = {};", seeds).unwrap();
    writeln!(&mut file, "pub const SEEDS: usize = {};", fboard * seeds).unwrap();
    writeln!(&mut file, "pub const NBOARDS: usize = {};",
             binom(fboard, fboard * (1 + seeds))).unwrap();
}
