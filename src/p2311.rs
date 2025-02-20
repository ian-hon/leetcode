pub fn run() {
    for i in [
        // ("1001010".to_string(), 5),
        // ("00101001".to_string(), 1),
        ("111100010000011101001110001111000000001011101111111110111000011111011000010101110100110110001111001001011001010011010000011111101001101000000101101001110110000111101011000101".to_string(), 11713332)
    ] {
        println!("{}", longest_subsequence(i.0, i.1));
    }
}

pub fn longest_subsequence(s: String, k: i32) -> i32 {
    let mut s_iter = s.chars().rev();

    let mut value = 0;
    let mut increment = 1u128;

    let mut taken = 0;

    while let Some(c) = s_iter.next() {
        if c == '0' {
            taken += 1;
        } else {
            if (value + increment) <= k as u128 {
                value += increment;
                taken += 1;
            }
        }

        if increment >= 158456325028528675187087900672 { // random number
            continue;
        }
        increment <<= 1;
    }

    taken

    // let length = s.len();
    // let mut s = s.chars().rev().enumerate();
    // let mut increment = 1; // 0b00000001

    // let mut taken = 0;
    // let mut value = 0;

    // while let Some((index, c)) = s.next() {
    //     println!("{increment} {c}");

    //     if (index + (taken as usize)) >= length {
    //         return taken;
    //     }

    //     if c == '1' {
    //         if (value + increment) > k {
    //             println!("broken here");
    //             return taken;
    //         }

    //         taken += 1;
    //         value += increment;
    //     }

    //     increment <<= 1;
    // }

    // taken
}