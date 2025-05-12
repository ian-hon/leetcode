use std::collections::{HashMap, HashSet};

pub fn run() {
    for i in [
        vec![2,1,3,0],
        vec![2,2,8,8,2],
        vec![3,7,5]
    ] {
        println!("{:?}", find_even_numbers(i));
    }
}

pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    let mut freq_map = vec![0; 10];
    for d in digits {
        freq_map[d as usize] += 1;
    }

    let mut result = vec![];

    let mut c = 100;
    'outer: while c <= 998 {
        let mut required = vec![0; 10];

        let mut t = c.clone();
        while t > 9 {
            required[t as usize % 10] += 1;
            t /= 10;
        }
        required[t as usize] += 1;

        for r in 0..10 {
            if required[r] > freq_map[r] {
                c += 2;
                continue 'outer;
            }
        }

        result.push(c);

        c += 2;
    }

    result
}