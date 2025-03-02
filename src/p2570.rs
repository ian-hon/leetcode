use std::collections::HashMap;

pub fn run() {
    for i in [
        (vec![vec![1,2],vec![2,3],vec![4,5]], vec![vec![1,4],vec![3,2],vec![4,1]]),
        (vec![vec![2,4],vec![3,6],vec![5,5]], vec![vec![1,3],vec![4,3]])
    ] {
        println!("{:?}", merge_arrays(i.0, i.1));
    }
}

pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut index_map = HashMap::new();

    for i in nums1 {
        index_map.insert(i[0], result.len());
        result.push(i.clone());
    }

    for i in nums2 {
        match index_map.get(&i[0]) {
            Some(item) => {
                result[*item][1] += i[1];
            },
            None => {
                result.push(i);
            }
        }
    }

    result.sort_by_key(|x| x[0]);
    
    result
}