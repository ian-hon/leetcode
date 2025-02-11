pub fn run() {
    for i in [
        ("daabcbaabcbc", "abc"),
        ("axxxxyyyyb", "xy"),
        ("aabababa", "aba")
    ] {
        println!("{}", remove_occurrences(i.0.to_string(), i.1.to_string()));
    }
}


pub fn remove_occurrences(s: String, part: String) -> String {
    let mut s = s;
    loop {
        let n = s.replacen(&part, "", 1);
        if s == n {
            return s;
        }

        s = n;
    }
}
