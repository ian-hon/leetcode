use std::collections::HashMap;

pub fn run() {
    for i in [
        ("abba", "dog cat cat dog")
    ].into_iter().map(|(n, m)| (n.to_string(), m.to_string())).collect::<Vec<(String, String)>>() {
        println!("{}", word_pattern(i.0, i.1));
    }
}

pub fn word_pattern(pattern: String, s: String) -> bool {
    if pattern.len() != s.split(" ").count() {
        return false;
    }

    let mut char_set: HashMap<char, String> = HashMap::new();
    let mut word_set: HashMap<String, char> = HashMap::new();

    let p = pattern.chars().collect::<Vec<char>>();
    for (index, word) in s.split(" ").enumerate() {
        let c = p[index];
        let word = word.to_string();

        match char_set.get(&c) {
            Some(w) => {
                if *w != word {
                    return false;
                }
            },
            None => {
                char_set.insert(c, word.clone());
            }
        }
        match word_set.get(&word) {
            Some(ch) => {
                if *ch != c {
                    return false;
                }
            },
            None => {
                word_set.insert(word, c);
            }
        }
    }

    true
}
