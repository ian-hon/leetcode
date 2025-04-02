pub fn run() {
    for i in [
        vec![12,6,1,2,7],
        vec![1,10,3,4,19],
        vec![1,2,3],
        vec![2,3,1]
    ] {
        println!("{}", maximum_triplet_value(i));
    }
}

pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut result = 0;

    let l = nums.len();
    for x in 0..(l - 2) {
        let first = nums[x];
        for y in (x + 1)..(l - 1) {
            let second = nums[y];
            for z in (y + 1)..l {
                let third = nums[z] as i64;

                result = result.max((first - second) as i64 * third);
            }
        }
    }

    result.max(0)
}