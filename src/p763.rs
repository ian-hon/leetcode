use std::collections::{HashMap, HashSet};

pub fn run() {
    for i in [
        "ababcbacadefegdehijhklij".to_string(),
        "eccbbbbdec".to_string()
    ] {
        println!("{:?}", partition_labels(i));
    }
}

pub fn partition_labels(s: String) -> Vec<i32> {
    let s = s.chars().collect::<Vec<char>>();

    let ascii_start = 'a' as usize;
    let mut frequency_map = vec![0; 26];
    s.iter().for_each(|c| frequency_map[*c as usize - ascii_start] += 1);

    let mut result = vec![];

    let mut current_map: HashSet<char> = HashSet::new();
    let mut current_length = 0;

    for char in s {
        current_length += 1;
        frequency_map[char as usize - ascii_start] -= 1;
        current_map.insert(char);

        if frequency_map[char as usize - ascii_start] != 0 {
            continue;
        }

        // do the check here
        let mut flag = true;
        for key in &current_map {
            if frequency_map[*key as usize - ascii_start] != 0 {
                flag = false;
                break;
            }
        }

        if flag {
            // no more occurances
            result.push(current_length);
            current_map.clear();
            current_length = 0;
        }
    }

    result
}
