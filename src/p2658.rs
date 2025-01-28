pub fn run() {
    for i in [
        vec![vec![0,2,1,0],vec![4,0,0,3],vec![1,0,0,4],vec![0,3,2,0]],
        vec![vec![1,0,0,0],vec![0,0,0,0],vec![0,0,0,0],vec![0,0,0,1]]
    ] {
        println!("{}", find_max_fish(i));
    }
}

pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    let width = grid[0].len();
    let height = grid.len();
    let mut visited = vec![vec![false; width]; height];

    let mut result = 0;

    for y in 0..height {
        for x in 0..width {
            if (grid[y][x] == 0) || (visited[y][x]) {
                continue;
            }

            let mut count = 0;

            get_group((x, y), &grid, &mut visited, &mut count, height as i32, width as i32);

            result = count.max(result);
        }
    }

    result
}

pub fn get_group(p: (usize, usize), grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, count: &mut i32, height: i32, width: i32) {
    if visited[p.1][p.0] {
        return
    }

    *count += grid[p.1][p.0];

    visited[p.1][p.0] = true;

    for i in [
        (p.0 as i32, p.1 as i32 + 1),
        (p.0 as i32, p.1 as i32 - 1),
        (p.0 as i32 + 1, p.1 as i32),
        (p.0 as i32 - 1, p.1 as i32),
    ] {
        if (i.0 < 0) || (i.0 >= width) || (i.1 < 0) || (i.1 >= height) {
            continue;
        }

        let x = i.0 as usize;
        let y = i.1 as usize;

        if grid[y][x] == 0 {
            continue;
        }

        get_group((x, y), grid, visited, count, height, width);
    }
}
