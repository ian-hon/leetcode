pub fn run() {
    for i in [
        ("abac", "cab"),
        // ("aaaaaaaa", "aaaaaaaa")
    ] {
        println!("{}", shortest_common_supersequence(i.0.to_string(), i.1.to_string()));
    }
}

pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let lcs = lcs(str1.clone(), str2.clone());

    let str1_chars = str1.chars().collect::<Vec<char>>();
    let str2_chars = str2.chars().collect::<Vec<char>>();

    let mut result = "".to_string();
    let mut a = 0;
    let mut b = 0;
    for c in lcs.chars() {
        while str1_chars[a] != c {
            result.push(str1_chars[a]);
            a += 1;
        }
        while str2_chars[b] != c {
            result.push(str2_chars[b]);
            b += 1;
        }
        result.push(c);
        a += 1;
        b += 1;
    }

    result.push_str(str1.get(a..str1.len()).unwrap());
    result.push_str(str2.get(b..str2.len()).unwrap());

    // for i in a..str1.len() {
    //     result.push(str1[i]);
    // }

    // for i in b..str2.len() {
    //     result.push(str2[i]);
    // }

    result
}

pub fn lcs(a: String, b: String) -> String {
    // basic lcs tabulation
    let mut matrix = vec![vec![("".to_string(), 0); a.len() + 1]; b.len() + 1];
    //   --A--
    // |
    // B
    // |

    for (y, first) in b.chars().enumerate() {
        let y = y + 1;
        for (x, second) in a.chars().enumerate() {
            let x = x + 1;

            if first == second {
                matrix[y][x] = matrix[y - 1][x - 1].clone();
                matrix[y][x].1 += 1;
                matrix[y][x].0.push(first);
            } else {
                let a = matrix[y - 1][x].clone();
                let b = matrix[y][x - 1].clone();

                if a.1 > b.1 {
                    matrix[y][x] = a;
                } else {
                    matrix[y][x] = b;
                }
            }
        }
    }

    matrix[b.len()][a.len()].0.clone()
}
