pub fn run() {
    for i in [
        vec![1,-3,2,3,-4],
        vec![2,-5,1,-4,3,-2],
        vec![-3,-5,-3,-2,-6,3,10,-10,-8,-3,0,10,3,-5,8,7,-9,-9,5,-8]
    ] {
        println!("{}", max_absolute_sum(i));
    }
}

pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut current = 0;
    let mut min = 0;
    let mut max = 0;

    for n in nums {
        current += n;
        min = min.min(current);
        max = max.max(current);
    }

    max - min
}