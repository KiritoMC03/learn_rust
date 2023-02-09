#![feature(custom_test_frameworks)]
#![feature(test)]
#![feature(is_sorted)]

extern crate rand;

mod mods;
use mods::radix_sort::*;
use rand::Rng;
use std::ops::Range;
use std::time::Instant;

const BUCKETS_NUM : usize = 10;
const ARR_LEN: usize = 500_000_000;
const MAX_VAL : u32 = 100_000_000;

fn main() {
    let vec_cap = 100_000_000;
    let mut buckets : [Vec<u32>; BUCKETS_NUM] = [Vec::with_capacity(vec_cap), Vec::with_capacity(vec_cap), Vec::with_capacity(vec_cap),
                                                 Vec::with_capacity(vec_cap), Vec::with_capacity(vec_cap), Vec::with_capacity(vec_cap), Vec::with_capacity(vec_cap),
                                                 Vec::with_capacity(vec_cap), Vec::with_capacity(vec_cap), Vec::with_capacity(vec_cap), ];

    let mut lists_1: Vec<u32> = get_test_list(0..MAX_VAL, ARR_LEN);
    let mut lists_2 = lists_1.clone();
    let mut lists_3 = lists_1.clone();

    println!("start std::sort");
    let before = Instant::now();
    lists_1.sort();
    println!("sort {:?}", before.elapsed());
    println!("sorted: {:?}", lists_1.is_sorted());

    println!("start msd::sort");
    let before = Instant::now();
    msd::sort(&mut lists_2);
    println!("msd::sort {:?}", before.elapsed());
    println!("sorted: {:?}", lists_2.is_sorted());

    let before = Instant::now();
    lsd::sort(&mut lists_3, &mut buckets);
    println!("lsd::sort {:?}", before.elapsed());
    println!("sorted: {:?}", lists_3.is_sorted());
}

#[allow(dead_code)]
fn huge_reduction(large_func: fn()) {
    let builder = std::thread::Builder::new()
        .name("reductor".into())
        .stack_size(1024 * 1024 * 32); // 32MB of stack space

    let handler = builder.spawn(large_func).unwrap();
    handler.join().unwrap();
}

fn get_test_list(range: Range<u32>, len: usize) -> Vec<u32> {
    let mut list : Vec<u32> = Vec::with_capacity(len);
    for _ in 0..len {
        list.push( rand::thread_rng().gen_range(range.clone()) );
    }
    return list;
}