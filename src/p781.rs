use std::collections::{HashMap, HashSet};

pub fn run() {
    for i in [
        vec![1, 1, 2],
        vec![10, 10, 10],
        vec![1,0,1,0,0],
        vec![0,0,1,1,1],
        // vec![2, 2, 2],
        // vec![4,4,4,4,4,4,4,4]
    ] {
        println!("{}", num_rabbits(i));
    }
}

pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in answers {
        *map.entry(i).or_insert(0) += 1;
    }

    for (k, v) in map {
        if k == 0 {
            result += v;
            continue;
        }

        let remainder = v % (k + 1);
        let difference = (k + 1) - remainder;

        if remainder == 0 {
            result += v;
        } else {
            result += v + difference;
        }
    }

    result
}
