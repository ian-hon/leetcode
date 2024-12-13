use std::i32;

pub fn run() {
    for i in [
        vec![2,1,3,4,5,2],
        vec![2,3,5,1,3,2]
    ] {
        println!("{}", find_score(i));
    }
}

pub fn find_score(nums: Vec<i32>) -> i64 {
    let mut c = vec![];
    let mut marked = vec![];

    let mut current_index = 0;
    for n in nums {
        c.push((n as i64, current_index));
        marked.push(false);

        current_index += 1usize;
    }
    c.sort();

    let length = c.len();

    let mut score = 0;
    for i in c {
        if marked[i.1] {
            continue;
        }

        score += i.0;
        marked[i.1] = true;
        if i.1 != 0 {
            marked[i.1 - 1] = true;
        }
        if i.1 != (length - 1) {
            marked[i.1 + 1] = true;
        }
    }

    score
}