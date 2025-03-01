pub fn run() {
    for i in [
        vec![1,2,2,1,1,0],
        vec![1,0]
    ] {
        println!("{:?}", apply_operations(i));
    }
}

pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut result = vec![0; nums.len()];
    let mut index = 0;

    for i in 0..nums.len() {
        if nums[i] == 0 {
            continue;
        }
        if i == (nums.len() - 1) {
            result[index] = nums[i];
            break;
        }

        if nums[i] == nums[i + 1] {
            nums[i] *= 2;
            nums[i + 1] = 0;
        }

        if nums[i] != 0 {
            result[index] = nums[i];
            index += 1;
        }
    }

    result
}