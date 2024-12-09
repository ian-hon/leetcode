pub fn run() {
    for i in [
        (vec![3,4,1,2,6], vec![vec![0,4]]),
        (vec![4,3,1,6], vec![vec![0,2],vec![2,3]])
    ] {
        println!("{:?}", is_array_special(i.0, i.1));
    }
}

pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut special: Vec<i32> = vec![0];
    for i in 1..nums.len() {
        special.push(special[i - 1]);
        if (nums[i] % 2) == (nums[i - 1] % 2) {
            // parities are the same
            special[i] += 1;
        }
    }

    let mut result = vec![];
    for q in queries {
        let start = q[0];
        let end = q[1];

        let count = special[end as usize] - special[start as usize];
        result.push(count == 0);
    }

    result
}