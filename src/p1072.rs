use std::collections::HashMap;

pub fn run() {
    for i in [
        [[0,1],[1,1]].iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>(),
        [[0,1],[1,0]].iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>(),
        [[0,0,0],[0,0,1],[1,1,0]].iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>(),
        [[0],[1],[0],[0],[1],[1],[1],[1],[0],[1]].iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>(),

    ] {
        println!("{}", max_equal_rows_after_flips(i));
    }
}

pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    // [(l if (l[0] == 0) else [[1, 0][i] for i in l]) for l in matrix]
    let matrix = matrix.iter().map(|x| if x[0] == 0 { x.clone() } else { x.iter().map(|x| if *x == 0 { 1 } else { 0 } ).collect() }).collect::<Vec<Vec<i32>>>();
    let mut map: HashMap<Vec<i32>, i32> = HashMap::new();

    for i in matrix {
        if map.contains_key(&i) {
            *map.get_mut(&i).unwrap() += 1;
            continue;
        }
        map.insert(i.clone(), 1);
    }

    *map.values().max().unwrap()
}


