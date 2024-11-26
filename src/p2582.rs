pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let section = (time - 1) / (n - 1);

    if (section % 2) == 1 {
        return n - (time - (section * (n - 1)));
    }

    time - (section * (n - 1)) + 1
}

pub fn run() {
    /*
    1 2
    2 3
    3 2
    4 1
    5 2
    6 3
    7 2
    8 1
    9 2
     */
    for i in [
        vec![3, 1],
        vec![3, 2],
        vec![3, 3],
        vec![3, 4],
        vec![3, 5],
        vec![3, 6],
        vec![3, 7],
        vec![3, 8],
        vec![3, 9],
        vec![18, 35],
        // vec![4, 5],
        // vec![3, 2],
        // vec![3, 1],
    ] {
        println!("{} : {}", i[1], pass_the_pillow(i[0], i[1]));
    }
}
