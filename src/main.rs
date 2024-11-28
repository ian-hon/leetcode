use std::time::Instant;
mod p2290;

pub fn main() {
    p2290::run();

    // // let mut a: Vec<(i32, i32)> = vec![];
    // // for _ in 0..1_000_000 {
    // //     a.push((10, 10));
    // // }
    // let mut a: Vec<[i32; 2]> = vec![];
    // for _ in 0..1_000_000 {
    //     a.push([10, 10]);
    // }
    // let mut now = Instant::now();
    // // for i in a { // 5.47ms
    // //     let b = i.0;
    // //     let c = i.1;
    // // }
    // for i in a {
    //     let b = i[0];
    //     let c = i[1];
    // }
    // println!("{:.2?}", now.elapsed());
}

// pub fn main() {
//     let n = 10000;
//     let mut now = Instant::now();
//     let a = (0..n).map(|y| (0..n).map(|x| (x, y)).collect::<Vec<(i32, i32)>>()).collect::<Vec<Vec<(i32, i32)>>>().into_iter().flatten().collect::<Vec<(i32, i32)>>();
//     println!("{:.2?}", now.elapsed());

//     now = Instant::now();
//     let mut v: Vec<(i32, i32)> = vec![];
//     for y in 0..n {
//         for x in 0..n {
//             v.push((x, y));
//         }
//     }
//     println!("{:.2?}", now.elapsed());
// }