use std::collections::HashSet;

pub fn run() {
    for i in [
        vec![vec![1,3],vec![2,2]],
        vec![vec![9,1,7],vec![8,9,2],vec![3,4,6]]
    ] {
        println!("{:?}", find_missing_and_repeated_values(i));
    }
}

pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let highest = grid.len().pow(2) as i32;

    let mut map: HashSet<i32> = HashSet::from_iter(1..=highest);

    let mut twice = 0;

    for row in grid {
        for item in row {
            if !map.remove(&item) {
                twice = item;
            }
        }
    }

    vec![twice, map.drain().collect::<Vec<i32>>()[0]]
}