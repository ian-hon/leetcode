pub fn run() {
    for i in [
        (vec![9], 2),
        (vec![2,4,8,2], 4)
    ] {
        println!("{}", minimum_size(i.0, i.1));
    }
}

pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    // I DONT UNDERSTAND WHAT THE FUCK
    let mut lo = 1;
    let mut hi = 1_000_000_000;
    while lo < hi {
        let mid = (lo + hi) / 2;

        let mut count = 0;
        for n in &nums {
            // HUH WHA HOW???
            count += (n - 1) / mid;
        }

        if count > max_operations {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
