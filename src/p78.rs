pub fn run() {
    for i in [
        [1,2,3].to_vec(),
        [0].to_vec()
    ] {
        println!("{:?}", subsets(i));
    }
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    (0..((2f32.powi(nums.len() as i32) as i32))).into_iter().map(|i| nums.iter().enumerate().filter(|(index, _)| ((i >> index) & 1) == 1).map(|x| *x.1).collect::<Vec<i32>>()).collect()
}
