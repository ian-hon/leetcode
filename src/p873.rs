use std::collections::HashSet;

pub fn run() {
    for i in [
        vec![1,2,3,4,5,6,7,8],
        vec![1,3,7,11,12,14,18],
        vec![1,3,5],
    ] {
        println!("{}", len_longest_fib_subseq(i));
    }
}

pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let mut result = 0;

    let mut map = HashSet::new();
    for i in &arr {
        map.insert(i);
    }

    for first in 0..arr.len() {
        for second in (first + 1)..arr.len() {
            let mut a = arr[first];
            let mut b = arr[second];
            let mut length = 0;
            while map.contains(&(a + b)) {
                (a, b) = (b, a + b);
                length += 1;
            }
            result = result.max(length);
        }
    }

    if result == 0 {
        return 0;
    }

    result + 2
}