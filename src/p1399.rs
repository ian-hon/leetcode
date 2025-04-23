pub fn run() {
    for i in [
        // 13,
        // 2,
        // 24,
        33
    ] {
        println!("{}", count_largest_group(i));
    }
}

pub fn count_largest_group(n: i32) -> i32 {
    let mut map = vec![0; 40];
    let mut highest = 0;

    for mut i in 1..=n {
        let mut sum = 0;
        while i > 9 {
            sum += i - ((i / 10) * 10);
            i /= 10;
        }
        sum += i;
        map[sum as usize] += 1;
        highest = highest.max(map[sum as usize]);
    }

    let mut result = 0;
    for i in map {
        if i == highest {
            result += 1;
        }
    }
    result
}