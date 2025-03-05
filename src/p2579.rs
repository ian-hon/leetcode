pub fn run() {
    for i in [

    ] {
        println!("{}", colored_cells(i));
    }
}

pub fn colored_cells(n: i32) -> i64 {
    let n = n as i64;
    (2 * ((n - 1).pow(2))) + (2 * (n - 2)) + 3
}