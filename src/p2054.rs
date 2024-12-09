use std::fs;

pub fn run() {
    for i in [
        vec![[1,3,2],[4,5,2],[1,5,5]],
        vec![[2,1000000000,1000000],[1,1,1000000]]
    ].into_iter().map(|x| x.into_iter().map(|y| y.to_vec()).collect::<Vec<Vec<i32>>>()).collect::<Vec<Vec<Vec<i32>>>>() {
        println!("{}", max_two_events(i));
    }

    let r = fs::read("./src/too_damn_long.txt").unwrap();
}

pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
//     # super clean, no need for DP or BST.
// class Solution:
//     def maxTwoEvents(self, events: List[List[int]]) -> int:
//         proc = []
//         ans = m = 0  # m represents max value of finished event so far
//         for s,e,v in events:
//             proc.append( (s, True, v) )     # time, is_start, val
//             proc.append( (e+1, False, v) )  # use e+1 (inclusive)
//         proc.sort()  # sort by time
        
//         for time, is_start, val in proc:
//             if is_start:
//                 ans = max(ans, m+val)
//             else:
//                 m = max(m, val)
//         return ans

    let mut current: Vec<(i32, bool, i32)> = vec![];
    for e in events {
        current.push((e[0], true, e[2]));
        current.push((e[1] + 1, false, e[2]));
    }
    current.sort_by(|a, b| if a.0 == b.0 { b.2.cmp(&a.2) } else { a.0.cmp(&b.0) });

    let mut highest = 0;
    let mut end = 0;
    for e in current {
        if e.1 {
            highest = highest.max(end + e.2);
        } else {
            end = end.max(e.2);
        }
    }

    highest
}
