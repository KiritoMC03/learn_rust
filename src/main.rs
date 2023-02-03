mod mods;
use mods::my_sorts::radix;
use mods::my_sorts::fast_radix;
use rand::Rng;
use std::ops::Range;
use std::time::Instant;


const VAL: usize = 10_000_000;
const TESTS: usize = 1;

fn main() {
    huge_reduction(make_test);
}

fn huge_reduction(large_func: fn()) {
    let builder = std::thread::Builder::new()
        .name("reductor".into())
        .stack_size(32 * 1024 * 1024 * 32); // 32MB of stack space

    let handler = builder.spawn(large_func).unwrap();
    handler.join().unwrap();
}

fn make_test() {
    let mut lists_1 = [get_test_list(0..VAL as i32); TESTS];
    let mut lists_2 = lists_1.clone();

    let mut i = 0;
    while i < lists_1.len() {
        radix::lsd_sort(lists_1.get_mut(i).unwrap());
        fast_radix::f_lsd_sort(lists_2.get_mut(i).unwrap());
        i += 1;
    }

    lists_1 = [get_test_list(0..1000); TESTS];
    lists_2 = lists_1.clone();

    i = 0;
    let mut before = Instant::now();
    while i < lists_1.len() {
        radix::lsd_sort(lists_1.get_mut(i).unwrap());
        i += 1;
    }
    println!("{:?}", before.elapsed());
    i = 0;
    before = Instant::now();
    while i < lists_2.len() {
        fast_radix::f_lsd_sort(lists_2.get_mut(i).unwrap());
        i += 1;
    }
    println!("{:?}", before.elapsed());
}

fn get_test_list(range: Range<i32>) -> [i32; VAL] {
    let mut list = [0i32; VAL];
    for i in 0..VAL {
        list[i] = rand::thread_rng().gen_range(range.clone());
        if list[i] < 0 {
            list[i] *= -1
        }
    }

    list
}
