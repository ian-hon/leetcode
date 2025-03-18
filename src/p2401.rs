pub fn run() {
    for i in [
        vec![1,3,8,48,10],
        vec![3,1,5,11,13],
        vec![744437702,379056602,145555074,392756761,560864007,934981918,113312475,1090,16384,33,217313281,117883195,978927664],
        vec![84139415,693324769,614626365,497710833,615598711,264,65552,50331652,1,1048576,16384,544,270532608,151813349,221976871,678178917,845710321,751376227,331656525,739558112,267703680]
    ] {
        println!("{}", longest_nice_subarray(i));
    }
}

pub fn _longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut current = 0;

    let mut nums_iter = nums.iter();
    let mut current_pool = *nums_iter.next().unwrap();

    while let Some(n) = nums_iter.next() {
        if (current_pool ^ *n) == (current_pool + n) {
            current_pool ^= n;
            if current == 0 {
                current = 2
            } else {
                current += 1;
            }
            result = result.max(current);
        } else {
            current_pool = *n;
            current = 0;
        }

        println!("{} {} {}", format!("{current_pool:032b}").replace("0", " "), if (format!("{current_pool:032b}").replace("0", " ") == format!("{n:032b}").replace("0", " ")) { ":" } else { "=" }, format!("{n:032b}").replace("0", " "));
    }

    if result == 0 {
        return 1;
    }
    result
}

pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut left =  0;
    let mut right = 1;

    let mut result = 1;

    let mut pool = nums[0];

    while right < nums.len() {
        let n = nums[right];

        while (pool & n) != 0 {
            // remove from left
            pool ^= nums[left];
            left += 1;
        }
        pool ^= n;
        right += 1;

        result = result.max(right - left);
    }

    result as i32
}