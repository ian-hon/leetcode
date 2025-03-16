pub fn run() {
    for i in [
        // (vec![4,2,3,1], 10),
        // (vec![5,1,8], 6),
        (vec![31,31,5,19,19,10,31,18,19,3,16,20,4,16,2,25,10,16,23,18,21,23,28,6,7,29,11,11,19,20,24,19,26,12,29,29,1,14,17,26,24,7,11,28,22,14,31,12,3,19,16,26,11], 736185)
        // 2358388332
    ] {
        println!("{}", repair_cars(i.0, i.1));
    }
}

pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    // t = r * n * n
    // (t / r)^0.5 = n

    let cars = cars as i64;

    let mut left = 1i64;
    let mut right = (*ranks.iter().min().unwrap() as i64) * cars * cars;

    'outer: while left < right {
        let mid = (left + right) / 2;

        let mut total = 0;
        for r in &ranks {
            total += ((mid / *r as i64) as f64).sqrt() as i64;
            // probably a faster way to do this?
            if total >= cars {
                right = mid;
                continue 'outer;
            }
        }

        left = mid + 1;
    }

    left
}