use std::collections::HashSet;

pub fn run() {
    for i in [
        vec![1,2,2],
        vec![0],
        vec![4,4,4,1,4]
        // [[],[1],[1,4],[1,4,4],[1,4,4,4],[1,4,4,4,4],[4],[4,4],[4,4,4],[4,4,4,4]]
    ] {
        println!("{:?}", subsets_with_dup(i));
    }
}

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut result = vec![];
    let mut map = HashSet::new();

    let length = nums.len();

    for iteration in 0..2i32.pow(length as u32) {
        let mut bucket = vec![];
        for i in 0..length {
            if (iteration >> i) & 1 == 1 {
                bucket.push(nums[i]);
            }
        }
        if map.insert(bucket.clone()) {
            result.push(bucket);
        }
    }

    result
}