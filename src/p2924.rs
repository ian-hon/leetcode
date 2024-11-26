use std::collections::{HashMap, HashSet};

pub fn run() {
    for i in [
        (3, vec![vec![0,1],vec![1,2]]),
        (4, vec![vec![0,2],vec![1,3],vec![1,2]])
    ] {
        println!("{}", find_champion(i.0, i.1));
    }
}

pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut parents: Vec<i32> = (0..n).collect();

    for i in edges {
        parents[i[1] as usize] = -1;
        // has_parents.remove(&i[1]);
    }

    let mut last = 0;
    let mut count = 0;
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


    // let mut parents: HashMap<i32, Vec<i32>> = HashMap::new();

    // for i in 0..n {
    //     parents.insert(i, vec![]);
    // }

    // for i in edges {
    //     parents.get_mut(&i[1]).unwrap().push(i[0]);
    // }

    // let mut candidates: Vec<i32> = vec![];
    // for i in parents {
    //     if !i.1.is_empty() {
    //         continue;
    //     }
    //     candidates.push(i.0);
    // }

    // if candidates.len() != 1 {
    //     return -1;
    // }

    // candidates[0]
}