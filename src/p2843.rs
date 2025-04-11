pub fn run() {
    for i in [
        (1, 100),
        (1200, 1230)
    ] {
        println!("{}", count_symmetric_integers(i.0, i.1));
    }
}

pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    let mut result = 0;

    for i in low..=high {
        if (i <= 10) || ((i <= 1000) && (i >= 100)) {
            continue;
        }

        if i <= 100 {
            // only two digits
            let first = i / 10;
            let second = i - (first * 10);
            if first == second {
                result += 1;
            }
        } else {
            // only 4 digits
            let first = i / 1000;
            let second = (i / 100) - (first * 10);
            let third = (i / 10) - ((first * 100) + (second * 10));
            let fourth = i - ((first * 1000) + (second * 100) + (third * 10));
            if (first + second) == (third + fourth) {
                result += 1;
            }
        }
    }

    result
}