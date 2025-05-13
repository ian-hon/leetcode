use std::collections::VecDeque;

pub fn run() {
    for i in [
        ("abcyy".to_string(), 2),
        ("azbk".to_string(), 1),
        ("jqktcurgdvlibczdsvnsg".to_string(), 7517)
    ] {
        println!("{}", length_after_transformations(i.0, i.1));
    }
}

pub fn length_after_transformations(s: String, t: i32) -> i32 {
    let mut freq = VecDeque::new();
    for _ in 0..26 {
        freq.push_back(0);
    }

    for c in s.chars() {
        freq[(c as usize) - ('a' as usize)] += 1;
    }

    for _ in 0..t {
        let f = freq.pop_back().unwrap() % 1_000_000_007;
        freq.push_front(f);
        freq[1] += f;
    }

    (freq.iter().sum::<i64>() % 1_000_000_007) as i32
}
