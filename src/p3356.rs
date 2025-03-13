use std::fs;

pub fn run() {
    for i in [
        (vec![2,0,2], vec![vec![0,2,1],vec![0,2,1],vec![1,1,3]]), // 2
        (vec![4,3,2,1], vec![vec![1,3,2],vec![0,2,1]]), // -1
        (vec![7,6,8], vec![vec![0,0,2],vec![0,1,5],vec![2,2,5],vec![0,2,4]]), // expect 4
        (vec![0,8], vec![vec![0,1,4],vec![0,1,1],vec![0,1,4],vec![0,1,1],vec![1,1,5],vec![0,1,2],vec![1,1,4],vec![0,1,1],vec![1,1,3],vec![0,0,2],vec![1,1,3],vec![1,1,2],vec![0,1,5],vec![1,1,2],vec![1,1,5]]), // 3
        (vec![1,0,6], vec![vec![1,2,1],vec![0,0,4],vec![1,1,5],vec![0,0,5],vec![1,2,4],vec![0,2,2],vec![2,2,4],vec![1,2,2],vec![1,2,4],vec![0,1,3]]), // 6
        // (
        //     serde_json::from_str(&fs::read_to_string("./src/too_damn_long.txt").unwrap().split('\n').next().unwrap()).unwrap(),
        //     serde_json::from_str(&fs::read_to_string("./src/too_damn_long.txt").unwrap().split('\n').last().unwrap()).unwrap()
        // )
    ] {
        println!("{}", min_zero_array(i.0, i.1));
    }
}

pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut difference_map = vec![0; nums.len() + 1];
    let l = queries.len();

    let mut running = 0;
    let mut query_index = 0;

    for index in 0..nums.len() {
        let n = nums[index];
        while running + difference_map[index] < n {
            query_index += 1;
            if query_index > l {
                // no more queries to consume
                return -1;
            }

            let left = queries[query_index - 1][0] as usize;
            let right = queries[query_index - 1][1] as usize;
            let value = queries[query_index - 1][2];
            // struggled here, figured out needed to max(index, left)
            // everything before left is redundant, so just take max of index and left
            // thus, ignore if after right as well
            if right < index {
                continue;
            }
            difference_map[left.max(index)] += value;
            difference_map[right + 1] -= value;
        }
        running += difference_map[index];
    }

    query_index as i32
}