pub fn run() {
    for i in [
        // (1, 3),
        // (1, 4),
        (3, 9)
    ] {
        println!("{}", get_happy_string(i.0, i.1));
    }
}

pub fn get_happy_string(n: i32, k: i32) -> String {
    // possible to just convert k to base3?

    let mut result = vec![];
    let mut index = 0;

    for i in 0..=2 {
        result.push(i);
        let r = dfs(i, &mut result, &mut index, k, n as usize);
        if !r.is_empty() {
            return r.iter().map(|x| (*x as u8 + 97) as char).collect::<String>();
        }
        result.pop();
    }

    "".to_string()
}

pub fn dfs(previous: i32, result: &mut Vec<i32>, index: &mut i32, target: i32, length: usize) -> Vec<i32> {
    if result.len() > length {
        return vec![];
    }

    if result.len() == length {
        *index += 1;

        if target == *index {
            return result.clone();
        }
    }

    for candidate in 0..=2 {
        if candidate == previous {
            continue;
        }

        result.push(candidate);
        let c = dfs(candidate, result, index, target, length);
        if !c.is_empty() {
            return c;
        }
        result.pop();
    }

    vec![]
}