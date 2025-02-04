pub fn run() {
    for i in [

    ] {
        println!("{}", max_ascending_sum(i));
    }
}

pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums.into_iter();
    let mut previous = nums.next().unwrap();

    let mut highest = previous;
    let mut current = previous;

    while let Some(n) = nums.next() {
        if previous >= n {
            current = 0;
        }

        current += n;

        highest = highest.max(current);

        previous = n;
    }

    highest
}