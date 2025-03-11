use std::collections::HashMap;

pub fn run() {
    for i in [
        "abcabc",
        "aaacb",
        "abc"
    ] {
        println!("{}", number_of_substrings(i.to_string()));
    }
}

pub fn number_of_substrings(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let a = 'a' as usize;

    let mut map: Vec<i32> = vec![0; 3];

    let width = chars.len();
    let mut result = 0;
    let mut i = 0;
    let mut j = 0;

    while j < width {
        map[(chars[j] as usize) - a] += 1;

        while (map[0] >= 1) && (map[1] >= 1) && (map[2] >= 1) {
            result += width - j;

            map[(chars[i] as usize) - a] -= 1;
            i += 1;
        }

        j += 1;
    }

    result as i32
}