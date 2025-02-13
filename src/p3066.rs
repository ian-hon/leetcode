use std::collections::BinaryHeap;

pub fn run() {
    for i in [
        // (vec![2,11,10,1,3], 10),
        // (vec![1,1,2,4,9], 20),
        (vec![999999999,999999999,999999999], 1000000000)
    ] {
        println!("{}", min_operations(i.0, i.1));
    }
}

pub fn min_operations(n: Vec<i32>, k: i32) -> i32 {
    let k = -k as i64;

    let mut nums: Vec<i64> = vec![];
    for i in n {
        nums.push(-i as i64);
    }

    let mut flag = false;
    for i in &nums {
        if *i > k {
            flag = true;
            break;
        }
    }
    if !flag {
        return 0;
    }

    let mut nums = BinaryHeap::from(nums);

    let mut count = 0;
    'outermost: loop {
        count += 1;

        let a = nums.pop().unwrap();
        let b = nums.pop().unwrap();

        nums.push((2 * a) + b);

        for i in &nums {
            if *i > k {
                continue 'outermost;
            }
        }

        return count;
    }
}