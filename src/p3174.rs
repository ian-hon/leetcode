use std::{collections::VecDeque, time::Instant};

pub fn run() {
    for i in [
        // "abc".to_string(),
        // "cb34".to_string(),
        // "a8f".to_string(),
        "ag3".to_string()
    ] {
        println!("{}", clear_digits(i));
    }
}

pub fn clear_digits(s: String) -> String {
    let mut s = s.chars().rev();

    let mut result: Vec<char> = vec![];

    let mut count = 0;
    while let Some(c) = s.next() {
        if c.is_ascii_digit() {
            count += 1;
        } else {
            if count >= 1 {
                count -= 1;
                continue;
            }

            result.push(c);
        }
    }

    result.iter().rev().collect::<String>()
}
