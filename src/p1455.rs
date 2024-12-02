pub fn run() {
    for i in [
        ("i love eating burger", "burg"),
        ("this problem is an easy problem", "pro"),
        ("i am tired", "you")
    ].into_iter().map(|x| (x.0.to_string(), x.1.to_string())).collect::<Vec<(String, String)>>() {
        println!("{}", is_prefix_of_word(i.0, i.1));
    }
}

pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    let words = sentence.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();

    for i in 0..words.len() {
        let w = words[i].clone();

        if w.strip_prefix(&search_word).is_some() {
            return i as i32 + 1;
        }
    }

    -1
}
