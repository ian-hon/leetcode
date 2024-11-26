use std::collections::HashSet;

pub fn run() {
    for i in [
        vec![vec![1,2,3], vec![4,0,5]],
        vec![vec![4,1,2], vec![5,0,3]],
        vec![vec![1,2,3],vec![5,4,0]]
    ] {
        println!("{}", sliding_puzzle(i));
    }
}

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    if board == vec![vec![1,2,3], vec![4,5,0]] {
        return 0;
    }

    let board: Vec<i32> = board.iter().flatten().map(|x| *x).collect();
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    set.insert(board.clone());

    let target = vec![1,2,3,4,5,0];
    let mut nodes: Vec<Vec<i32>> = vec![board];
    let mut count = 0;
    loop {
        count += 1;

        let mut result: Vec<Vec<i32>> = vec![];
        let mut flag = false;
        for i in nodes.clone() {
            for m in perform_all_moves(&i) {
                if !set.insert(m.clone()) {
                    continue;
                }
                if m == target {
                    return count;
                }
                flag = true;
                result.push(m);
            }
        }

        if !flag {
            // no changes to all nodes, means no possible moves
            return -1;
        }
        nodes = result.clone();
    }
}

pub fn perform_all_moves(n: &Vec<i32>) -> Vec<Vec<i32>> {
    // 4 moves -> u,r,d,

    let index = n.iter().position(|x| *x == 0).unwrap();

    let possible_swaps = vec![
        vec![1, 3], vec![0, 2, 4], vec![1, 5], vec![0, 4], vec![1, 3, 5], vec![2, 4]
    ];

    let mut result = vec![];
    for s in &possible_swaps[index] {
        let mut t = n.clone();
        t[index] = t[*s as usize];
        t[*s as usize] = 0;

        result.push(t);
    }

    result
}
