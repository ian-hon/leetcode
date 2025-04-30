pub fn run() {
    for i in [
        vec![12,345,2,6,7896],
        vec![555,901,482,1771]
    ] {
        println!("{}", find_numbers(i));
    }
}

pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for n in nums {
        if 
            ((n >= 10) && (n <= 99)) ||
            ((n >= 1000) && (n <= 9999)) ||
            (n >= 100_000)
        {
            result += 1;
        }
    }

    result
}