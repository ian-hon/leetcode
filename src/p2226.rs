use std::collections::HashMap;

pub fn run() {
    for i in [
        (vec![5,8,6], 3),
        (vec![2,5], 11)
    ] {
        println!("{}", maximum_candies(i.0, i.1));
    }
}

pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut freq_map: HashMap<i32, i64> = HashMap::new();
    for i in candies {
        match freq_map.get_mut(&i) {
            Some(n) => {
                *n += 1;
            },
            None => {
                freq_map.insert(i, 1);
            }
        }
    }

    let mut right = 1_000_000_000_000 as usize;
    let mut left = 0;

    let mut previous_best = 0;

    while (left <= right) && (left != right - 1) {
        let mid = ((right + left) / 2) as i64;

        let mut count = 0;
        for (k, v) in &freq_map {
            let k = *k as i64;
            if k < mid {
                continue;
            }
            count += (k / mid) * v;
        }

        if count >= k {
            previous_best = mid;
            // go left
            left = mid as usize;
        } else {
            // go right
            right = mid as usize;
        }
    }

    previous_best as i32
}