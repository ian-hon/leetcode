pub fn run() {
    for i in [
        vec![0,1,1,1,0,0],
        vec![0,1,1,1]
    ] {
        println!("{}", min_operations(i));
    }
}

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut count = 0;
    for i in 0..(nums.len() - 2) {
        if nums[i] == 0 {
            count += 1;
            nums[i] ^= 1;
            nums[i + 1] ^= 1;
            nums[i + 2] ^= 1;
        }
    }

    if (nums[nums.len() - 1] == 0) || (nums[nums.len() - 2] == 0) {
        return -1;
    }

    count
}