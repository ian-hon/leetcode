use std::cmp::max;

pub fn run() {
    for i in [
        vec!["d1/","d2/","../","d21/","./"],
        vec!["d1/","d2/","./","d3/","../","d31/"],
        vec!["d1/","../","../","../"]
    ] {
        println!("{}", min_operations(i.iter().map(|x| x.to_string()).collect()));
    }
}

pub fn min_operations(logs: Vec<String>) -> i32 {
    // let logs = logs.iter().filter(|x| *x != "./").map(|x| x.to_string()).collect::<Vec<String>>();

    let mut depth = 0;

    for i in logs {
        if i == "./".to_string() {
            continue;
        }

        if i == "../".to_string() {
            depth = max(depth - 1, 0);
        } else {
            depth += 1;
        }
    }

    depth
}
