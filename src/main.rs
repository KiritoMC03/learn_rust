#![feature(custom_test_frameworks)]
#![feature(test)]
#![feature(is_sorted)]

#[allow(dead_code)]

extern crate rand;

mod mods;
use mods::my_sorts:: { radix, fast_radix, msd_radix };
use rand:: { Rng, seq::SliceRandom, thread_rng };
use std::ops::Range;
use std::time::Instant;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const BUCKETS_NUM : usize = 10;

const VAL: usize = 1_000_000;
const LEN: usize = 500_000_000;
const MAX_VAL : u32 = 100_000_000;
//const TESTS: usize = 10;

fn main() {
    let v_cap = 170_000_000;
    let mut buckets : [Vec<u32>; BUCKETS_NUM] = [Vec::with_capacity(v_cap), Vec::with_capacity(v_cap), Vec::with_capacity(v_cap),
                                                 Vec::with_capacity(v_cap), Vec::with_capacity(v_cap), Vec::with_capacity(v_cap), Vec::with_capacity(v_cap),
                                                 Vec::with_capacity(v_cap), Vec::with_capacity(v_cap), Vec::with_capacity(v_cap), ];

    //let mut buckets_heap: Vec<BinaryHeap<u32>> = vec![BinaryHeap::with_capacity(v_cap); 10];

    println!("fill 1");
    let mut lists_1: Vec<u32> = get_test_list(0..MAX_VAL, LEN);
    let mut lists_2 = lists_1.clone();
    /*println!("fill 3");
    let mut lists_3 = lists_1.clone();*/
/*
    let before = Instant::now();
    radix::radix_sort(&mut lists_1, &mut buckets);
    println!("radix_sort {:?}", before.elapsed());
    println!("sorted: {:?}", lists_1.is_sorted());

    for b in buckets.iter_mut() {
        b.clear();
    }
*/
    println!("start");
    let before = Instant::now();
    lists_1.sort();
    //msd_radix::sort(&mut lists_1, &mut buckets);
    println!("msd_radix {:?}", before.elapsed());
    println!("sorted: {:?}", lists_1.is_sorted());

    println!("start");
    let before = Instant::now();
//    msd_radix::radix_sort(&mut lists_2);
//    msd_radix::sort(&mut lists_2, &mut buckets);
    radix::radix_sort(&mut lists_2, &mut buckets);
    println!("msd_radix_sort {:?}", before.elapsed());
    println!("sorted: {:?}", lists_2.is_sorted());

/*
    let before = Instant::now();
    lists_3.sort();
    println!("sort {:?}", before.elapsed());
    println!("sorted: {:?}", lists_3.is_sorted());
    */

    return;
/*

    println!("fill 2");
    let mut lists_2: Box<Vec<i32>> = lists_1.clone();
    println!("fill 3");
    let mut lists_3: Box<Vec<i32>> = lists_1.clone();


    println!("buckets alloc");


    println!("fast alloc");
    fast_radix::alloc_temp(LEN + 1);
    println!("start");

    for _ in 0..5 {

        let before = Instant::now();
        fast_radix::f_lsd_sort(&mut lists_1);
        println!("fast_radix {:?}", before.elapsed());

        println!("sorted: {:?}", lists_1.is_sorted());

        let before = Instant::now();
        println!("sort {:?}", before.elapsed());

        println!("sorted: {:?}", lists_2.is_sorted());

        let before = Instant::now();
        radix::radix_sort(&mut lists_3, &mut buckets);
        println!("radix_sort {:?}", before.elapsed());

        println!("sorted: {:?}", lists_3.is_sorted());
        println!("");
        println!("");
        println!("");
        let mut rng = thread_rng();
        lists_1.shuffle(&mut rng);
        lists_2 = lists_1.clone();
        lists_3 = lists_1.clone();
    }
*/
    // huge_reduction(make_test);
}
/*
fn huge_reduction(large_func: fn()) {
    let builder = std::thread::Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024 * 32); // 32MB of stack space

    let handler = builder.spawn(large_func).unwrap();
    handler.join().unwrap();
}
*/
/*
fn make_test() {
    let mut lists_1 = [get_test_list(0..VAL as i32); TESTS];
    let mut lists_2 = lists_1.clone();

    let mut i = 0;
    while i < lists_1.len() {
        radix::lsd_sort(lists_1.get_mut(i).unwrap());
        fast_radix::f_lsd_sort(lists_2.get_mut(i).unwrap());
        i += 1;
    }

    lists_1 = [get_test_list(0..i32::MAX); TESTS];
    lists_2 = lists_1.clone();

    i = 0;
    let mut before = Instant::now();
    while i < lists_1.len() {
        radix::lsd_sort(lists_1.get_mut(i).unwrap());
        i += 1;
    }
    println!("lsd_sort: {:?}", before.elapsed());
    i = 0;
    before = Instant::now();
    while i < lists_2.len() {
        fast_radix::f_lsd_sort(lists_2.get_mut(i).unwrap());
        i += 1;
    }
    println!("f_lsd_sort: {:?}", before.elapsed());
}
*/

fn get_test_list(range: Range<u32>, len: usize) -> Vec<u32> {
    let mut list : Vec<u32> = Vec::with_capacity(len);
    for _ in 0..len {
        list.push( rand::thread_rng().gen_range(range.clone()) );
    }

    list
}





extern crate test;
use test::Bencher;
/*
const F : usize = 100_000;
const T : usize = 100;
const TT : usize = 100;

#[bench]
fn a0_burn_test(b: &mut Bencher) {
    let arr : [i8; F] = [1; F];
    let mut c = 1i8;

    b.iter(|| {
        for _ in 0..TT {
            for v in arr.iter() {
                c *= v;
            }
        }
    });

    assert_eq!(c, 1i8);
}

#[bench]
fn a1_stack_test(b: &mut Bencher) {
    let arr : [i8; F] = [1; F];
    let mut c = 1i8;

    b.iter(|| {
        for _ in 0..T {
            for v in arr.iter() {
                c *= v;
            }
        }
    });

    assert_eq!(c, 1i8);
}

#[bench]
fn a2_heap_test(b: &mut Bencher) {
    let arr : Vec<i8> = vec![1; F];
    let mut c = 1i8;

    b.iter(|| {
        for _ in 0..T {
            for v in arr.iter() {
                c *= v;
            }
        }
    });

    assert_eq!(c, 1i8);
}

#[bench]
fn a3_stack_box_test(b: &mut Bencher) {
    let arr : Box<[i8; F]> = Box::new([1; F]);
    let mut c = 1i8;

    b.iter(|| {
        for _ in 0..T {
            for v in arr.iter() {
                c *= v;
            }
        }
    });

    assert_eq!(c, 1i8);
}
*/

/*
#[bench]
fn a1_burn_bench_workload(b: &mut Bencher) {
    return;
    let mut lists_1 = get_test_list(0..MAX_VAL as i32, LEN);
    b.iter(|| {
        radix::lsd_sort(&mut lists_1);
        fast_radix::f_lsd_sort(&mut lists_1);
    });
}

#[bench]
fn a2_base_radix(b: &mut Bencher) {
    return;
    let mut lists_1 = get_test_list(0..MAX_VAL as i32, LEN);
    b.iter(|| {radix::radix_sort(&mut lists_1);});
}

#[bench]
fn a3_fast_radix(b: &mut Bencher) {
    let mut lists_1 = get_test_list(0..MAX_VAL as i32, LEN);
    b.iter(|| {fast_radix::f_lsd_sort(&mut lists_1);});
}
#[bench]
fn a4_def_sort(b: &mut Bencher) {
    let mut lists_1 = get_test_list(0..MAX_VAL as i32, LEN);
    b.iter(|| {
        lists_1.shuffle(&mut thread_rng());
        lists_1.sort();
    });
}
*/