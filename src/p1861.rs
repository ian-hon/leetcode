pub fn run() {
    for i in [
        vec![
            vec!["#",".","*","."].iter().map(|x| x.chars().collect::<Vec<char>>()[0]).collect::<Vec<char>>(),
            vec!["#","#","*","."].iter().map(|x| x.chars().collect::<Vec<char>>()[0]).collect::<Vec<char>>()
        ],
        vec![
            vec!["#","#","*",".","*","."].iter().map(|x| x.chars().collect::<Vec<char>>()[0]).collect::<Vec<char>>(),
            vec!["#","#","#","*",".","."].iter().map(|x| x.chars().collect::<Vec<char>>()[0]).collect::<Vec<char>>(),
            vec!["#","#","#",".","#","."].iter().map(|x| x.chars().collect::<Vec<char>>()[0]).collect::<Vec<char>>()
        ]
    ] {
        println!("{:?}", rotate_the_box(i));
    }
}

pub fn rotate_the_box(box_: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut collection: Vec<Vec<char>> = vec![];

    for row in box_.clone() {
        let mut bucket: Vec<char> = vec![];
        let mut stepping_hold: Vec<char> = vec![];
        for i in &row {
            if *i == '*' {
                bucket.append(&mut stepping_hold);
                bucket.push('*');
                continue;
            }

            if *i == '#' {
                stepping_hold.push('#');
                continue;
            }

            stepping_hold.insert(0, '.');
        }
        bucket.append(&mut stepping_hold);

        collection.push(bucket);
    }

    let mut c: Vec<Vec<char>> = (0..collection[0].len()).into_iter().map(|_| (0..collection.len()).map(|_| '.').collect::<Vec<char>>()).collect();
    for (row_index, row) in collection.iter().rev().enumerate() {
        for (index, item) in row.iter().enumerate() {
            c[index][row_index] = *item;
        }
    }

    c
}