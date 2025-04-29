pub fn run() {
    for i in [
        (vec![1,3,2,3,3], 2),
        (vec![0,3,0,3,3], 2),
        // 1,3,2,3
        // 1,3,2,3,3
        //   3,2,3
        //   3,2,3,3
        //     2,3,3
        //       3,3

        // 1
        // 1,3
        // 1,3,2
        // --
        // 1,3,2,3 +
        //   3,2,3 +
        //     2,3
        // --
        //     2,3,3 +
        //       3,3 +
        //         3
        // 
        // (vec![1,4,2,1], 3),
        // (vec![61,23,38,23,56,40,82,56,82,82,82,70,8,69,8,7,19,14,58,42,82,10,82,78,15,82], 2),
        // (vec![ 0, 0, 0, 0, 0, 0,82, 0,82,82,82, 0,0, 0,0,0, 0, 0, 0, 0,82, 0,82, 0, 0,82], 2)
        (vec![28,5,58,91,24,91,53,9,48,85,16,70,91,91,47,91,61,4,54,61,49], 1)
    ] {
        println!("{}", count_subarrays(i.0, i.1));
    }
}

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let mut result = 0;
    let highest = *nums.iter().max().unwrap();

    let mut running = 0;
    let mut left = 0;
    for (right, n) in nums.iter().enumerate() {
        if *n == highest {
            running += 1;
        }

        while running >= k {
            if nums[left] == highest {
                running -= 1;
            }
            left += 1;

            result += (nums.len() - right) as i64;
        }
    }

    result
}