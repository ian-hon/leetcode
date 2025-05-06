pub fn run() {
    for i in [
        vec![0,2,1,5,3,4],
        vec![5,0,1,2,3,4]
    ] {
        println!("{:?}", build_array(i));
    }
}

pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|n| nums[*n as usize]).collect::<Vec<i32>>()
}