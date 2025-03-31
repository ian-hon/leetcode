pub fn run() {
    for i in [
        // (vec![1,3,5,1], 2),
        (vec![1,3], 2),
        (vec![1,4,2,5,2], 3),
        (vec![54,6,34,66,63,52,39,62,46,75,28,65,18,37,18,13,33,69,19,40,13,10,43,61,72], 4),
        // 3
        (vec![1,3,5,1], 1),
        (vec![1000000000,1000000000,1000000000,1000000000], 3),
        (vec![1,4,2,5,2], 3),
        (vec![24,16,62,27,8,3,70,55,13,34,9,29,10], 11),
        (vec![68,65,5,74,12,67,10,55,27,38,69,54,62,50,30,3,1,24,39,65,73,33,43,9,64], 9)
    ] {
        println!("{}", put_marbles(i.0, i.1));
    }
}

pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let mut paired_weights = weights.windows(2).map(|x| (x[0] + x[1]) as i64).collect::<Vec<i64>>();
    paired_weights.sort_unstable();
    paired_weights.iter().rev().take((k as usize) - 1).sum::<i64>() - paired_weights.iter().take((k as usize) - 1).sum::<i64>()
}
