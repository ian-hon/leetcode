use std::collections::HashMap;

pub fn run() {
    for i in [
        (10, vec![vec![5,7],vec![1,3],vec![9,10]]), // 2
        (5, vec![vec![2,4],vec![1,3]]), // 1
        (6, vec![vec![1,6]]), // 0
        (1000000000, vec![vec![1,1000000000]]), // 0
        (8, vec![vec![3,4],vec![4,8],vec![2,5],vec![3,8]]) // 1
    ] {
        println!("{}", count_days(i.0, i.1));
    }
}

pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let mut diff_map = HashMap::new();

    for i in meetings {
        match diff_map.get_mut(&(i[0])) {
            Some(d) => {
                *d += 1;
            },
            None => {
                diff_map.insert(i[0], 1);
            }
        }

        match diff_map.get_mut(&(i[1] + 1)) {
            Some(d) => {
                *d -= 1;
            },
            None => {
                diff_map.insert(i[1] + 1, -1);
            }
        }
    }

    let mut differences = diff_map.into_iter().collect::<Vec<(i32, i32)>>();
    differences.sort_by_key(|x| x.0);

    let mut result = differences[0].0 - 1;
    let mut running = differences[0].1;
    for index in 1..(differences.len()) {
        let i = differences[index];

        running += i.1;

        if running == 0 {
            if index != differences.len() - 1 {
                let added = differences[index + 1].0 - i.0;
                result += added;
            } else {
                result += days + 1 - i.0;
            }
        }
    }

    result as i32
}