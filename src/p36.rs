use std::collections::HashMap;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    {
        let mut field_horizontal: HashMap<usize, HashMap<i32, i32>> = (0..9).map(|n| (n as usize, (1..=9).map(|m| (m, 0)).collect::<HashMap<i32, i32>>())).collect::<HashMap<usize, HashMap<i32, i32>>>();
        let mut field_vertical: HashMap<usize, HashMap<i32, i32>> = (0..9).map(|n| (n as usize, (1..=9).map(|m| (m, 0)).collect::<HashMap<i32, i32>>())).collect::<HashMap<usize, HashMap<i32, i32>>>();

        for (y, column) in board.iter().enumerate() {
            for (x, i) in column.iter().enumerate() {
                let i = *i;
                if i == '.' {
                    continue;
                }
                let i = i.to_string().parse::<i32>().unwrap();

                let h = field_horizontal.get_mut(&x).unwrap().get_mut(&i).unwrap();
                if *h >= 1 {
                    return false;
                }

                let v = field_vertical.get_mut(&y).unwrap().get_mut(&i).unwrap();
                if *v >= 1 {
                    return false;
                }

                *h += 1;
                *v += 1;
            }
        }
    }

    {
        let mut groups: Vec<Vec<char>> = (0..9).map(|_| vec![]).collect();

        for (y, row) in board.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if *item == '.' {
                    continue;
                }

                let group_index = ((y / 3) * 3) + (x / 3);

                if groups[group_index].contains(item) {
                    return false;
                }

                groups[group_index].push(item.clone());
            }
        }
    }

    true
}

pub fn run() {
    for i in [
        [['5','3','.','.','7','.','.','.','.'],['6','.','.','1','9','5','.','.','.'],['.','9','8','.','.','.','.','6','.'],['8','.','.','.','6','.','.','.','3'],['4','.','.','8','.','3','.','.','1'],['7','.','.','.','2','.','.','.','6'],['.','6','.','.','.','.','2','8','.'],['.','.','.','4','1','9','.','.','5'],['.','.','.','.','8','.','.','7','9']],
        [['8','3','.','.','7','.','.','.','.'],['6','.','.','1','9','5','.','.','.'],['.','9','8','.','.','.','.','6','.'],['8','.','.','.','6','.','.','.','3'],['4','.','.','8','.','3','.','.','1'],['7','.','.','.','2','.','.','.','6'],['.','6','.','.','.','.','2','8','.'],['.','.','.','4','1','9','.','.','5'],['.','.','.','.','8','.','.','7','9']],
        [   ['.','.','.','.','5','.','.','1','.'],
            ['.','4','.','3','.','.','.','.','.'],
            ['.','.','.','.','.','3','.','.','1'],
            ['8','.','.','.','.','.','.','2','.'],
            ['.','.','2','.','7','.','.','.','.'],
            ['.','1','5','.','.','.','.','.','.'],
            ['.','.','.','.','.','2','.','.','.'],
            ['.','2','.','9','.','.','.','.','.'],
            ['.','.','4','.','.','.','.','.','.']]
    ] {
        println!("{}", is_valid_sudoku(i.to_vec().iter().map(|x| x.to_vec()).collect::<Vec<Vec<char>>>()));
    }
}
