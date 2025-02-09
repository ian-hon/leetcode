use std::{collections::HashMap, fs};

pub fn run() {
    for i in [
        vec![1,2,3,4,5],
        vec![4,1,3,3],
        serde_json::from_str::<Vec<i32>>(&fs::read_to_string("./src/too_damn_long.txt").unwrap()).unwrap()
    ] {
        println!("{}", count_bad_pairs(i));
    }
}

pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let mut total = 0;
    let length = nums.len();
    let mut map = HashMap::new();

    for (index, n) in nums.iter().enumerate() {
        let d = index as i32 - *n;
        match map.get_mut(&d) {
            Some(i) => {
                total += *i;
                *i += 1;
            },
            None => {
                map.insert(d, 1);
            }
        }
    }

    ((length * (length - 1)) / 2) as i64 - total
}
