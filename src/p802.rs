pub fn run() {
    for i in [
        vec![vec![1,2],vec![2,3],vec![5],vec![0],vec![5],vec![],vec![]],
        // vec![vec![3],vec![0, 2,3],vec![0, ],vec![0],vec![5],vec![],vec![]],
        vec![vec![1,2,3,4],vec![1,2],vec![3,4],vec![0,4],vec![]]
    ] {
        println!("{:?}", eventual_safe_nodes(i));
    }
}

pub fn cyclical(g: &Vec<Vec<i32>>, i: usize, states: &mut Vec<i32>) -> bool {
    match states[i] {
        1 => true,
        2 => false,
        3 => true,
        _ => {
            // unvisited only
            states[i] = 1;

            for children in &g[i] {
                if cyclical(g, *children as usize, states) {
                    states[i] = 3;
                    // child state already set in recursive cyclical()
                    return true;
                }
            }
            states[i] = 2;
            false
        }
    }
}

pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    // heavily inspired by 'safe if non-cyclical' assumption https://leetcode.com/problems/find-eventual-safe-states/solutions/667633/rust-easily-understandable-graph-colouring-cycle-detection-using-dfs
    let mut result = vec![];
    let mut states = vec![0; graph.len()];
    // 0 unvisted,
    // 1 visited,
    // 2 safe,
    // 3 unsafe

    for i in 0..graph.len() {
        if !cyclical(&graph, i, &mut states) {
            result.push(i as i32);
        }
    }

    result
}