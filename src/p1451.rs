use std::collections::HashMap;

pub fn run() {
    for i in [
        "This is leetcode"
    ] {
        println!("{}", arrange_words(i.to_string()));
    }
}

pub fn arrange_words(text: String) -> String {
    let mut map: HashMap<usize, String> = HashMap::new();
    let text = text.to_lowercase();

    for i in text.split(" ") {
        let l = i.len();
        match map.get_mut(&l) {
            Some(c) => {
                c.push(' ');
                c.push_str(i);
            },
            None => {
                map.insert(l, i.to_string());
            }
        }
    }

    let mut result = "".to_string();

    let mut map = map.into_iter().collect::<Vec<(usize, String)>>();
    map.sort_by_key(|x| x.0);

    let mut first = false;
    for (_, v) in map {
        if !first {
            let mut v = v;
            v.replace_range(0..1, v.get(0..1).unwrap().to_ascii_uppercase().as_str());
            result.push_str(v.as_str());
            first = true;
            continue;
        }
        result.push(' ');
        result.push_str(v.as_str());
    }

    result
}