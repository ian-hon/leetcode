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
    let mut result = 0;

    nums.sort();
    let mut map = nums.iter().map(|x| vec![*x]).collect::<Vec<Vec<i32>>>();
    
    for (index, n) in nums.iter().enumerate() {
        let mut highest = (0, 0);
        let mut flag = false;
        for i in 0..index {
            if (*n % nums[i]) == 0 {
                if map[i].len() >= highest.1 {
                    flag = true;
                    highest.0 = i;
                    highest.1 = map[i].len();
                } 
            }
        }
        if flag {
            map[index] = map[highest.0].clone();
            map[index].push(*n);
        }

        if map[result].len() < map[index].len() {
            result = index;
        }
    }

    map[result].clone()
}