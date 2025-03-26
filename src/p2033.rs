use std::i32;

pub fn run() {
    for i in [
        (vec![vec![2,4],vec![6,8]], 2),
        (vec![vec![1,5],vec![2,3]], 1),
        (vec![vec![1,2],vec![3,4]], 2),
        (vec![vec![146]], 86),
        (vec![vec![529,529,989],vec![989,529,345],vec![989,805,69]], 92)
        // 25
    ] {
        println!("{}", min_operations(i.0, i.1));
    }
}

pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut numbers = grid.into_iter().flatten().collect::<Vec<i32>>();
    numbers.sort_unstable();
    let median = numbers[numbers.len() / 2];

    let mut result = 0;
    for n in numbers.iter().map(|n| (n - median).abs()) {
        if (n % x) != 0 {
            return -1;
        }
        result += n / x;
    }

    result
}