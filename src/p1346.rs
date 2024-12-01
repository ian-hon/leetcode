use std::collections::HashSet;

pub fn run() {
    for i in [
        vec![10,2,5,3],
        vec![3,1,7,11],
        vec![-20,8,-6,-14,0,-19,14,4],
        vec![-2,0,10,-19,4,6,-8]
    ] {
        println!("{}", check_if_exist(i))
    }
}

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    let z = arr.iter().filter(|i| **i == 0).count() >= 2;
    let set: HashSet<i32> = HashSet::from_iter(arr.into_iter());

    for i in &set {
        if set.contains(&(i * 2)) {
            if *i == 0 {
                if z {
                    return true;
                } else {
                    continue;
                }
            }
            return true;
        }
    }

    false
}
