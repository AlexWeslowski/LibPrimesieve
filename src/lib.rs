#![allow(dead_code)]
#![allow(unused_imports)]
use libc::{c_short, c_ushort, c_int, c_uint, c_long, c_ulong, c_longlong, c_ulonglong, c_void, size_t, free};
use std::clone;
use std::collections::HashMap;
use std::mem;
use std::slice;
use std::time::Instant;


pub enum PrimeType {
    Short,
    Ushort,
    Int,
    Uint,
    Long,
    Ulong,
    Longlong,
    Ulonglong,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
}

#[link(name = "primesieve")]
extern {
    fn primesieve_generate_primes(start: c_ulong, stop: c_ulong, size: *mut size_t, r#type: c_int) -> *mut c_void;
    fn primesieve_generate_n_primes(n: c_ulong, start: c_ulong, r#type: c_int);
    fn primesieve_free();
}

macro_rules! generate_primes {
($func_name:ident, $int_type:ty, $uint_type:ty, $type:expr) => {
        pub fn $func_name(start: $uint_type, stop: $uint_type) -> Vec<$int_type> {
            let mut usz: usize = 0;
            let r#type = $type;
            let vec: Vec<$int_type>;
            unsafe {
                let ptr = primesieve_generate_primes(start as u64, stop as u64, &mut usz, r#type);
                vec = slice::from_raw_parts(ptr as *const $int_type, usz).to_vec();
                free(ptr as *mut c_void);
            }
            return vec;
        }
    }
}

pub fn primes<T: Clone>(start: u64, stop: u64, prime_type: PrimeType) -> Vec<T> {
    let mut usz: usize = 0;
    let r#type = match prime_type {
        PrimeType::Short => 0,
        PrimeType::Ushort => 1,
        PrimeType::Int => 2,
        PrimeType::Uint => 3,
        PrimeType::Long => 4,
        PrimeType::Ulong => 5,
        PrimeType::Longlong => 6,
        PrimeType::Ulonglong => 7,
        PrimeType::Int16 => 8,
        PrimeType::Uint16 => 9,
        PrimeType::Int32 => 10,
        PrimeType::Uint32 => 11,
        PrimeType::Int64 => 12,
        PrimeType::Uint64 => 13,
    };
    let vec: Vec<T>;
    unsafe {
        let ptr = primesieve_generate_primes(start, stop, &mut usz, r#type);
        vec = slice::from_raw_parts(ptr as *const T, usz).to_vec();
        free(ptr as *mut c_void);
    }
    return vec;
}

generate_primes!(primes_short, c_short, c_ushort, 0);
generate_primes!(primes_ushort, c_ushort, c_ushort, 1);
generate_primes!(primes_int, c_int, c_uint, 2);
generate_primes!(primes_uint, c_uint, c_uint, 3);
generate_primes!(primes_long, c_long, c_ulong, 4);
generate_primes!(primes_ulong, c_ulong, c_ulong, 5);
generate_primes!(primes_longlong, c_longlong, c_ulonglong, 6);
generate_primes!(primes_ulonglong, c_ulonglong, c_ulonglong, 7);
generate_primes!(primes_i16, i16, u16, 8);
generate_primes!(primes_u16, u16, u16, 9);
generate_primes!(primes_i32, i32, u32, 10);
generate_primes!(primes_u32, u32, u32, 11);
generate_primes!(primes_i64, i64, u64, 12);
generate_primes!(primes_u64, u64, u64, 13);
    


