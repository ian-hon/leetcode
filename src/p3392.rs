pub fn run() {
    for i in [
        vec![1,2,1,4,1],
        vec![1,1,1]
    ] {
        println!("{}", count_subarrays(i));
    }
}

pub fn count_subarrays(nums: Vec<i32>) -> i32 {
    // nums.windows(3).filter(|n| ((n[0] + n[2]) * 2) == n[1]).count() as i32
    let mut result = 0;
    for n in nums.windows(3) {
        if ((n[0] + n[2]) * 2) == n[1] {
            result += 1;
        }
    }
    result
}