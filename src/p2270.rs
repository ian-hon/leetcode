use std::fs;

pub fn run() {
    for i in [
        // vec![10,4,-8,7],
        // vec![2,3,1,0],
        serde_json::from_str(&fs::read_to_string("./src/too_damn_long.txt").unwrap()).unwrap()
    ] {
        println!("{}", ways_to_split_array(i));
    }
}

pub fn ways_to_split_array(nums: Vec<i64>) -> i32 {
    let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<i64>>();
    let length = nums.len() - 1;
    let mut count = 0;

    let mut right_sum = nums.iter().sum();
    let mut running = 0;

    let mut index = 0;
    for i in nums {
        running += i;
        right_sum -= i;
        if running >= right_sum {
            count += 1;
        }
        index += 1;
        if index == length {
            break;
        }
    }

    count
}