pub fn run() {
    for i in [

    ] {
        println!("{}", longest_monotonic_subarray(i));
    }
}

pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut inc = 1;
    let mut dec = 1;

    let mut highest = 1;

    let mut nums = nums.iter();
    let mut previous = *nums.next().unwrap();
    while let Some(n) = nums.next() {
        if *n > previous { 
            // inc
            inc += 1;
            dec = 1;
        } else if *n < previous {
            // dec
            dec += 1;
            inc = 1;
        } else {
            dec = 1;
            inc = 1;
        }

        highest = highest.max(inc.max(dec));
        previous = *n;
    }

    highest
}