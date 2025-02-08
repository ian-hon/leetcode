use std::collections::HashSet;

pub fn run() {
    for i in [
        vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,2,-1]],
        vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,0,2]],
        vec![vec![0,1],vec![2,0]]
    ] {
        println!("{}", unique_paths_iii(i));
    }
}

pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut target_count = 0;
    let mut starting = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if *i != -1 {
                target_count += 1;

                if *i == 1 {
                    starting = (x, y);
                }
            }
        }
    }

    let mut final_count = 0;

    traverse(&grid, target_count, height, width, &mut final_count, starting, HashSet::from([starting]));
    // traverse(&mut final_count, target_count, height, width, starting, HashSet::from([starting]));

    final_count
}

pub fn traverse(grid: &Vec<Vec<i32>>, target_count: i32, height: i32, width: i32, final_count: &mut i32, pos: (usize, usize), map: HashSet<(usize, usize)>) {
    // println!("traverse started at : {pos:?} with {map:?}");

    for n in neighbours(grid, &map, (pos.0 as i32, pos.1 as i32), height, width) {
        // println!("{n:?}");
        if grid[n.1][n.0] == 2 {
            // reached end
            if (map.len() as i32 + 1) == target_count {
                *final_count += 1;
            }

            continue;
        }

        // not reached end
        let mut t = map.clone();
        t.insert(n);

        traverse(grid, target_count, height, width, final_count, n, t);
    }
}

pub fn neighbours(grid: &Vec<Vec<i32>>, traversed: &HashSet<(usize, usize)>, i: (i32, i32), h: i32, w: i32) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for n in [
        (i.0 + 1, i.1),
        (i.0 - 1, i.1),
        (i.0, i.1 + 1),
        (i.0, i.1 - 1),
    ] {
        if (n.0 < 0) || (n.1 < 0) || (n.0 >= w) || (n.1 >= h) {
            continue;
        }

        let p = (n.0 as usize, n.1 as usize);

        if (grid[p.1][p.0] != -1) && (!traversed.contains(&p)) {
            result.push((p.0, p.1));
        }
    }

    result
}