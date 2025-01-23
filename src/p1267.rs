pub fn run() {
    for i in [
        // vec![vec![1,4],vec![2,5],vec![3,6]],
        vec![vec![1,0],vec![1,1]],
        vec![
            vec![0, 0],
            vec![0, 1],
        ],
        // vec![
        //     vec![1,1,1,1],
        //     vec![0,1,0,0],
        //     vec![0,0,1,0],
        //     vec![0,0,0,1]
        // ],
        vec![
            vec![1,1,1,0],
            vec![0,0,1,0],
            vec![0,0,1,0],
            vec![0,0,0,1]
        ]
    ] {
        println!("{}", count_servers(i));
        // count_servers(i);
    }
}


pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;

    let height = grid.len();
    let width = grid[0].len();

    let mut rows = grid.iter().map(|_| 0).collect::<Vec<i32>>();
    let mut columns = grid[0].iter().map(|_| 0).collect::<Vec<i32>>();

    for (y, row) in grid.iter().enumerate() {
        for (x, i) in row.iter().enumerate() {
            if *i == 1 {
                rows[y] += 1;
                columns[x] += 1;
            }
        }
    }

    let mut score = 0;

    for (y, row) in grid.clone().iter().enumerate() {
        if rows[y] <= 1 {
            continue;
        }

        for (x, item) in row.iter().enumerate() {
            if *item != 0 {
                grid[y][x] = 2;
                score += 1;
            }
        }
    }

    for x in 0..width {
        if columns[x] <= 1 {
            continue;
        }

        for y in 0..height {
            let item = grid[y][x];
            
            if item != 0 {
                if item != 2 {
                    score += 1;
                    grid[y][x] = 2;
                }
            }
        }
    }

    score
}