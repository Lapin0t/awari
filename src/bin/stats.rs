extern crate rand;
extern crate awari;


use std::iter::Iterator;
use std::env;

use rand::{Rng,thread_rng};

use awari::NBOARDS;
use awari::awari::Awari;


fn rand_awari<R: Rng>(gen: &mut R) -> Awari {
    Awari::decode(gen.gen_range(0, NBOARDS))
}

fn mean_gain<R: Rng>(gen: &mut R, n: usize) -> f64 {
    let mut x = 0;
    let mut cnt = 0;
    for _ in 0..n {
        let u = rand_awari(gen);
        for (_, k) in u.successors() {
            cnt += 1;
            x += k as usize;
        }
    }
    return (x as f64) / (cnt as f64);
}


fn mean_nz_move<R: Rng>(gen: &mut R, n: usize) -> f64 {
    let mut x = 0;
    for _ in 0..n {
        let u = rand_awari(gen);
        for (_, k) in u.successors() {
            if k > 0 {
                x += 1;
            }
        }
    }
    return (x as f64) / (n as f64);
}

fn mean_moves<R: Rng>(gen: &mut R, n: usize) -> f64 {
    let mut x = 0;
    for _ in 0..n {
        let u = rand_awari(gen);
        x += u.successors().len();
    }
    return (x as f64) / (n as f64);
}

fn mean_predecessors<R: Rng>(gen: &mut R, n: usize) -> f64 {
    let mut x = 0;
    for _ in 0..n {
        let u = rand_awari(gen);
        x += u.predecessors().len();
    }
    return (x as f64) / (n as f64);
}


fn main() {
    let n = env::args().nth(1).unwrap().parse::<usize>().unwrap();
    println!("{} samples", n);
    let mut gen = thread_rng();
    println!("mean gain: {}", mean_gain(&mut gen, n));
    println!("mean moves: {}", mean_moves(&mut gen, n));
    println!("mean non-zero moves: {}", mean_nz_move(&mut gen, n));
    println!("mean back-moves: {}", mean_predecessors(&mut gen, n));
}
