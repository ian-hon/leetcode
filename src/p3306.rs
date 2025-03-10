use std::collections::HashMap;

pub fn run() {
    for i in [
        ("aeioqq", 1),
        ("aeiou", 0),
        ("ieaouqqieaouqq", 1)
    ] {
        println!("{}", count_of_substrings(i.0.to_string(), i.1));
    }
}

pub fn count_of_substrings(word: String, k: i32) -> i64 {
    let chars = word.chars().collect::<Vec<char>>();

    at_least(&chars, k) - at_least(&chars, k + 1)
}

pub fn at_least(chars: &Vec<char>, k: i32) -> i64 {
    let is_vowel = |x: char| (x == 'a') || (x == 'e') || (x == 'i') || (x == 'o') || (x == 'u');

    let l = chars.len();

    let mut map: HashMap<char, i64> = HashMap::new();
    let mut consonant_count = 0;

    let mut vowel_reached = false;

    let mut result = 0;

    let mut left = 0;
    let mut right = 0;
    while (right < l) || (left < l) {
        if (consonant_count >= k) && vowel_reached {
            // increment left
            result += (l - right + 1) as i64;

            let next = chars[left];
            if is_vowel(next) {
                let m = map.get_mut(&next).unwrap();
                *m -= 1;
                if *m <= 0 {
                    map.remove(&next);
                    vowel_reached = map.len() == 5;
                }
            } else {
                consonant_count -= 1;
            }
            left += 1;
        } else {
            // consume forwards
            if right >= l {
                break;
            }

            let next = chars[right];
            if is_vowel(next) {
                match map.get_mut(&next) {
                    Some(m) => {
                        *m += 1;
                    },
                    None => {
                        map.insert(next, 1);
                        vowel_reached = map.len() == 5;
                    }
                }
            } else {
                consonant_count += 1;
            }

            right += 1;
        }
    }

    result
}