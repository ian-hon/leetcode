pub fn run() {
    for i in [
        3
    ] {
        println!("{:?}", generate_parenthesis(i));
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let count = n;
    let mut result = vec![];

    generate("(".to_string(), count - 1, 1, &mut result);

    result
}

fn generate(current: String, open: i32, close: i32, result: &mut Vec<String>) {
    if (open == 0) && (close == 0) {
        result.push(current);
        return;
    }

    let c = current.clone();
    if open > 0 {
        generate(c.clone() + "(", open - 1, close + 1, result);
    }
    if close > 0 {
        generate(c + ")", open, close - 1, result);
    }
}
