pub fn run() {
    for i in [
        "IIIDIDDD".to_string(),
        "DDD".to_string()
    ] {
        println!("{}", smallest_number(i));
    }
}

pub fn smallest_number(pattern: String) -> String {
    let sequence = pattern.chars().into_iter().map(|x| x == 'I').collect::<Vec<bool>>();

    // has to be in a 0..9 loop for the first index
    for i in 1..=9 {
        let mut result = vec![i as i32];
        let mut map = (0..11).map(|_| false).collect::<Vec<bool>>();
        map[i] = true;

        if dfs(0, &sequence, &mut map, &mut result) {
            return result.iter().map(|x| x.to_string()).collect::<String>();
        }
        map[i] = false;
    }

    "".to_string()
}

fn dfs(index: usize, sequence: &Vec<bool>, map: &mut Vec<bool>, result: &mut Vec<i32>) -> bool {
    if index >= sequence.len() {
        return true;
    }

    let item = result[result.len() - 1];
    let range = if sequence[index] { (item as usize + 1)..10 } else { 1..(item as usize) };
    
    for i in range {
        if map[i] {
            continue;
        }
        map[i] = true;
        result.push(i as i32);
        if dfs(index + 1, sequence, map, result) {
            return true;
        }
        result.pop();
        map[i] = false;
    }

    false
}