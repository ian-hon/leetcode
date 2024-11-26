pub fn run() {
    for i in [
        vec![vec![0,1],vec![0,0]],
        vec![vec![0,0,1],vec![1,0,1],vec![0,0,0]]
    ] {
        println!("{}", find_champion(i));
    }
}

pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len() as usize;
    let mut parents: Vec<i32> = (0..n as i32).collect();

    for a in 0..n {
        for b in 0..n {
            if grid[a][b] == 1 {
                parents[b] = -1;
            }

            // grid[a][b] == 1 {
            //     a is stronger than b
            // }
        }
    }

    let mut count = 0;
    let mut last = 0;
    for i in parents {
        if i == -1 {
            continue;
        }
        if count != 0 {
            return -1;
        }
        last = i;
        count += 1;
    }
    if count == 1 {
        return last;
    }
    -1
}