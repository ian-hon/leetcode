pub fn run() {
    for i in [
        // (vec![4,6,1,2], 2),
        (vec![1,2,4,6], 2),
        (vec![1,1,1,1], 10),
        (vec![49,26], 12) // expect 2
    ] {
        println!("{}", maximum_beauty(i.0, i.1));
    }
}

pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut w = 0;

    for n in 0..nums.len() {
        if (nums[n] - nums[w]) > (k * 2) {
            w += 1;
        }
    }

    (nums.len() - w) as i32
}