use std::i32;

pub fn run() {
    for i in [
        vec![8,4,6,2,3],
        vec![1,2,3,4,5],
        vec![10,1,1,6]
    ] {
        println!("{:?}", final_prices(i));
    }
}

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut index = 0;
    let length = prices.len();
    for p in &prices {
        index += 1;
        if index == length {
            result.push(*p);
            break;
        }

        let mut first_lowest = *p;
        for i in index..length {
            if prices[i] < first_lowest {
                first_lowest = prices[i];
                break;
            }
        }
        if first_lowest == *p {
            first_lowest = 0;
        }

        result.push(*p - first_lowest);
    }

    result
}