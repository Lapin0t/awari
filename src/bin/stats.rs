extern crate rand;
extern crate awari;


use std::iter::Iterator;
use std::env;

use rand::{Rng,thread_rng};

use awari::{FPITS,SEEDS};
use awari::awari::Awari;
use awari::utils::n_boards;



fn rand_awari<R: Rng>(g: &mut R, n: u8) -> Awari {
    let mut b: Awari = Default::default();

    b[0] = 0;
    for i in 1..FPITS {
        b[i] = g.gen_range(0, n);
    }
    b.sort();
    for i in 0..FPITS-1 {
        b[i] = b[i+1] - b[i];
    }
    b[FPITS-1] = n - b[FPITS-1];
    return b;
}


fn mean<R,F,T>(gen: &mut R, seeds: u8, n: usize, mut f: F) -> f64
    where R: Rng,
          F: FnMut(Awari) -> T,
          T: Into<f64> {
    let mut tot = 0.0;
    for _ in 0..n {
        tot += f(rand_awari(gen, seeds)).into();
    }
    return tot / (n as f64);
        
}

fn mean_gain<R: Rng>(gen: &mut R, seeds: u8, n: usize) -> f64 {
    let mut x = 0;
    let mut cnt = 0usize;
    for _ in 0..n {
        let u = rand_awari(gen, seeds);
        for (_, k) in u.successors() {
            cnt += 1;
            x += k as usize;
        }
    }
    return (x as f64) / (cnt as f64);
}


fn main() {
    let niters = env::args().nth(1).unwrap().parse::<usize>().unwrap();
    println!("{} samples per measure", niters);
    println!("====== ====== ====== ======");
    println!("  gain  moves  nz-mv  bk-mv");
    println!("====== ====== ====== ======");
    let mut gen = thread_rng();
    let mut cost = 0.0;

    for n in 1..SEEDS+1 {
        let s = n as u8;
        let g = mean_gain(&mut gen, s, niters);
        let mv = mean(&mut gen, s, niters, |u| u.successors().len() as f64);
        let nzm = mean(&mut gen, s, niters, |u| u.successors().into_iter()
                .filter(|&(_, k)| k > 0).count() as f64);
        let bm = mean(&mut gen, s, niters, |u| u.predecessors().len() as f64);

        if n != SEEDS-1 {
            cost += (2.0 + ((n+1)/2) as f64 + bm + nzm) * (n_boards(n) as f64);
        }
        println!("{:6.4} {:6.4} {:6.4} {:6.4}", g, mv, nzm, bm);
    }
    println!("====== ====== ====== ======");
    println!("estimated memory accesses: {}", cost);
}
