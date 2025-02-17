pub fn run() {
    for i in [
        "AAB".to_string(),
        "AAABBC".to_string()
    ] {
        println!("{}", num_tile_possibilities(i));
    }
}

pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut map = vec![0; 26];
    for c in tiles.chars() {
        map[c as usize - 'A' as usize] += 1;
    }

    let mut result = 0;
    dfs(&mut result, &mut map);
    result - 1
}

fn dfs(i: &mut i32, map: &mut Vec<i32>) -> i32 {
    *i += 1;
    for candidate in 0..26 {
        if map[candidate] <= 0 {
            continue;
        }

        map[candidate] -= 1;
        dfs(i, map);
        map[candidate] += 1;
    }

    *i
}
