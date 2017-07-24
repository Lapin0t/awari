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

    let pits = match env::var("AWARI_PITS") {
        Ok(v) => v.parse::<usize>().unwrap(),
        Err(_) => 6,
    };

    let fpits = 2 * pits;

    let seeds = match env::var("AWARI_SEEDS") {
        Ok(v) => v.parse::<usize>().unwrap(),
        Err(_) => 4,
    };

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("size.rs");
    let mut file = File::create(&path).unwrap();

    writeln!(&mut file, "pub const PITS: usize = {};", pits).unwrap();
    writeln!(&mut file, "pub const FPITS: usize = {};", fpits).unwrap();
    writeln!(&mut file, "pub const START_SEEDS: usize = {};",
             seeds / fpits).unwrap();
    writeln!(&mut file, "pub const SEEDS: usize = {};", seeds).unwrap();
    writeln!(&mut file, "pub const NBOARDS: usize = {};",
             binom(fpits, fpits + seeds) - binom(fpits, pits + seeds)
             - binom(fpits-1, fpits+seeds-2) + binom(fpits-1, pits+seeds-2)
            ).unwrap();

    write!(&mut file, "pub const BINOM_TBL: [usize; {}] = [",
             fpits * (fpits + seeds + 1)).unwrap();
    for k in 0..fpits {
        write!(&mut file, "{},", 0).unwrap();
    }
    for n in 1..fpits+seeds+1 {
        for k in 0..fpits {
            write!(&mut file, "{},", binom(k+1, n)).unwrap();
        }
    }
    writeln!(&mut file, "];").unwrap();
}
