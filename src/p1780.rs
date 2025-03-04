pub fn run() {
    for i in [
        12,
        91,
        21
    ] {
        println!("{}", check_powers_of_three(i));
    }
}

pub fn check_powers_of_three(n: i32) -> bool {
    let n = n as u32;

    for c in format_radix(n, 3).chars() {
        if c == '2' {
            return false;
        }
    }

    true
}

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}