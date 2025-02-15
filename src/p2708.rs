pub fn run() {
    for i in [
        vec![3,-1,-5,2,5,-9],
        vec![-4,-5,-4],
        vec![0,-1], // 0
        vec![-9] // 9
    ] {
        println!("{}", max_strength(i));
    }
}

pub fn max_strength(nums: Vec<i32>) -> i64 {
    let mut nums = nums;
    nums.sort();

    let mut flag = false;
    let mut result = 1 as i64;
    let mut previous_n: Option<i32> = None;
    let mut z_count = 0;
    for n in nums {
        if n == 0 {
            z_count += 1;
            continue;
        }
        if n.is_positive() {
            result *= n as i64;
            flag = true;
        } else {
            match previous_n {
                Some(i) => {
                    result *= (i * n) as i64;
                    flag = true;
                    previous_n = None;
                },
                None => {
                    previous_n = Some(n);
                }
            }
        }
    }

    if !flag {
        if z_count > 0 {
            return 0;
        }
        if previous_n.is_some() {
            return previous_n.unwrap() as i64;
        }

        return 0;
    }

    result
}