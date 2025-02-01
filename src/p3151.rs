pub fn run() {
    for i in [
        vec![1]
    ] {
        println!("{}", is_array_special(i));
    }
}

pub fn is_array_special(nums: Vec<i32>) -> bool {
    let mut nums = nums.into_iter();

    let mut previous = nums.next().unwrap();

    while let Some(n) = nums.next() {
        // xor 1st bit
        // 0 0 false
        // 0 1 true
        // 1 0 true
        // 1 1 false

        if (previous & 1) ^ (n & 1) == 0 {
            return false
        }

        previous = n
    }

    true
}