pub fn run() {
    for i in [

    ] {
        println!("{}", minimum_sum(i));
    }
}

pub fn minimum_sum(nums: Vec<i32>) -> i32 {
    let mut smallest = 10000;

    for x in 0..nums.len() {
        let a = nums[x];
        for y in x..nums.len() {
            let b = nums[y];
            if !(b > a) {
                continue;
            }

            for z in y..nums.len() {
                let c = nums[z];

                if b > c {
                    smallest = smallest.min(a + b + c);
                }
            }
        }
    }

    if smallest == 10000 {
        return -1;
    }
    smallest
}