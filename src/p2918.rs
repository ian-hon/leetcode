pub fn run() {
    for i in [
        (vec![3,2,0,1,0], vec![6,5,0]),
        (vec![2,0,2,0], vec![1,4]),
        (vec![0,17,20,17,5,0,14,19,7,8,16,18,6], vec![21,1,27,19,2,2,24,21,16,1,13,27,8,5,3,11,13,7,29,7]),
        (vec![0,0,10,10,12,0,13,6,0,2,10], vec![24,5,12,22])
    ] {
        println!("{}", min_sum(i.0, i.1));
    }
}

pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let mut zero_count_1 = 0;
    let mut sum_1 = 0;
    for n in nums1 {
        if n == 0 {
            zero_count_1 += 1;
        }
        sum_1 += n as i64;
    }

    let mut zero_count_2 = 0;
    let mut sum_2 = 0;
    for n in nums2 {
        if n == 0 {
            zero_count_2 += 1;
        }
        sum_2 += n as i64;
    }

    let lowest_1 = sum_1 + zero_count_1;
    let lowest_2 = sum_2 + zero_count_2;

    if (zero_count_1 == 0) || (zero_count_2 == 0) {
        if (zero_count_1 == 0) && (sum_1 < lowest_2) {
            return -1;
        }

        if (zero_count_2 == 0) && (sum_2 < lowest_1) {
            return -1;
        }
    }

    lowest_1.max(lowest_2)
}