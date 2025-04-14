pub fn run() {
    for i in [
        (vec![3,0,1,1,9,7], 7, 2, 3),
        (vec![1,1,2,2,3], 0, 0, 1)
    ] {
        println!("{}", count_good_triplets(i.0, i.1, i.2, i.3));
    }
}

pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let l = arr.len();
    let mut result = 0;

    for x in 0..l {
        let first = arr[x];
        for y in (x + 1)..l {
            let second = arr[y];

            if (second - first).abs() > a {
                continue;
            }

            for z in (y + 1)..l {
                let third = arr[z];

                if (third - second).abs() > b {
                    continue;
                }

                if (third - first).abs() > c {
                    continue;
                }
                
                result += 1;
            }
        }
    }

    result
}