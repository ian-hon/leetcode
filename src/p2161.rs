use std::fs;

pub fn run() {
    for i in [
        (vec![9,12,5,10,14,3,10], 10),
        (vec![-3,4,3,2], 2),
        (serde_json::from_str(&fs::read_to_string("./src/too_damn_long.txt").unwrap()).unwrap(), 1)
    ] {
        println!("{:?}", pivot_array(i.0, i.1));
    }
}

pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    // keep it stupid, simple

    let mut less = vec![];
    let mut more = vec![];
    let mut equal_count = 0;

    for i in nums {
        if i < pivot {
            less.push(i);
            continue;
        }
        if i > pivot {
            more.push(i);
            continue;
        }
        equal_count += 1;
    }

    let mut result = vec![];

    result.append(&mut less);
    result.append(&mut vec![pivot; equal_count]);
    result.append(&mut more);

    result

    // let mut nums = nums;

    // let mut ten_indices = vec![];

    // let mut offset = 0;
    // for index in 0..nums.len() {
    //     let i = nums[index - offset];

    //     if i > pivot {
    //         nums.remove(index - offset);
    //         nums.push(i);
    //         offset += 1;
    //     }

    //     if i == pivot {
    //         ten_indices.push(index - offset);
    //     }
    // }

    // let mut ten_offset = 0;
    // for i in ten_indices {
    //     nums.remove(i - ten_offset);
    //     ten_offset += 1;
    //     nums.insert(nums.len() - offset, pivot);
    // }

    // nums
}