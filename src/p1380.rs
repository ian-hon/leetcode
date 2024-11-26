pub fn run() {
    for i in [
        vec![vec![3,7,8],vec![9,11,13],vec![15,16,17]],
        vec![vec![1,10,4,2],vec![9,3,8,7],vec![15,16,17,12]],
        vec![vec![7,8],vec![1,2]]
    ] {
        println!("{:?}", lucky_numbers(i));
    }
}

pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];

    for row in matrix.iter() {
        for (index, item) in row.iter().enumerate() {
            if item == row.iter().min().unwrap() {
                if *item == matrix.iter().map(|x| x[index]).max().unwrap() {
                    result.push(*item);
                }
            }
        }
    }

    result
}