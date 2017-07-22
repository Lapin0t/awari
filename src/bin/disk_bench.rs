#![feature(test)]

extern crate rand;
extern crate test;
extern crate tempfile;

use rand::thread_rng;
use rand::distributions::{Range,IndependentSample};
use std::fs::{OpenOptions,File};
use std::io::{Read,Write,SeekFrom,Seek};
use tempfile::tempfile;
use test::{black_box,BenchSamples,fmt_bench_samples};
use test::bench::benchmark;


fn bench_read(fd: &mut File, read_len: usize, file_len: usize) -> BenchSamples {
    let mut buf = vec![0u8; read_len];
    fd.seek(SeekFrom::Start(0)).unwrap();
    fd.set_len(file_len as u64).unwrap();
    let mut g = thread_rng();
    let dist = Range::new(0, (file_len - read_len) as u64);

    return benchmark(|b| {
        let mut n = 0;
        b.iter(|| {
            n += 1;
            fd.seek(SeekFrom::Start(dist.ind_sample(&mut g))).unwrap();
            fd.read_exact(&mut buf).unwrap();
        });
        b.bytes = read_len as u64;
    });
}

fn bench_write(fd: &mut File, write_len: usize, file_len: usize) -> BenchSamples {
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
}

fn main() {
    let mut fd = OpenOptions::new().read(true).write(true).create(true)
        .open("test_tmp").unwrap();
    let (k, m, g) = (1024, 1024*1024, 1024*1024*1024);
    for &block in [1, 2, 10, 100, 512, k, 2*k].iter() {
        for &file in [10*k, m, g, 20*g].iter() {
            println!("file: {}, block: {}", file, block);
            println!("reading: {}", fmt_bench_samples(&bench_read(
                &mut fd, block, file)));
            println!("writing: {}", fmt_bench_samples(&bench_write(
                &mut fd, block, file)));
        }
    }
    /*for &block in [1*m, 10*m, 512*m, g, 2*g].iter() {
        println!("file: 6G, block: {}", block);
        println!("reading: {}", fmt_bench_samples(&bench_read(&mut fd, block, 6*g)));
        println!("writing: {}", fmt_bench_samples(&bench_write(&mut fd, block, 6*g)));
    }*/
}
