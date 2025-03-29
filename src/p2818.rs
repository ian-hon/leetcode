use std::collections::{BinaryHeap, HashSet, VecDeque};

pub fn run() {
    for i in [
        // (vec![8,3,9,3,8], 2),
        // (vec![19,12,14,6,10,18], 3),
        // (vec![3289,2832,14858,22011], 6),
        // (vec![1,1,2,18,1,9,3,1], 32),
        (vec![1,75866,1,92619,1334,29568,62581,53130,94710,72816,87780,67830,20930,49559,50505,63669,33660,42252,457,17339,13668,73583,94603,82062,1,21090,8101,90146,86195,43839,7460,30690,21661,1,1,12680,68710,96288,57558,10920,94613,1,59960,1,63389,19264,51409], 106)
    ] {
        println!("{}", maximum_score(i.0, i.1));
    }
}

pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let mut k = k;
    let length = nums.len();

    let scores = nums.iter().map(|x| get_prime_score(*x)).collect::<Vec<i32>>();

    let mut monotonic_stack: VecDeque<(i32, usize)> = VecDeque::new();
    let mut left = vec![-1; length];
    let mut right = vec![length as i32; length];

    for (index, n) in scores.into_iter().enumerate() {
        if monotonic_stack.is_empty() {
            monotonic_stack.push_back((n, index));
            continue;
        }
        // let top = *monotonic_stack.back().unwrap();

        while (!monotonic_stack.is_empty()) && (n > monotonic_stack.back().unwrap().0) {
            let removed = monotonic_stack.pop_back().unwrap();

            right[removed.1] = index as i32;
        }

        if !monotonic_stack.is_empty() {
            left[index] = monotonic_stack.back().unwrap().1 as i32;
        }

        monotonic_stack.push_back((n, index));
    }

    let mut heap = BinaryHeap::from(
        nums
            .iter()
            .enumerate()
            .map(
                |(index, x)| (*x, index)
            )
            .collect::<Vec<(i32, usize)>>()
    );

    // println!("{left:?}");
    // println!("{right:?}");

    let mut result = 1;
    while k > 0 {
        let top = heap.pop().unwrap();

        let mut count = (top.1 as i64 - left[top.1] as i64) * (right[top.1] as i64 - top.1 as i64);
        count = count.min(k as i64);

        result = (result * mod_pow(top.0 as i64, count as i64, 1_000_000_007)) % 1_000_000_007;

        k -= count as i32;
    }

    result as i32
}

pub fn get_prime_score(num: i32) -> i32 {
    // stolen lol
    let mut num = num;
    let mut primes = HashSet::new();
    let mut factor = 2;
    while factor * factor <= num {
        while num % factor == 0 {
            primes.insert(factor);
            num /= factor;
        }
        factor += 1;
    }
    if num > 1 {
        primes.insert(num);
    }
    primes.len() as i32
}

fn mod_pow(a: i64, mut n: i64, p: i64) -> i64 {
    // stolen lol
    let mut result = 1;
    let mut base = a % p;
    while n > 0 {
        if n & 1 == 1 {
            result = (result * base) % p;
        }
        base = (base * base) % p;
        n >>= 1;
    }
    result
}
