use std::collections::HashMap;

pub fn run() {
    for i in [
        (vec![1,1,1,1,1], 10),
        (vec![3,1,4,3,2,2,4], 2)
    ] {
        println!("{}", count_good(i.0, i.1));
    }
}

pub fn factorial(i: usize, map: &mut Vec<i64>) -> i64 {
    if i >= map.len() {
        for n in map.len()..=i {
            map.push(map[map.len() - 1] * n as i64);
        }
    }
    map[i as usize]
}

pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut left = 0;
    let mut count = 0;
    let mut result = 0i64;

    for right in 0..nums.len() {
        count += map.get(&nums[right]).unwrap_or(&0);
        *map.entry(nums[right]).or_insert(0) += 1;

        while count >= k {
            *map.get_mut(&nums[left]).unwrap() -= 1;
            count -= map.get(&nums[left]).unwrap();
            left += 1;
        }

        result += left as i64;
    }

    result

    // let mut left = 0;
    // let mut right = 0;
    // let width = nums.len() - 1;

    // let mut map: HashMap<i32, i32> = HashMap::new();
    // let mut has_pair = false;

    // let mut fact_map = vec![0, 1, 2, 6];

    // let mut result = 0;
    // 'outer: while right < width {
    //     println!("{left} : {right}\t{} : {}", nums[left], nums[right]);
    //     if has_pair {
    //         // increment left
    //         // let x = (right - left - 1) as i64;
    //         // result += (x.pow(2) - x) / 2;
    //         let x = right - left;
    //         result += factorial(x, &mut fact_map);
    //         left += 1;

    //         let l = map.get_mut(&nums[left]).unwrap();
    //         *l -= 1;
    //         let l = *l;
    //         if l == 0 {
    //             map.remove(&l);
    //         }

    //         has_pair = false;
    //         for (k, v) in &map {
    //             if v >= k {
    //                 has_pair = true;
    //                 continue 'outer;
    //             }
    //         }

    //         // check if theres a pair
    //     } else {
    //         // increment right
    //         right += 1;

    //         match map.get_mut(&nums[right]) {
    //             Some(r) => {
    //                 *r += 1;
    //                 if *r >= k {
    //                     has_pair = true;
    //                 }
    //             },
    //             None => {
    //                 map.insert(nums[right], 1);
    //                 if k == 1 {
    //                     has_pair = true;
    //                 }
    //             }
    //         }
    //     }
    // }

    // result
}