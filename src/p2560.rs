pub fn run() {
    for i in [
        (vec![2,3,5,9], 2),
        (vec![2,7,9,3,1], 2)
    ] {
        println!("{}", min_capability(i.0, i.1));
    }
}

pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 1_000_000_000;
    let mut right = 0;

    for i in &nums {
        if *i < left {
            left = *i;
        }
        if *i > right {
            right = *i;
        }
    }

    'outer: while left < right {
        let mid = (left + right) / 2;

        let mut count = 0;
        let mut last_toggle = false;
        for i in &nums {
            if last_toggle {
                last_toggle = false;
                continue;
            }

            if *i <= mid {
                count += 1;
                last_toggle = true;

                if count >= k {
                    right = mid;
                    continue 'outer;
                }
            }
        }
        
        left = mid + 1;
    }

    left
}