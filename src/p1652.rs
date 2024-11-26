pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    // consider memoisation

    let mut result = vec![];

    let size = code.len() as i32;

    for i in 0..size {
        let mut i= i as i32;
        if k > 0 {
            i += 1;
        }
        let mut start = i;
        let mut end = i + k;

        if k < 0 {
            start = i + k;
            end = i;
        }

        let mut sum = 0;
        for s in (start..end).map(|x| fetch(&code, x, size)).collect::<Vec<i32>>() {
            sum += s;
        }
        result.push(sum);
    }

    result
}

fn fetch(c: &Vec<i32>, i: i32, size: i32) -> i32 {
    if i < 0 {
        let n = (-i) % size;
        if n == 0 {
            return 0;
        }
        return c[(size - ((-i) % size)) as usize];
    }

    if i >= size {
        return c[(i % size) as usize];
    }

    c[i as usize]
}

pub fn run() {
    for i in [
        ([5,7,1,4], 3),
        ([1,2,3,4], 0),
        ([2,4,9,3], -2)
    ] {
        println!("{:?}", decrypt(i.0.to_vec(), i.1));
    }
}