pub fn run() {
    for i in [
        "110".to_string(),
        "001011".to_string()
    ] {
        println!("{:?}", min_operations(i));
    }
}

pub fn min_operations(boxes: String) -> Vec<i32> {
    let n = boxes.len();

    let mut result = vec![];

    let mut indices = vec![];
    for (index, c) in boxes.chars().enumerate() {
        if c == '0' {
            continue;
        }
        indices.push(index);
    }

    for i in 0..n {
        let mut r = 0;
        for index in &indices {
            r += (i as i32 - *index as i32).abs();
        }
        result.push(r);
    }

    result
}