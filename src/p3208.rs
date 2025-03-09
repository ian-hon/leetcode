pub fn run() {
    for i in [
        (vec![0,1,0,1,0], 3),
        (vec![0,1,0,0,1,0,1], 6)
    ] {
        println!("{}", number_of_alternating_groups(i.0, i.1));
    }
}

pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut current = 0;

    let mut flag = true;
    let mut previous = colors[0];
    for n in colors.iter().cycle().take(colors.len() + k as usize - 1) {
        if flag {
            flag = false;
            continue;
        }

        if previous != *n {
            current += 1;
            if current > (k - 2) {
                count += 1;
            }
        } else {
            current = 0;
        }
        previous = *n;
    }

    // for i in 0..(colors.len() + k as usize - 1) {
    //     let n = if i >= l {
    //         colors[i - l]
    //     } else {
    //         colors[i]
    //     };

    //     if previous != n {
    //         current += 1;
    //         if current > (k - 2) {
    //             count += 1;
    //         }
    //     } else {
    //         current = 0;
    //     }
    //     previous = n;
    // }

    count
}