pub fn run() {
    for i in [
        (vec![2,1,4,3,5], 10),
        (vec![1,1,1], 5),
        (vec![9,5,3,8,4,7,2,7,4,5,4,9,1,4,8,10,8,10,4,7], 4) // 3
    ] {
        println!("{}", count_subarrays(i.0, i.1));
    }
}

pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let mut result = 0;
    let mut left = 0;
    let mut running = 0;
    for (right, n) in nums.iter().enumerate() {
        let right = right as i64;
        running += *n as i64;
        while (running * (right- left + 1)) >= k {
            running -= nums[left as usize] as i64;
            left += 1;
        }
        result += right - left + 1;
    }
    result
}