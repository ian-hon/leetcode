use std::{cmp::max, collections::{BTreeSet, HashMap, HashSet}};

pub fn run() {
    for i in [
        ([1,5,4,2,9,9,9].to_vec(), 3),
        ([4,4,4].to_vec(), 3),
        ((0..=100000).collect(), 100000)
    ] {
        println!("{}", maximum_subarray_sum(i.0.to_vec(), i.1));
    }
}

pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut final_result = 0;
    let mut stepping_sum = 0;
    let mut stepping_hold: HashMap<i32, i32> = HashMap::new();
    let mut i = 0usize;
    while (i < k) && (i < nums.len()) {
        match stepping_hold.get_mut(&nums[i]) {
            Some(m) => { *m += 1; },
            None => { stepping_hold.insert(nums[i], 1); }
        }
        stepping_sum += nums[i] as i64;
        i += 1;
    }
    if stepping_hold.len() == k {
        final_result = stepping_sum;
    }

    while i < nums.len() {
        match stepping_hold.get_mut(&nums[i]) {
            Some(m) => { *m += 1; },
            None => { stepping_hold.insert(nums[i], 1); }
        }
        *stepping_hold.get_mut(&nums[i-k]).unwrap() -= 1;

        if *stepping_hold.get(&nums[i - k]).unwrap() == 0 {
            stepping_hold.remove(&nums[i-k]);
        }

        stepping_sum += nums[i] as i64;
        stepping_sum -= nums[i - k] as i64;

        if stepping_hold.len() == k {
            final_result = max(final_result, stepping_sum);
        }

        i += 1;
    }
    final_result
}
