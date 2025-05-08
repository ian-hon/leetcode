use std::{cmp::Reverse, collections::BinaryHeap};

pub fn run() {
    for i in [
        vec![vec![0,4],vec![4,4]],
        vec![vec![0,0,0,0],vec![0,0,0,0]],
        vec![vec![0,1],vec![1,2]]
    ] {
        println!("{}", min_time_to_reach(i));
    }
}

pub fn get_neighbours(pos: (usize, usize), height: i32, width:i32, traversed: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let pos = (pos.0 as i32, pos.1 as i32);

    for (x, y) in [
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
    ] {
        if (x >= width) || (y >= height) || (x < 0) || (y < 0) {
            continue;
        }

        let (x, y) = (x as usize, y as usize);

        if traversed[y][x] {
            continue;
        }

        result.push((x, y));
    }

    result
}

pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let height = move_time.len() as i32;
    let width = move_time[0].len() as i32;

    let mut traversed: Vec<Vec<bool>> = vec![vec![false; width as usize]; height as usize];
    let target = (width as usize - 1, height as usize - 1);

    let mut nodes: BinaryHeap<Reverse<(i32, (usize, usize), bool)>> = BinaryHeap::new();
    nodes.push(Reverse((0, (0, 0), false)));
    traversed[0][0] = true;

    // current node -> (current_time, pos, cost)
    // current_time includes current pos

    while let Some(Reverse((time, pos, cost))) = nodes.pop() {
        let next_cost = if cost { 2 } else { 1 };

        for (x, y) in get_neighbours(pos, height, width, &traversed) {
            let payment = (move_time[y][x] - time).max(0) + next_cost;

            if (x, y) == target {
                return time + payment;
            }
            nodes.push(Reverse((time + payment, (x, y), !cost)));
            traversed[y][x] = true;
        }
    }

    0
}