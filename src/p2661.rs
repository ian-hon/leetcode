use std::collections::HashMap;

pub fn run() {
    for i in [
        (vec![1,3,4,2], vec![vec![1,4],vec![2,3]]),
        (vec![2,8,7,4,1,3,5,6,9], vec![vec![3,2,5],vec![1,4,6],vec![8,7,9]]),
        (vec![6,2,3,1,4,5], vec![vec![5,1],vec![2,4],vec![6,3]]),
        (vec![1,4,5,2,6,3], vec![vec![4,3,5],vec![1,2,6]])
    ] {
        println!("{}", first_complete_index(i.0, i.1));
    }
}

pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let size = (mat[0].len() as i32, mat.len() as i32);

    let mut column_count: Vec<i32> = (0..size.0).map(|_| 0).collect::<Vec<i32>>();
    let mut row_count: Vec<i32> = (0..size.1).map(|_| 0).collect::<Vec<i32>>();

    let mut index_map: HashMap<i32, (usize, usize)> = HashMap::new();

    for (y, row) in mat.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            index_map.insert(*item, (x, y));
        }
    }

    let mut final_count = 0;
    for i in arr {
        let result = index_map.get(&i).unwrap();
        column_count[result.0] += 1;
        row_count[result.1] += 1;

        println!("{final_count}. {column_count:?} : {row_count:?}");

        if (column_count[result.0] >= size.1) || (row_count[result.1] >= size.0) {
            return final_count;
        }

        final_count += 1;
    }

    0
}