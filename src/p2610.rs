use std::collections::HashMap;

pub fn run() {
    for i in [
        vec![1,3,4,1,2,3,1],
        vec![1,2,3,4]
    ] {
        println!("{:?}", find_matrix(i));
    }
}

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut result = vec![];

    let mut map = HashMap::new();

    for n in nums {
        let mut index = 0;

        match map.get_mut(&n) {
            Some(p) => {
                *p += 1;
                index = *p as usize - 1;
            },
            None => {
                map.insert(n, 1);
            }
        }

        if index >= result.len() {
            result.push(vec![]);
        }

        println!("{index} : {n}");

        result[index].push(n);
    }

    result
}