pub mod radix;
pub mod fast_radix;
pub mod msd_radix;

/*
extern crate test;
use test::Bencher;

// #[bench]
fn bench_workload(b: &mut Bencher) {
    let mut g = 0;
    b.iter(|| {
        let mut c = test::black_box(0);
        for i in 0..100000000 {
            c += i * 0 + 1;
        }
        g += c;
    });

    println!("{}", g);
}*/
/*
fn bench_workload_b(b: &mut Bencher) {
    b.iter(|| {
        let mut c = 0;
        for i in 0..100 {
            c += 1;
        }
    });
}
*/