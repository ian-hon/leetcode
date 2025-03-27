use std::collections::{HashMap, VecDeque};

pub fn run() {
    for i in [
        vec![1,2,2,2],
        vec![2,1,3,1,1,1,7,1,2,1]
    ] {
        println!("{}", minimum_index(i));
    }
}

pub fn minimum_index(nums: Vec<i32>) -> i32 {
    let length = nums.len();

    let mut left_freq: HashMap<i32, i32> = HashMap::new();
    let mut right_freq: HashMap<i32, i32> = HashMap::new();

    for n in &nums {
        match right_freq.get_mut(&n) {
            Some(r) => {
                *r += 1;
            },
            None => {
                right_freq.insert(*n, 1);
            }
        }
    }

    for (index, n) in nums.iter().enumerate() {
        match left_freq.get_mut(&n) {
            Some(l) => {
                *l += 1;
            },
            None => {
                left_freq.insert(*n, 1);
            }
        }

        *right_freq.get_mut(&n).unwrap() -= 1;

        if *left_freq.get(&n).unwrap() > ((index as i32 + 1) / 2) && *right_freq.get(&n).unwrap() > ((length - index - 1) / 2) as i32 {
            return index as i32;
        }
    }

    -1
}