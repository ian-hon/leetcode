use std::collections::HashMap;

pub fn run() {
    for i in [
        (vec![3, 2, 4], 2, 1),
        (vec![3,1,9,6], 3, 0),
        (vec![11,12,21,31], 10, 1)
    ] {
        println!("{}", count_interesting_subarrays(i.0, i.1, i.2));
    }
}

pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    let mut map: HashMap<i32, i64> = HashMap::new();
    map.insert(0, 1);

    let mut running = 0;
    let mut result = 0;
    for n in nums {
        running += if n % modulo == k { 1 } else { 0 };

        let remainder = running % modulo;
        let lookback = (remainder - k + modulo) % modulo;
        result += map.get(&lookback).unwrap_or(&0);
        *map.entry(remainder).or_insert(0) += 1;
    }

    result
}