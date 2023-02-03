use std::i32;

use libm::{log10, powf};

const BUCKETS_NUM : usize = 10;

#[allow(dead_code)]
pub fn f_digits_sort (nums: &mut [i32]) {
    if is_sorted(nums) {return}
    let mut buckets: [i32; 10] = [0; 10];

    for i in nums.iter_mut() {
        buckets[*i as usize] += 1;
    }

    let mut index : usize = 0;
    let mut digit: i32 = 0;
    for repeats in buckets {
        for _ in 0..repeats {
            nums[index] = digit;
            index += 1;
        }

        digit += 1;
    }
}

#[allow(dead_code)]
pub fn f_lsd_sort(nums: &mut [i32]) {
    if is_sorted(nums) {return}

    let max_len = get_max_length(nums);
    let len = nums.len();
    let mut step = 0;
    let mut buckets : [Vec<i32>; BUCKETS_NUM] = Default::default();
    let mut item : i32;
    let mut bucket_index : usize;

    while step < max_len{
        let mut temp_index = 0;
        while temp_index < len {
            item = nums[temp_index];
            bucket_index = item as usize % powf(10f32, step as f32 + 1f32) as usize / powf(10f32, step as f32) as usize;
            buckets.get_mut(bucket_index).unwrap().push(item);
            temp_index += 1;
        }

        temp_index = 0;
        for cur_bucket in buckets.iter_mut() {
            for bucket_item in cur_bucket.iter_mut() {
                nums[temp_index] = *bucket_item;
                temp_index += 1;
            }
            cur_bucket.clear();
        }

        step += 1;
    }
}

fn is_sorted(nums: &mut [i32]) -> bool {
    let len = nums.len();
    if len < 2 { return true }
    let mut index : usize = 1;
    let mut is_sorted = true;
    while index < len {
        if nums[index] < nums[index - 1] {
            is_sorted = false;
            break;
        }
        index += 1;
    }

    is_sorted
}

fn get_max_length(nums: &mut [i32]) -> u32 {
    let mut max = i32::MIN;
    for i in nums {
        if i > &mut max {max = *i};
    }
    log10(max as f64) as u32 + 1u32
}




/*
for cur_bucket in &mut buckets {
    for bucket_item in &mut *cur_bucket {
        nums[temp_index] = *bucket_item;
        temp_index += 1;
    }
    cur_bucket.clear();
}
*/