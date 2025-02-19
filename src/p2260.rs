use std::{collections::HashMap, fs};

pub fn run() {
    for i in [
        vec![3,4,2,3,4,7],
        vec![1,0,5,3],
        serde_json::from_str::<Vec<i32>>(&fs::read_to_string("./src/too_damn_long.txt").unwrap()).unwrap()
    ] {
        println!("{}", minimum_card_pickup(i))
    }
}

pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut result = std::i32::MAX;
    
    for (index, c) in cards.iter().enumerate() {
        match map.get_mut(c) {
            Some(i) => {
                result = result.min((index - *i) as i32 + 1);

                *i = index;
            },
            None => {
                map.insert(*c, index);
            }
        }
    }

    if result == std::i32::MAX {
        return -1;
    }

    result
}