pub fn run() {
    for i in [
        vec![3,4,5,1,2],
        vec![2,1,3,4],
        vec![1,2,3]
    ] {
        println!("{}", check(i));
    }
}

pub fn check(nums: Vec<i32>) -> bool {
    let mut broken = false;

    let mut nums_iter = nums.clone().into_iter();
    let mut previous = nums_iter.next().unwrap();

    while let Some(n) = nums_iter.next() {
        if previous > n {
            if broken {
                return false;
            }

            broken = true;
        }

        previous = n;
    }

    if nums[nums.len() - 1] > nums[0] {
        if broken {
            return false;
        }
    }

    true
}