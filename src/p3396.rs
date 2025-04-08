use std::collections::HashSet;

pub fn run() {
    for i in [
        vec![1,2,3,4,2,3,3,5,7],
        vec![4,5,6,4,4],
        vec![6,7,8,9]
    ] {
        println!("{}", minimum_operations(i));
    }
}

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut map = HashSet::new();

    let mut collision = -1;

    for i in nums.iter().enumerate().rev() {
        if !map.insert(*i.1) {
            collision = i.0 as i32;
            break;
        }
    }

    (((collision + 1) as f32) / 3f32).ceil() as i32
}