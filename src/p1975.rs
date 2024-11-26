pub fn run() {
    for i in [
        vec![vec![1,-1],vec![-1,1]],
        vec![vec![1,2,3],vec![-1,-2,-3],vec![1,2,3]],
        vec![vec![-1,0,-1],vec![-2,1,3],vec![3,2,2]],
        vec![vec![-3,0,0],vec![0,0,0],vec![0,3,2]]
        /*
        -3  0  0
         0  0  0
         0  3  2


         */
    ] {
        println!("{}", max_matrix_sum(i));
    }
}

pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut sum = 0;
    let mut smallest = (matrix[0][0] as i64).abs();
    let mut count = 0;
    for row in matrix {
        for item in row {
            if item <= 0 {
                count += 1;
            }
            let item = item.abs() as i64;
            sum += item;

            if item < smallest {
                smallest = item;
            }
        }
    }

    if (count % 2) == 1 {
        sum -= smallest * 2;
    }

    sum
}