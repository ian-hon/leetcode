use std::collections::HashSet;

pub fn run() {
    for i in [
        (vec![5,2,5,4,5], 2),
        (vec![2,1,2], 2),
        (vec![9,7,5,3], 1)
    ] {
        println!("{}", min_operations(i.0, i.1));
    }
}

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut map = vec![false; 100];

    for i in nums {
        if i < k {
            return -1;
        }

        if i > k && !map[i as usize] {
            count += 1;
            map[i as usize] = true;
        }
    }

    count
}