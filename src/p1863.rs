pub fn run() {
    for i in [
        vec![1,3],
        vec![5,1,6],
        vec![3,4,5,6,7,8]
    ] {
        println!("{}", subset_xor_sum(i));
    }
}

pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let l = nums.len();

    let mut total = 0;
    for mask in 0..(2u32.pow(l as u32)) {
        let mut current = 0;
        for i in 0..l {
            if ((mask >> i) & 1) == 1 {
                current ^= nums[i];
            }
        }
        total += current;
    }

    total
}
