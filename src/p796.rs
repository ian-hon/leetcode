pub fn run() {
    for i in [
        ("abcde".to_string(), "cdeab".to_string()),
        ("abcde".to_string(), "abced".to_string()),
    ] {
        println!("{}", rotate_string(i.0, i.1));
    }
}

pub fn rotate_string(s: String, goal: String) -> bool {
    if goal.len() != s.len() {
        return false;
    }

    let goal = goal.chars().collect::<Vec<char>>();
    let s = s.chars().collect::<Vec<char>>();
    let key = goal[0];
    let length = goal.len();

    let indices = s.iter().enumerate().filter(|x| *x.1 == key).map(|x| x.0).collect::<Vec<usize>>();

    for i in indices {
        let mut flag = true;
        for n in 0..length {
            if s[get(n + i, length)] == goal[n] {
                continue;
            }
            flag = false;
        }
        if !flag {
            continue;
        }
        return true;
    }

    false
}

pub fn get(i: usize, length: usize) -> usize {
    if i >= length {
        return i - length
    }
    i
}
