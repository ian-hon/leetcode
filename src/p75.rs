pub fn run() {
    for i in [

    ] {
        sort_colors(i);
    }
}

pub fn sort_colors(nums: &mut Vec<i32>) {
    let length = nums.len();
    let mut l = 0;
    let mut r = nums.len();
    for n in &mut *nums {
        if *n == 0 {
            l += 1;
        }
        if *n == 2 {
            r -= 1;
        }
    }

    for i in 0..l {
        nums[i] = 0;
    }

    for i in r..length {
        nums[i] = 2;
    }

    for i in (l + 1)..r {
        nums[i] = 1;
    }
}