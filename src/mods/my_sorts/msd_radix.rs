#[inline]

pub fn sort(arr: &mut [u32], buckets: &mut [Vec<u32>; 10]) {
    let mut exp = 1;
    let max = *arr.iter().max().unwrap();
    let mut start : usize;

    while max / exp > 0 {
        for &x in arr.iter() {
            let digit = (x / exp) % 10;
            buckets[digit as usize].push(x);
        }

        start = 0;
        for bucket in buckets.iter_mut() {
            arr[start..(start+bucket.len())].copy_from_slice(bucket);
            start += bucket.len();
            bucket.clear();
        }

        exp *= 10;
    }
}




fn msd_radix_sort_intern(arr: &mut [u32], num_bits: u32) {
    let mut count = [0; 1 << 8];
    let mask = (1 << num_bits) - 1;

    for &x in arr.iter() {
        count[((x >> (num_bits * 8)) & mask) as usize] += 1;
    }

    let mut pos = [0; 1 << 8];
    for i in 1..(1 << 8) {
        pos[i] = pos[i - 1] + count[i - 1];
    }

    let mut temp = arr.to_owned();
    for &x in arr.iter() {
        let bucket = ((x >> (num_bits * 8)) & mask) as usize;
        temp[pos[bucket]] = x;
        pos[bucket] += 1;
    }

    arr.copy_from_slice(&temp);
}

pub fn radix_sort(arr: &mut [u32]) {
    let mut num_bits = 0;
    let mut max = *arr.iter().max().unwrap();
    while max > 0 {
        msd_radix_sort_intern(arr, num_bits);
        num_bits += 1;
        max >>= 8;
    }
}
