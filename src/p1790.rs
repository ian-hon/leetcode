use std::collections::HashSet;

pub fn run() {
    for i in [
        ("bank", "kanb"),
        ("attack", "defend"),
        ("kelb", "kelb"),
        ("aa", "ac"),
        ("aa", "bb")
    ] {
        println!("{}", are_almost_equal(i.0.to_string(), i.1.to_string()));
    }
}

pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let mut s1 = s1.chars();
    let mut s2 = s2.chars();

    let mut differences_a = HashSet::new();
    let mut differences_b = HashSet::new();

    let mut count = 0;

    while let Some(a) = s1.next() {
        let b = s2.next().unwrap();

        if a != b {
            differences_a.insert(a);
            differences_b.insert(b);

            count += 1;
            if count >= 3 {
                return false;
            }
        }
    }

    (differences_a == differences_b) &&
    ((count == 0) || (count == 2))
}