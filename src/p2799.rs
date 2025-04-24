use std::collections::{HashMap, HashSet};

pub fn run() {
    for i in [
        vec![1,3,1,2,2],
        vec![5,5,5,5]
    ] {
        println!("{}", count_complete_subarrays(i));
    }
}

pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    let mut target_count = 0;
    let mut set = HashSet::new();

    for n in &nums {
        if set.insert(n) {
            target_count += 1;
        }
    }

    let mut result = 0;
    let mut left = 0;
    let mut container = HashMap::new();

    for right in 0..nums.len() {
        *container.entry(nums[right]).or_insert(0) += 1;
        while container.len() == target_count {
            let a = container.get_mut(&nums[left]).unwrap();
            *a -= 1;
            if *a == 0 {
                container.remove(&nums[left]);
            }

            left += 1;
        }
        result += left;
    }

    result as i32
}