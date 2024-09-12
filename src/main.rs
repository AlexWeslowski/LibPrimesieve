#![allow(dead_code)]
#![allow(unused_imports)]
use libc::c_longlong;
use libprimesieve;
use std::env;
use std::mem;
use std::string::String;
use std::time::Instant;
use std::vec::Vec;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <integer1> <integer2>", args[0]);
        return;
    }
    let i1: u64 = match args[1].parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Error: First argument must be a positive integer");
            return;
        }
    };
    let i2: u64 = match args[2].parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Error: Second argument must be a positive integer");
            return;
        }
    };
    let start = Instant::now();
    let vec = libprimesieve::primes_i64(i1, i2);
    for iprime in &vec {
        println!("{}", iprime);
    }
    let duration = start.elapsed();
    println!("Generated {} prime numbers in {:.4} seconds", vec.len(), duration.as_secs_f64());
}

