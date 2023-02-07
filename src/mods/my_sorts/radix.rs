use libm::{log10, powf};

const BUCKETS_NUM : usize = 10;

#[inline]
pub fn radix_sort(nums: &mut [u32], buckets: &mut [Vec<u32>; BUCKETS_NUM]) {
    // if is_sorted(nums) {return}

    let max_val = *nums.iter().max().unwrap();
    let steps = log10(max_val as f64) as u32 + 1;
    let mut bucket_index : usize;

    for step in 0..steps {
        let div = powf(10f32, step as f32 + 1f32) as usize;
        let ost = powf(10f32, step as f32) as usize;
        for item in nums.iter() {
            bucket_index = *item as usize % div / ost;
            buckets[bucket_index].push(*item);
        }

        let mut temp_index = 0;

        for cur_bucket in buckets.iter_mut() {
            for bucket_item in cur_bucket.iter_mut() {
                nums[temp_index] = *bucket_item;
                temp_index += 1;
            }
            cur_bucket.clear();
        }
    }
}