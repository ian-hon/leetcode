pub fn run() {
    for i in [
        vec![vec![3,2],vec![4,3],vec![4,4],vec![2,5]],
        vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5]],
        vec![vec![21,5],vec![92,3],vec![74,2],vec![39,4],vec![58,2],vec![5,5],vec![49,4],vec![65,3]]
    ] {
        println!("{}", most_points(i));
    }
}

pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut map = vec![-1i64; questions.len()];

    backtrack(0, &questions, &mut map)
}

pub fn backtrack(index: usize, q: &Vec<Vec<i32>>, map: &mut Vec<i64>) -> i64 {
    if index >= q.len() {
        return 0
    }

    if map[index] != -1 {
        return map[index];
    }

    let result = (backtrack(q[index][1] as usize + index + 1, q, map) + q[index][0] as i64).max(backtrack(index + 1, q, map));
    map[index] = result;
    result
}
