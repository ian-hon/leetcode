pub fn run() {
    for i in [
        ("_L__R__R_", "L______RR")
    ].into_iter().map(|x| (x.0.to_string(), x.1.to_string())).collect::<Vec<(String, String)>>() {
        println!("{}", can_change(i.0, i.1));
    }
}

pub fn can_change(start: String, target_s: String) -> bool {
    let mut stripped_start = start.clone();
    stripped_start.retain(|c| c != '_');
    let mut stripped_target = target_s.clone();
    stripped_target.retain(|c| c != '_');
    if stripped_start != stripped_target {
        return false;
    }

    let mut field: Vec<(usize, i32)> = vec![];
    for s in start.chars().into_iter().enumerate() {
        if s.1 == '_' { continue; }
        field.push((s.0, if s.1 == 'L' { 0 } else { 1 }));
    }

    let mut i = 0;
    for t in target_s.chars().into_iter().enumerate() {
        if t.1 == '_' {
            continue;
        }
        let e = field[i];
        if e.1 == 0 {
            // left
            if t.0 > e.0 {
                return false;
            }
        } else {
            // right
            if t.0 < e.0 {
                return false;
            }
        }

        i += 1;
    }

    true
}
