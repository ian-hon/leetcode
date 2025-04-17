pub fn run() {
    for i in [
        (vec![3,1,2,2,2,1,3], 2),
        (vec![1,2,3,4], 1)
    ] {
        println!("{}", count_pairs(i.0, i.1));
    }
}

pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;

    for (l, n) in nums.iter().enumerate() {
        for r in (l + 1)..nums.len() {
            if nums[r] != *n {
                continue;
            }
            if (l * r) as i32 % k == 0 {
                result += 1;
            }
        }
    }

    result
}