use std::collections::{HashMap, HashSet};

// pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
//     let mut result = vec![];

//     let mut paths = (0..(n - 1)).map(|i| (i, vec![i + 1])).collect::<HashMap<i32, Vec<i32>>>();
//     for q in queries {
//         paths.get_mut(&q[0]).unwrap().push(q[1]);

//         result.push(Solution::shortest_path(&paths, n));
//     }

//     result
// }

// pub fn shortest_path(paths: &HashMap<i32, Vec<i32>>, n: i32) -> i32 {
//     let mut traversed: HashSet<i32> = HashSet::new();
//     traversed.insert(0);
//     let mut traversal: Vec<i32> = vec![0];
//     let mut depth = 0;

//     let mut next= vec![];
//     loop {
//         depth += 1;
//         for t in traversal {
//             for path in paths.get(&t).unwrap() {
//                 if !traversed.insert(*path) {
//                     continue;
//                 }
//                 if *path == (n - 1) {
//                     return depth;
//                 }
//                 next.push(*path);
//             }
//         }
//         traversal = next.clone();
//         next = vec![];
//     }
// }

pub fn run() {
    for i in [
        (6, vec![vec![1,4],vec![0,2]]),
        // (5, vec![vec![2,4],vec![0,2],vec![0,4]]),
        // (4, vec![vec![0,3],vec![0,2]])
    ] {
        println!("{:?}", shortest_distance_after_queries(i.0, i.1));
    }
}

pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let mut pre_empty: Vec<bool> = (0..n).map(|_| false).collect();
    pre_empty[0] = true;

    let mut paths = (0..(n - 1)).map(|x| vec![x + 1]).collect::<Vec<Vec<i32>>>();
    for q in queries {
        paths[q[0] as usize].push(q[1]);

        result.push(shortest_path(&paths, n - 1, &pre_empty));
    }

    result
}

pub fn shortest_path(paths: &Vec<Vec<i32>>, target: i32, pre: &Vec<bool>) -> i32 {
    let mut traversed: Vec<bool> = pre.clone();
    let mut traversal: Vec<i32> = vec![0];
    let mut depth = 0;

    loop {
        depth += 1;
        for _ in 0..traversal.len() {
            for path in &paths[traversal.remove(0) as usize] {
                let p = *path;
                let u = p as usize;
                if traversed[u] {
                    // if its traversed before
                    continue;
                }
                traversed[u] = true;
                if p == target {
                    return depth;
                }
                traversal.push(p);
            }
        }
    }
}