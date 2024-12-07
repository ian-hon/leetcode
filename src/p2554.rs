use std::collections::HashSet;

pub fn run() {
    for i in [
        (vec![1,6,5], 5, 6),
        (vec![1,2,3,4,5,6,7], 8, 1),
        (vec![11], 7, 50)
    ] {
        println!("{}", max_count(i.0, i.1, i.2));
    }
}

pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    let banned_set: HashSet<i32> = HashSet::from_iter(banned);

    let mut count = 0;
    let mut max_sum = max_sum;
    for i in 1..=n {
        if banned_set.contains(&i) {
            continue;
        }
        if i <= max_sum {
            max_sum -= i;
            count += 1;
        }
    }

    count
}