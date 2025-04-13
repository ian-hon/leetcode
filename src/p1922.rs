pub fn run() {
    for i in [
        1,
        4,
        50
    ] {
        println!("{}", count_good_numbers(i));
    }
}

pub fn count_good_numbers(n: i64) -> i32 {
    let modulo = 1_000_000_007;

    let fours = n / 2;
    let fives = n - fours;

    // bitshifting 4 solution doesnt really help
    //
    // if n == 100
    // bitshift ~64, mod
    // bitshift ~36, mod
    //
    // at most can only bitshift 64 positions
    // each iteration does 2 digits (1 digit -> 2 shifts)
    //
    // if n = 10^15, this will take ~10^14 iterations

    (((power(5, fives, modulo) % modulo) * (power(4, fours, modulo) % modulo)) % modulo) as i32
}

pub fn power(mut x: i64, mut y: i64, m: i64) -> i64 {
    let mut res = 1;

    x = x % m; // Update x if it is more than or equal to p
    if x == 0 {
        return 0;
    }

    while y > 0 {
        if (y & 1) == 1 {
            res = (res*x) % m;
        }
        
        y = y >> 1;
        x = (x * x) % m;
    }
    return res;
}
