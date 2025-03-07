pub fn run() {
    for i in [
        (10, 19),
        (4, 6)
    ] {
        println!("{:?}", closest_primes(i.0, i.1));
    }
}

pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    if left == right {
        return vec![-1, -1];
    }

    let mut all = sieve_of_eratosthenes(right as usize, left as usize).into_iter();

    let previous = all.next();
    if previous.is_none() {
        return vec![-1, -1];
    }
    let mut previous = previous.unwrap();
    let mut smallest = std::i32::MAX;

    let mut result = vec![-1, -1];

    while let Some(n) = all.next() {
        let d = n - previous;
        if smallest > d {
            smallest = d;
            result[0] = previous;
            result[1] = n;
        }

        previous = n;
    }

    result
}

// fn sieve_of_eratosthenes(n: usize, smallest: usize) -> Vec<i32> {
//     let mut map = vec![false; n + 1];
//     let mut result = vec![];

//     for i in 2..=n {
//         if map[i] {
//             continue;
//         }

//         if i >= smallest {
//             result.push(i as i32);
//         }

//         let mut c = i;
//         while c <= n {
//             map[c] = true;
//             c += i;
//         }
//     }

//     result
// }

fn sieve_of_eratosthenes(n: usize, smallest: usize) -> Vec<i32> {
    let mut map = vec![false; n + 1];

    for i in 2..=((n as f64).sqrt() as usize + 1) {
        if map[i] {
            continue;
        }

        let mut c = i * i;
        while c <= n {
            map[c] = true;
            c += i;
        }
    }

    let mut result = vec![];

    for (index, i) in map.into_iter().enumerate() {
        if i {
            continue;
        }
        if index < smallest {
            continue;
        }
        if index == 1 {
            continue;
        }
        result.push(index as i32);
    }

    result
}