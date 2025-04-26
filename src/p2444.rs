pub fn run() {
    for i in [
        (vec![1,3,5,2,7,5], 1, 5),
        (vec![1,1,1,1], 1, 1)
    ] {
        println!("{}", count_subarrays(i.0, i.1, i.2));
    }
}

pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut min = -1;
    let mut max = -1;
    let mut last_oob = -1;

    let mut result = 0;
    for index in 0..nums.len() {
        let n = nums[index];
        let index = index as i32;
        if n == min_k {
            min = index;
        }
        if n == max_k {
            max = index;
        }
        if (n > max_k) || (n < min_k) {
            last_oob = index;
        }

        result += 0.max(min.min(max) - last_oob) as i64;
    }
    result
}