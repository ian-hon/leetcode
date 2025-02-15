pub fn run() {
    for i in [
        10,
        20,
        37,
        1000
    ] {
        println!("{}", punishment_number(i));
    }
}

pub fn punishment_number(n: i32) -> i32 {
    let mut result = 0;

    for i in 1..=n {
        let m = i % 9;
        if !((m == 0) || (m == 1)) {
            continue;
        }

        if partitionable((i*i).to_string(), i) {
            result += i * i;
        }
    }

    result
}

pub fn partitionable(i: String, target: i32) -> bool {
    if (i == "") && (target == 0) { // exactly fits
        return true;
    }
    if target < 0 { // exceeded, cant continue further
        return false;
    }
    if i.parse::<i32>().unwrap() == target {
        return true;
    }

    for n in 1..i.len() {
        let (left, right) = i.split_at(n);
        // 1, 296
        // 12, 96
        // 129, 6

        let left = left.parse::<i32>().unwrap();
        if partitionable(right.to_string(), target - left) {
            return true;
        }
    }

    false
}
