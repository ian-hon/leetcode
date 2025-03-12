pub fn run() {
    for i in [

    ] {
        println!("{}", maximum_count(i));
    }
}

pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut p = 0;
    let mut n = 0;

    for i in nums {
        if i > 0 {
            p += 1;
        } else if i < 0 {
            n += 1;
        }
    }

    p.max(n)
}