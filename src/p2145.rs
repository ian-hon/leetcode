pub fn run() {
    for i in [
        (vec![1,-3,4], 1, 6),
        (vec![3,-4,5,1,-2], -4, 5),
        (vec![4,-7,2], 3, 6),
        (vec![-40], -46, 53), // 60
    ] {
        println!("{}", number_of_arrays(i.0, i.1, i.2));
    }
}

pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let mut smallest = 0;
    let mut highest = 0;

    let mut i = 0;
    for d in differences {
        i += d as i64;
        smallest = smallest.min(i);
        highest = highest.max(i);
    }

    // println!("{smallest} : {highest} : {}", highest - smallest);

    ((upper - lower) - (highest - smallest) as i32 + 1).max(0)
}