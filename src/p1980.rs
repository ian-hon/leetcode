use std::collections::HashSet;

pub fn run() {
    for i in [
        // vec!["01","10"],
        vec!["00","10"],
        // vec!["00","01"],
        // vec!["111","011","001"]
    ] {
        println!("{}", find_different_binary_string(i.into_iter().map(|x| x.to_string()).collect::<Vec<String>>()));
    }
}

pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let length = nums.len();
    let nums: HashSet<i32> = HashSet::from_iter(nums.iter().map(|x| i32::from_str_radix(x, 2).unwrap()).collect::<Vec<i32>>());

    for i in 0..(2i32.pow(length as u32)) {
        if !nums.contains(&i) {
            println!("{:b}", i);
            return format!("{:0width$b}", i, width = length);
        }
    }

    "".to_string()
}