use std::collections::{HashMap, HashSet};

pub fn run() {
    for i in [
        (4, 6, [[0,0],[1,1],[2,3]].to_vec().iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>(), [[0,1],[2,2],[1,4]].to_vec().iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>()),
        (4, 16, [[0, 5],[0, 7]].to_vec().iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>(), [[0, 2],[0, 10],[0, 13]].to_vec().iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>()),
        (3, 3, [[1, 1]].to_vec().iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>(), [[0, 1],[1, 0],[2, 1], [1, 2]].to_vec().iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>())
    ] {
        println!("{}", count_unguarded(i.0, i.1, i.2, i.3));
    }
}

pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    // m -> x
    // n -> y
    let mut map: Vec<Vec<u8>> = (0..m).map(|_| (0..n).map(|_| 0).collect::<Vec<u8>>()).collect();
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();

    // 0 -> empty
    // 1 -> walls
    // 2 -> guards

    for w in &walls {
        map[w[0] as usize][w[1] as usize] = 1;
    }

    for g in &guards {
        map[g[0] as usize][g[1] as usize] = 2;
    }

    for (row_index, row) in map.clone().iter().enumerate() {
        for i in walk_through(&row) {
            occupied.insert((i, row_index as i32));
        }
    }

    for column_index in 0..map[0].len() {
        let column = map.iter().map(|x| x[column_index]).collect::<Vec<u8>>();
        for i in walk_through(&column) {
            occupied.insert((column_index as i32, i));
        }
    }

    (m * n) - (guards.len() + walls.len() + occupied.len()) as i32
}


fn walk_through(c: &Vec<u8>) -> HashSet<i32> {
    let mut pointer = 0;
    let mut result = HashSet::new();

    let mut infection = false;
    let mut bucket: Vec<i32> = vec![];

    loop {
        if pointer as usize >= c.len() {
            if infection {
                for i in bucket {
                    result.insert(i);
                }
            }
            break;
        }

        let item = c[pointer as usize];

        match item {
            0 => {
                bucket.push(pointer);
            },
            1 => {
                if infection {
                    for i in bucket {
                        result.insert(i);
                    }
                }
                bucket = vec![];
                infection = false;
            },
            2 => {
                infection = true;
            },
            _ => {}
        }

        pointer += 1;
    }

    result
}
