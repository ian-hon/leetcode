pub fn run() {
    for i in [

    ] {
        println!("{}", min_partitions(i));
    }
}

pub fn min_partitions(n: String) -> i32 {
    let mut highest = 0;

    for c in n.chars() {
        highest = highest.max(c.to_digit(10).unwrap());
    }

    highest as i32
}