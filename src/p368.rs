pub fn run() {
    for i in [
        vec![1,2,3],
        vec![1,2,4,8],
        vec![1, 16, 7, 8, 4],
        vec![3,4,16,8]
    ] {
        println!("{:?}", largest_divisible_subset(i));
    }
}

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    nums.sort();
    let mut dp = nums.iter().map(|x| vec![*x]).collect::<Vec<Vec<i32>>>();
    
    for (index, n) in nums.iter().enumerate() {
        for i in 0..index {
            if (*n % nums[i]) == 0 {
                if dp[i].len() >= dp[index].len() {
                    dp[index] = dp[i].clone();
                    dp[index].push(*n);
                } 
            }
        }
        
        if result.len() < dp[index].len() {
            result = dp[index].clone();
        }
    }

    result
}