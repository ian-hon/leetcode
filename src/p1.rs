use std::collections::HashSet;

pub fn run() {
    for i in [
        (vec![2,7,11,15], 9)
    ] {
        println!("{:?}", two_sum(i.0, i.1));
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());

    for (index, i) in nums.iter().enumerate() {
        let d = target - i;

        if set.contains(&d) {
            let mut a = 0;
            for n in &nums {
                if a == index as i32 {
                    continue;
                }
                if *n == d {
                    return vec![index as i32, a];
                }
                a += 1;
            }
        }
    }

    vec![]
}