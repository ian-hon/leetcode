pub fn run() {
    println!("{}", my_sqrt(2147395600));
//     for i in 0..15 {
//         println!("{i} {}", my_sqrt(i));
//     }
}

pub fn my_sqrt(x: i32) -> i32 {
    let mut i: i64 = 0;
    let x: i64 = (x as i64) + 1;
    loop {
        if (i * i) >= x {
            return (i - 1) as i32;
        }
        i += 1;
    }
}
