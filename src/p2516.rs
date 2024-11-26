pub fn take_characters(s: String, k: i32) -> i32 {
    let mut start = 0usize;
    let mut end = s.len();
    let s = s.chars().collect::<Vec<char>>();
    loop {
        if start == end {
            return -1;
        }

        
    }

    

    0
}

pub fn run() {
    for i in [
        ("aabaaaacaabc".to_string(), 2),
        ("a".to_string(), 1)
    ] {
        println!("{}", take_characters(i.0, i.1));
    }
}