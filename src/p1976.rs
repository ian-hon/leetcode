use std::{cmp::Reverse, collections::BinaryHeap, fs};

pub fn run() {
    for i in [
        // (7, vec![vec![0,6,7],vec![0,1,2],vec![1,2,3],vec![1,3,3],vec![6,3,3],vec![3,5,1],vec![6,5,1],vec![2,5,1],vec![0,4,5],vec![4,6,2]]),
        // (10, vec![vec![0, 1, 1], vec![0, 2, 1], vec![0, 3, 1], vec![0, 4, 1], vec![1, 5, 1], vec![1, 6, 1], vec![1, 7, 1], vec![1, 8, 1], vec![2, 5, 1], vec![2, 6, 1], vec![2, 7, 1], vec![2, 8, 1], vec![3, 5, 1], vec![3, 6, 1], vec![3, 7, 1], vec![3, 8, 1], vec![4, 5, 1], vec![4, 6, 1], vec![4, 7, 1], vec![4, 8, 1], vec![5, 9, 1], vec![6, 9, 1], vec![7, 9, 1], vec![8, 9, 1]]),
        // (2, vec![vec![1,0,10]]),
        // (12, vec![vec![1,0,2348],vec![2,1,2852],vec![2,0,5200],vec![3,1,12480],vec![2,3,9628],vec![4,3,7367],vec![4,0,22195],vec![5,4,5668],vec![1,5,25515],vec![0,5,27863],vec![6,5,836],vec![6,0,28699],vec![2,6,23499],vec![6,3,13871],vec![1,6,26351],vec![5,7,6229],vec![2,7,28892],vec![1,7,31744],vec![3,7,19264],vec![6,7,5393],vec![2,8,31998],vec![8,7,3106],vec![3,8,22370],vec![8,4,15003],vec![8,6,8499],vec![8,5,9335],vec![8,9,5258],vec![9,2,37256],vec![3,9,27628],vec![7,9,8364],vec![1,9,40108],vec![9,5,14593],vec![2,10,45922],vec![5,10,23259],vec![9,10,8666],vec![10,0,51122],vec![10,3,36294],vec![10,4,28927],vec![11,4,25190],vec![11,9,4929],vec![11,8,10187],vec![11,6,18686],vec![2,11,42185],vec![11,3,32557],vec![1,11,45037]]),
        // (47, serde_json::from_str::<Vec<Vec<i32>>>(&fs::read_to_string("./src/too_damn_long.txt").unwrap()).unwrap()),
        // (5, vec![vec![0,1,1],vec![1,2,4],vec![0,4,3],vec![3,2,5],vec![3,4,1],vec![3,0,5],vec![1,3,1]]),
        (6, vec![vec![3,0,4],vec![0,2,3],vec![1,2,2],vec![4,1,3],vec![2,5,5],vec![2,3,1],vec![0,4,1],vec![2,4,6],vec![4,3,1]])
    ] {
        println!("{:?}", count_paths(i.0, i.1));
    }
}

pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    // trying out rust's binary heap and the Reverse thingy
    // VecDeque didnt work

    let n = n as usize;
    let modulo = 1000000007;

    let mut routes: Vec<Vec<(usize, i64)>> = vec![vec![]; n];

    let mut count = vec![0; n];
    let mut cost = vec![i64::MAX; n];
    count[0] = 1;
    cost[0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0)));

    for r in roads {
        routes[r[0] as usize].push((r[1] as usize, r[2] as i64));
        routes[r[1] as usize].push((r[0] as usize, r[2] as i64));
    }

    while let Some(Reverse((current_cost, node))) = heap.pop() {
        if current_cost > cost[node] {
            continue;
        }

        for (next, weight) in &routes[node] {
            if (current_cost + weight) < cost[*next] {
                // update and reset
                cost[*next] = current_cost + weight;
                count[*next] = count[node];

                heap.push(Reverse((current_cost + weight, *next)));
            } else if (current_cost + weight) == cost[*next] {
                count[*next] = (count[*next] + count[node]) % modulo;
            }
        }
    }

    count[n - 1]
}