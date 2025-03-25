pub fn run() {
    for i in [
        (5, vec![vec![1,0,5,2],vec![0,2,2,4],vec![3,2,5,3],vec![0,4,4,5]]),
        (4, vec![vec![0,0,1,1],vec![2,0,3,4],vec![0,2,2,3],vec![3,0,4,3]]),
        (4, vec![vec![0,2,2,4],vec![1,0,3,2],vec![2,2,3,4],vec![3,0,4,2],vec![3,2,4,4]])
    ] {
        println!("{}", check_valid_cuts(i.0, i.1));
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Event {
    End,
    Start
}

pub fn get_segments(events: &Vec<(i32, Event)>) -> i32 {
    let mut result = 0;

    let mut previous = events[0];
    let mut running = 0;
    for event in events {
        if previous.0 == event.0 {
            if previous.1 != event.1 {
                if running == 0 {
                    result += 1;
                }
            }
        }

        running += match event.1 {
            Event::Start => {
                1
            },
            Event::End => {
                -1
            }
        };

        previous = event.clone();
    }

    result
}

pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
    let mut horizontal_events = vec![];
    let mut vertical_events = vec![];

    for i in rectangles {
        horizontal_events.push((i[0], Event::Start));
        horizontal_events.push((i[2], Event::End));

        vertical_events.push((i[1], Event::Start));
        vertical_events.push((i[3], Event::End));
    }

    horizontal_events.sort();
    vertical_events.sort();

    (get_segments(&horizontal_events) >= 2) || (get_segments(&vertical_events) >= 2)
}