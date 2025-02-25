use std::fs;

pub fn run() {
    for i in [
        // vec![1,3,5],
        // vec![2,4,6],
        // vec![1,2,3,4,5,6,7],
        serde_json::from_str(&fs::read_to_string("./src/too_damn_long.txt").unwrap()).unwrap()
    ] {
        println!("{}", num_of_subarrays(i));
    }
}

pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut even = 0;
    let mut odd = 0;

    for start in arr {
        even += 1;
        if start & 1 == 1 {
            (even, odd) = (odd, even);
        }
        result += odd;
        result %= 10i32.pow(9) + 7;
    }

    result
}