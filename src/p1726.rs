use std::collections::HashMap;

pub fn run() {
    for i in [
        vec![2,3,4,6],
        vec![1,2,4,5,10]
    ] {
        println!("{}", tuple_same_product(i));
    }
}

pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let mut pairs: HashMap<i32, i32> = HashMap::new();

    for n in 0..nums.len() {
        for i in &nums[(n + 1)..nums.len()] {
            let p = nums[n] * *i;
            match pairs.get_mut(&p) {
                Some(m) => {
                    *m += 1;
                },
                None => {
                    pairs.insert(p, 1);
                }
            }
        }
    }

    let mut count = 0;

    for (_, v) in pairs {
        if v <= 1 {
            continue;
        }

        count += 8 * compounded_sum(v);
    }

    count
}

pub fn compounded_sum(i: i32) -> i32 {
    // area of triangle
    let i = i - 1;
    ((i * i) + i) / 2
}
