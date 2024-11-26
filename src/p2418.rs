pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut c = heights.iter().enumerate().map(|(i, x)| (names[i].to_string(), *x)).collect::<Vec<(String, i32)>>();
    c.sort_by(|a, b| a.1.cmp(&b.1));
    c.iter().map(|x| x.0.to_string()).rev().collect()
}

pub fn run() {
    for i in [
        (vec!["Mary","John","Emma"], vec![180,165,170])
    ] {
        println!("{:?}", sort_people(i.0.iter().map(|x| x.to_string()).collect(), i.1));
    }
}
