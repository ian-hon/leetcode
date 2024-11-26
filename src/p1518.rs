pub fn run() {
    for i in [
        (9, 3),
        (15, 4)
    ] {
        println!("{}", num_water_bottles(i.0, i.1));
    }
}

pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut filled = num_bottles;
    let mut empty = 0;
    let mut drank = 0;

    loop {
        empty += filled;
        drank += filled;
        filled = 0;

        let exchanged_times = empty / num_exchange;
        if exchanged_times == 0 {
            return filled + drank;
        }
        empty -= exchanged_times * num_exchange;
        filled += exchanged_times;
    }
}
