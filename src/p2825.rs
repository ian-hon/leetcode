use std::collections::HashSet;

pub fn run() {
    for i in [
        // ("abc", "ad"),
        ("ab", "a"),
    //103, 113,  107, 110
        ("gq", "kn"),
        ("z", "z")
        // ("eao", "ofa")
    ].into_iter().map(|x| (x.0.to_string(), x.1.to_string())).collect::<Vec<(String, String)>>() {
        println!("{}", can_make_subsequence(i.0, i.1));
    }
}

pub fn can_make_subsequence_(str1: String, str2: String) -> bool {
    let c = str1.as_bytes().to_vec();
    let mut target: HashSet<u8> = HashSet::from_iter(str2.as_bytes().to_vec().into_iter());

    let mut differences: HashSet<u8> = HashSet::new();
    for e in c {
        if target.contains(&e) {
            target.remove(&e);
        } else {
            differences.insert(e);
        }
    }

    if target.len() == 0 {
        return true;
    }

    println!("{target:?}");

    let mut wanted = target.drain().into_iter().collect::<Vec<u8>>();

    for w in &mut wanted {
        *w -= 1;

        if *w < 97 {
            *w = 122;
        }

        if !differences.contains(&w) {
            return false;
        }
    }

    true
}

pub fn can_make_subsequence(str1: String, str2: String) -> bool {
    let mut c = str1.as_bytes().to_vec();
    let target = str2.as_bytes().to_vec();

    let mut plate: Vec<u8> = vec![];
    for t in &target {
        for e in c.clone() {
            c.remove(0);
            if is_convertable(e, *t) {
                plate.push(e);
                break;
            }
        }
    }

    plate.len() == target.len()

    // for window in c.windows(target.len()) {
    //     let mut validity = true;
    //     for (i, t) in target.iter().enumerate() {
    //         if window[i] == 122 {
    //             if (*t == 97) || (*t == 122) {
    //                 continue;
    //             }
    //         }

    //         if window[i] > *t {
    //             validity = false;
    //             break;
    //         }

    //         if (*t - window[i]) > 1 {
    //             validity = false;
    //             break;
    //         }
    //     }
    //     if validity {
    //         return true;
    //     }
    // }

    // false



    // differences.contains(&wanted)

    // for (i, e) in c.clone().iter().enumerate() {
    //     c[i] = *e + 1;
    //     if c[i] >= 122 {
    //         c[i] = 97;
    //     }
    // }

    // for w in c.windows(target.len()) {
    //     if w == target {
    //         return true;
    //     }
    // }

    // false
}

fn is_convertable(i: u8, t: u8) -> bool {
    if i == 122 {
        return (t == 97) || (t == 122);
    }

    if i > t {
        return false;
    }

    (t - i) <= 1
}
