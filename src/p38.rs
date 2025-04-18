pub fn run() {
    for i in [
        1, // 1
        2, // 11
        3, // 21
        4  // 1211
    ] {
        println!("{}", count_and_say(i));
    }
}

pub fn count_and_say(n: i32) -> String {
    let mut pairs = vec![1 as i32];

    for _ in 0..(n - 1) {
        let mut new_pairs = vec![];

        let mut previous = pairs[0];
        let mut running = 0;
        for item in pairs {
            if previous != item {
                new_pairs.push(running);
                new_pairs.push(previous);
                running = 0;
            }

            previous = item;
            running += 1;
        }

        new_pairs.push(running);
        new_pairs.push(previous);

        pairs = new_pairs;
    }

    let mut result = "".to_string();

    for i in pairs {
        result.push(((i + 48) as u8) as char);
    }

    result
}
