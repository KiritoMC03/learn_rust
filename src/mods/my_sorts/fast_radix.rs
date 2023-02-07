use libm::{log10, powf};

const BUCKETS_NUM : usize = 10;
const VEC_CAP : usize = 100;


#[allow(dead_code)]
pub fn f_lsd_sort(nums: &mut [i32], buckets: &mut [Vec<i32>; BUCKETS_NUM]) {
    // if is_sorted(nums) {return}

    let max_len = get_max_length(nums);
    let len = nums.len();
    let mut step = 0;
    let mut item : i32;
    let mut bucket_index : usize;
    let mut ost;
    let mut div;

    let mut index_0;
    let mut index_1;
    let mut index_2;
    let mut index_3;

    while step < max_len{
        ost = powf(10f32, step as f32 + 1f32) as usize;
        div = powf(10f32, step as f32) as usize;

        index_0 = 0;
        index_1 = 1;
        index_2 = 2;
        index_3 = 3;
        while index_0 < len && index_3 < len {
            item = nums[index_0];
            bucket_index = item as usize % ost / div;
            buckets[bucket_index].push(item);

            item = nums[index_1];
            bucket_index = item as usize % ost / div;
            buckets[bucket_index].push(item);

            item = nums[index_2];
            bucket_index = item as usize % ost / div;
            buckets[bucket_index].push(item);

            item = nums[index_3];
            bucket_index = item as usize % ost / div;
            buckets[bucket_index].push(item);

            index_0 += 4;
            index_1 += 4;
            index_2 += 4;
            index_3 += 4;
        }

        if index_0 < len {
            item = nums[index_0];
            bucket_index = item as usize % ost / div;
            buckets[bucket_index].push(item);
        }
        if index_1 < len {
            item = nums[index_1];
            bucket_index = item as usize % ost / div;
            buckets[bucket_index].push(item);
        }
        if index_2 < len {
            item = nums[index_2];
            bucket_index = item as usize % ost / div;
            buckets[bucket_index].push(item);
        }

        index_0 = 0;

        for cur_bucket in buckets.iter_mut() {
            for bucket_item in cur_bucket.iter_mut() {
                nums[index_0] = *bucket_item;
                index_0 += 1;
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

const M : i32 = 999999999;
fn get_max_length(nums: &mut [i32]) -> u32 {
    let mut max = i32::MIN;
    let len = nums.len();
    let mut i = 0;
    let mut cur;

    while i < len {
        cur = nums[i];
        if cur > max {max = cur};
        if max > M {break;}
        i += 1;
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