#[inline]
pub fn sort(nums: &mut [u32]) {
    let mut num_bits = 0;
    let mut max = *nums.iter().max().unwrap();
    while max > 0 {
        msd_radix_sort_intern(nums, num_bits);
        num_bits += 1;
        max >>= 8;
    }
}

fn msd_radix_sort_intern(nums: &mut [u32], num_bits: u32) {
    let mut count = [0; 1 << 8];
    let mask = (1 << num_bits) - 1;

    for &item in nums.iter() {
        count[((item >> (num_bits * 8)) & mask) as usize] += 1;
    }

    let mut pos = [0; 1 << 8];
    for i in 1..(1 << 8) {
        pos[i] = pos[i - 1] + count[i - 1];
    }

    let mut temp_nums = nums.to_owned();
    for &x in nums.iter() {
        let bucket = ((x >> (num_bits * 8)) & mask) as usize;
        temp_nums[pos[bucket]] = x;
        pos[bucket] += 1;
    }

    nums.copy_from_slice(&temp_nums);
}