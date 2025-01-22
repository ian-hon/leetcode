use std::time::Instant;

pub fn run() {
    for i in [
        vec![vec![2,5,4],vec![1,5,1]],
        vec![vec![3,3,1],vec![8,5,2]],
        vec![vec![1,3,1,15],vec![1,3,3,1]],
    ] {
        println!("{}", grid_game(i));
    }
}

pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
    // iterations are 0..width

    let grid = grid.iter().map(|x| x.iter().map(|y| *y as i64).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();

    let width = grid[0].len();

    let mut lowest_score = i64::MAX;

    let mut down_sum = 0;
    let mut up_sum = 0;
    for i in &grid[0] {
        up_sum += i;
    }

    for t in 0..width {
        up_sum -= grid[0][t];
        if t > 0 {
            down_sum += grid[1][t - 1];
        }
        lowest_score = (down_sum.max(up_sum)).min(lowest_score);
    }

    lowest_score
}
