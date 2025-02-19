pub fn run() {
    for i in [
        50, 2
    ] {
        println!("{:?}", even_odd_bit(i));
    }
}

pub fn even_odd_bit(n: i32) -> Vec<i32> {
    let mut result = vec![0, 0];

    for i in 0..11 {
        if ((n >> i) & 1) == 0 {
            continue;
        }

        if (i & 1) == 0 {
            // even
            result[0] += 1;
        } else {
            // odd
            result[1] += 1;
        }
    }

    result
}