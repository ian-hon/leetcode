use std::collections::HashMap;

pub fn run() {
    for i in [
        vec![vec![1,2],vec![2,1],vec![3,4],vec![5,6]],
        vec![vec![1,2],vec![1,2],vec![1,1],vec![1,2],vec![2,2]]
    ] {
        println!("{}", num_equiv_domino_pairs(i));
    }
}

pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for d in dominoes {
        let d = (d[0].max(d[1]), d[0].min(d[1]));
        *map.entry(d).or_insert(0) += 1;
    }

    let mut result = 0;
    for (_, v) in map {
        if v == 1 {
            continue;
        }
        result += ((v * v) - v) / 2;
    }

    result
}