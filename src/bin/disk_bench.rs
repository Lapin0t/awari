#![feature(test,placement_in_syntax,box_heap)]

extern crate rand;
extern crate test;
extern crate time;

use std::boxed::HEAP;
use std::fs::{File,OpenOptions};
use std::io::{Read,Write,SeekFrom,Seek};
use time::Duration;
use std::env;

use rand::weak_rng;
use rand::distributions::{Range,IndependentSample};
use test::black_box;



fn run_n<F,T>(n: usize, mut f: F) -> f64 where F: FnMut() -> T {
    let mut runs = [0.; 11];
    for r in runs.iter_mut() {
        let d = Duration::span(|| { for _ in 0..n { black_box(f()); }});
        *r = d.num_nanoseconds().unwrap() as f64;
    }
    runs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return runs[5] / n as f64;
}

fn bench(fd: &mut File, bl: usize, fl: usize, n: usize) -> (f64, f64) {
    let mut buf = vec![0u8; bl];
    fd.seek(SeekFrom::Start(0)).unwrap();
    fd.set_len(fl as u64).unwrap();

    let mut g = weak_rng();
    let dist = Range::new(0, (fl - bl) as u64);

    let read = run_n(n, || {
        fd.seek(SeekFrom::Start(dist.ind_sample(&mut g))).unwrap();
        fd.read_exact(&mut buf).unwrap();
    });
    let write = run_n(n, || {
        fd.seek(SeekFrom::Start(dist.ind_sample(&mut g))).unwrap();
        fd.write(&buf).unwrap();
    });

    return (read, write);
}

/*fn bench_write(fd: &mut File, write_len: usize, file_len: usize) -> BenchSamples {
    let buf = vec![123u8; write_len];
    fd.seek(SeekFrom::Start(0)).unwrap();
    fd.set_len(file_len as u64).unwrap();
    let mut g = thread_rng();
    let dist = Range::new(0, (file_len - write_len) as u64);

    return benchmark(|b| {
        let mut n = 0;
        b.iter(|| {
            n += 1;
            fd.seek(SeekFrom::Start(dist.ind_sample(&mut g))).unwrap();
            fd.write(&buf).unwrap();
        });
        b.bytes = write_len as u64;
    });
}*/

const EXTS: &'static str = " KMGT";
fn fmt(a: f64) -> String {
    let lg = (a.log2() / 10.).floor().max(0.);
    format!("{:5.1}{}", a / (10.*lg).exp2(), EXTS.chars().nth(lg as usize).unwrap())
}

fn main() {
    let n: usize = env::args().nth(1).unwrap().parse().unwrap();
    let fl: usize = 1 << env::args().nth(2).unwrap().parse::<usize>().unwrap();

    // exhaust RAM to slow down disk caching
    let big_thing = HEAP <- [0u8; 12*1024*1024*1024];
    black_box(big_thing[0]);

    let mut fd = OpenOptions::new().read(true).write(true).create(true)
        .open("tmp.tmp").unwrap();

    println!("file length: {}", fmt(fl as f64));
    for b in 10..28 {
        let bl = 1 << b;
        let (r, w) = bench(&mut fd, bl, fl, n / (b*b));
        let x = 1_000_000_000. * bl as f64;
        println!("{} {:15.3} ({}/s) {:15.3} ({}/s)",
                 fmt(bl as f64), r, fmt(x / r), w, fmt(x / w));
    }
}
