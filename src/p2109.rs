use std::fs;

pub fn run() {
    for i in [
        ("LeetcodeHelpsMeLearn", vec![8,13,15]),
    ].into_iter().map(|x| (x.0.to_string(), x.1)) {
        println!("{}", add_spaces(i.0, i.1));
    }

    let r = fs::read("./src/too_damn_long.txt").unwrap();
    let in_words = r.into_iter().map(|x| x as char).collect::<String>().split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
    add_spaces(
        in_words[0].clone(),
        in_words[1].split(",").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>()
    );
    // println!("{:?}", r.into_iter().map(|x| x as char).collect::<String>());
}

pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    if spaces.is_empty() {
        return s;
    }

    let spaces = spaces.into_iter().map(|x| x as usize).collect::<Vec<usize>>();

    let mut result: String = "".to_string();

    let c = s.chars().into_iter().collect::<Vec<char>>();

    let mut s_flag = true;

    let mut next_s_pos = 0usize;
    let mut next_s = spaces[0];

    for index in 0..s.len() {
        if s_flag && (index == next_s) {
            result.push(' ');
            if next_s_pos == (spaces.len() - 1) {
                s_flag = false;
            } else {
                next_s_pos += 1;
                next_s = spaces[next_s_pos];
            }
        }
        result.push(c[index]);
    }

    result
}