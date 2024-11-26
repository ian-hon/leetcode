pub fn run() {
    for i in [
        (10, 7),
    ] {
        println!("{:?}", min_bit_flips(i.0, i.1));
    }
}

pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    format!("{:b}", start ^ goal).chars().filter(|x| *x == '1').count() as i32
}
