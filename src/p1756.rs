use std::collections::HashSet;

pub fn run() {
    for i in [
        vec![vec![0,1],vec![0,0]],
        vec![vec![0,0,1],vec![1,0,0],vec![0,0,0]],
    ] {
        println!("{:?}", highest_peak(i));
    }
}

pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut is_water = is_water;
    let height = is_water.len() as i32;
    let width = is_water[0].len() as i32;

    let mut nodes: HashSet<(i32, i32)> = HashSet::new();
    for (y, row) in is_water.clone().iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if *i == 1 {
                nodes.insert((x as i32, y as i32));
                is_water[y][x] = 0;
            } else {
                is_water[y][x] = -1;
            }
        }
    }

    loop {
        let mut flag = false;

        let mut next_points = is_water.clone();
        let mut next_nodes = HashSet::new();

        for w in &nodes {
            let current = is_water[w.1 as usize][w.0 as usize];

            for n in [
                [w.0, w.1 - 1],
                [w.0 + 1, w.1],
                [w.0, w.1 + 1],
                [w.0 - 1, w.1]
            ] {
                if (n[0] < 0) || (n[0] >= width) || (n[1] < 0) || (n[1] >= height) {
                    continue;
                }

                let neighbour = is_water[n[1] as usize][n[0] as usize];

                if neighbour != -1 {
                    continue;
                }

                next_points[n[1] as usize][n[0] as usize] = current + 1;
                next_nodes.insert((n[0], n[1]));
                flag = true;
            }
        }

        is_water = next_points;
        nodes = next_nodes;

        if !flag {
            return is_water;
        }
    }

    // let mut target_height = 1;

    // loop {
    //     let mut flag = false;

    //     let mut next: Vec<Vec<i32>> = is_water.clone();

    //     for (y, row) in is_water.iter().enumerate() {
    //         for (x, item) in row.iter().enumerate() {
    //             if *item == 0 {
    //                 continue;
    //             }

    //             if *item != target_height {
    //                 continue;
    //             }

    //             let mut passes = true;

    //             for n in [
    //                 [x as i32, y as i32 - 1],
    //                 [x as i32 + 1, y as i32],
    //                 [x as i32, y as i32 + 1],
    //                 [x as i32 - 1, y as i32]
    //             ] {
    //                 if (n[0] < 0) || (n[0] >= width) || (n[1] < 0) || (n[1] >= height) {
    //                     continue;
    //                 }

    //                 if *item != is_water[n[1] as usize][n[0] as usize] {
    //                     passes = false;
    //                 }
    //             }

    //             if !passes {
    //                 next[y][x] = *item;
    //                 continue;
    //             }
    //             next[y][x] = item + 1;
    //             flag = true;
    //         }
    //     }

    //     target_height += 1;
    //     is_water = next;

    //     if !flag {
    //         return is_water;
    //     }
    // }
}
