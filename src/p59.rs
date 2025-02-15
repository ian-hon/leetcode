pub fn run() {
    for i in 1..20 {
        println!("{:?}", generate_matrix(i));
    }
}

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut result = vec![vec![1; n as usize]; n as usize];

    for y in 0..n {
        for x in 0..n {
            // yes i stole this
            let level = x.min(n - x - 1).min(y).min(n - y - 1);
            result[x as usize][y as usize] = ((n * n) - ((n - (2 * level)) * (n - (2 * level)))) + 1 + if y >= x {
                (x + y) - (2 * level)
            }
            else {
                (4 * n) - (6 * level) - 4 - x - y
            };
        }
    }

    result
}
