pub fn run() {
    for i in [
        vec![12,6,1,2,7],
        vec![1,10,3,4,19],
        vec![1,2,3],
        vec![1000000,1,1000000],
        vec![18,15,8,13,10,9,17,10,2,16,17],
        // 272
        vec![6,11,12,12,7,9,2,11,12,4,19,14,16,8,16]
        // 190
    ] {
        println!("{}", maximum_triplet_value(i))
    }
}

pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut highest_difference = 0;
    let mut highest = 0;

    for n in nums {
        result = result.max(highest_difference as i64 * n as i64);
        highest_difference = highest_difference.max(highest - n);
        highest = highest.max(n);
    }

    result
}