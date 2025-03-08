pub fn run() {
    for i in [
        ("WBBWWBBWBW", 7),
        ("WBWBBBW", 2)
    ] {
        println!("{}", minimum_recolors(i.0.to_string(), i.1));
    }
}

pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let k = k as usize;
    let chars = blocks.chars().collect::<Vec<char>>();
    let length = chars.len();
    let mut highest = 0i32;

    for i in 0..k {
        if chars[i] == 'B' {
            highest += 1;
        }
    }

    let mut current = highest;
    for i in k..length {
        if chars[i] == 'B' {
            current += 1;
        }

        if chars[i - k] == 'B' {
            current -= 1;
        }

        highest = highest.max(current);
    }
    
    let r = (k as i32) - highest;
    if r < 0 {
        return 0;
    }
    r
}
