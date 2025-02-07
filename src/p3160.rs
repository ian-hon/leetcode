use std::{collections::HashMap, env, fs};

pub fn run() {
    for i in [
        (4, vec![vec![1,4],vec![2,5],vec![1,3],vec![3,4]]),
        (4, vec![vec![0,1],vec![1,2],vec![2,2],vec![3,4],vec![4,5]]),
        // (1000000000, serde_json::from_str::<Vec<Vec<i32>>>(&fs::read_to_string("./src/too_damn_long.txt").unwrap()).unwrap())
    ] {
        println!("{:?}", query_results(i.0, i.1));
    }
}

pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut colour_amounts = HashMap::new();
    let mut result = vec![];

    for q in queries {
        let previous = match map.get_mut(&q[0]) {
            Some(n) => {
                *n
            },
            None => {
                map.insert(q[0], 0);
                0
            }
        };
        let current = q[1];

        if previous != 0 {
            *colour_amounts.get_mut(&previous).unwrap() -= 1;
            if *colour_amounts.get(&previous).unwrap() <= 0 {
                colour_amounts.remove(&previous);
            }
        }

        match colour_amounts.get_mut(&current) {
            Some(n) => {
                *n += 1;
            },
            None => {
                colour_amounts.insert(current, 1);
            }
        };

        // *previous = current;
        *map.get_mut(&q[0]).unwrap() = current;

        result.push(colour_amounts.len() as i32);
    }

    result
}