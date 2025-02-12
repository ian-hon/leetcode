use std::collections::HashMap;

pub fn run() {
    for i in [
        vec![18,43,36,13,7],
        vec![10,12,19,14],
        vec![368,369,307,304,384,138,90,279,35,396,114,328,251,364,300,191,438,467,183]
    ] {
        println!("{}", maximum_sum(i));
    }
}

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut pairs: HashMap<i32, Vec<i32>> = HashMap::from_iter((0..81).map(|x| (x, vec![])).collect::<Vec<(i32, Vec<i32>)>>());
    // let mut pairs = [vec![]; 81];
    
    for i in nums {
        let mut s = 0;
        let mut n = i;
        loop {
            s += n % 10;
            n /= 10;
            if n <= 0 {
                break;
            }
        }

        pairs.get_mut(&s).unwrap().push(i);
    }

    let mut total = -1;
    for (_, p) in pairs {
        if p.len() <= 1 {
            continue;
        }

        let mut p = p.clone();
        p.sort();

        total = total.max(p[p.len() - 2] + p[p.len() - 1]);
    }

    total
}