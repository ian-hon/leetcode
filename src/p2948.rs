use std::collections::VecDeque;

pub fn run() {
    for i in [
        (vec![1,5,3,9,8], 2),
        (vec![1,7,6,18,2,1], 3),
        (vec![1,7,28,19,10], 3)
    ] {
        println!("{:?}\n\n", lexicographically_smallest_array(i.0, i.1));
    }
}

pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let mut group_index = vec![0; nums.len()];
    let mut result = vec![];

    let mut sorted_nums = nums.iter().enumerate().map(|x| (x.0, *x.1)).collect::<Vec<(usize, i32)>>();
    sorted_nums.sort_by_key(|x| x.1);

    let mut groups: Vec<VecDeque<i32>> = vec![];
    let mut bucket = VecDeque::new();
    let mut previous = sorted_nums[0].1;
    for i in sorted_nums.clone() {
        if (previous - i.1).abs() <= limit {
            group_index[i.0] = groups.len();
            bucket.push_back(i.1);
        } else {
            groups.push(bucket);
            bucket = VecDeque::new();
            bucket.push_back(i.1);
            group_index[i.0] = groups.len();
        }

        previous = i.1;
    }
    groups.push(bucket);

    for i in &group_index {
        result.push(groups[*i][0]);
        groups[*i].remove(0);
    }

    result
}