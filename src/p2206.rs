pub fn run() {
    for i in [
        vec![3,2,3,2,2,2],
        vec![1,2,3,4]
    ] {
        println!("{}", divide_array(i));
    }
}

pub fn divide_array(nums: Vec<i32>) -> bool {
    let mut map = vec![false; 500];

    for i in nums {
        let i = i as usize;
        map[i] = !map[i];
    }

    for i in map {
        if i {
            return false;
        }
    }

    true
}