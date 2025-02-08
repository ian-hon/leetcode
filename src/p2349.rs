// pub fn run() {
//     for i in [

//     ] {

//     }
// }

use std::collections::{BTreeSet, HashMap};

pub fn run() {
    let mut obj = NumberContainers::new();
    obj.change(10, 24);
    let ret_2: i32 = obj.find(24);

    println!("{ret_2}");
}

// struct NumberContainers {
//     map: HashMap<i32, i32>,
//     indices: BTreeSet<i32>,

//     reverse_index: HashMap<i32, BTreeSet<i32>>
//     // number, lowest index
// }

// impl NumberContainers {
//     fn new() -> Self {
//         NumberContainers {
//             map: HashMap::new(),
//             indices: BTreeSet::new(),
//             reverse_index: HashMap::new()
//         }
//     }
    
//     fn change(&mut self, index: i32, number: i32) {
//         match self.indices.get(&index) {
//             Some(i) => {
//                 let previous = self.map.get_mut(i).unwrap();

//                 self.reverse_index.get_mut(&previous).unwrap().pop_first();
//                 match self.reverse_index.get_mut(&number) {
//                     Some(n) => {
//                         if *n > index {
//                             *n = index;
//                         }
//                     },
//                     None => {
//                         self.reverse_index.insert(number, index);
//                     }
//                 }
                
//                 *previous = number;
//             },
//             None => {
//                 self.map.insert(index, number);
//                 self.indices.insert(index);

//                 match self.reverse_index.get_mut(&number) {
//                     Some(n) => {
//                         if *n > index {
//                             *n = index;
//                         }
//                     },
//                     None => {
//                         self.reverse_index.insert(number, index);
//                     }
//                 }
//             }
//         }
//     }
    
//     fn find(&self, number: i32) -> i32 {
//         match self.reverse_index.get(&number) {
//             Some(n) => {
//                 *n
//             },
//             None => {
//                 -1
//             }
//         }
//     }
// }

struct NumberContainers {
    numbers: HashMap<i32, i32>,
    // number, indices
    map: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            numbers: HashMap::new(),
            map: HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        // match self.map.get(&number) {
        //     Some(n) => {
        //         match self.numbers.get_mut(&index) {
        //             Some(p) => {
        //                 self.map.get_mut(&p).unwrap().remove(&index);
        //                 self.map.get_mut(&number).unwrap().insert(index);

        //                 *p = number;
        //             },
        //             None => {
        //                 self.numbers.insert(index, number);
        //             }
        //         }
        //     },
        //     None => {
        //         match self.numbers.get_mut(&index) {
        //             Some(n) => {
        //                 *n = number;
        //             },
        //             None => {
        //                 self.numbers.insert(index, number);
        //             }
        //         }
        //         self.map.insert(number, BTreeSet::from([index]));
        //     }
        // }

        match self.numbers.get_mut(&index) {
            Some(n) => {
                self.map.get_mut(&n).unwrap().remove(&index);
                // self.map.get_mut(&number).unwrap().insert(index);
                match self.map.get_mut(&number) {
                    Some(m) => {
                        m.insert(index);
                    },
                    None => {
                        self.map.insert(number, BTreeSet::from([index]));
                    }
                }

                *n = number;
            },
            None => {
                self.numbers.insert(index, number);

                match self.map.get_mut(&number) {
                    Some(m) => {
                        // self.map.get_mut(&number).unwrap().remove(&index);
                        m.insert(index);
                    },
                    None => {
                        self.map.insert(number, BTreeSet::from([index]));
                    }
                }
            }
        }
    }
    
    fn find(&self, number: i32) -> i32 {
        match self.map.get(&number) {
            Some(n) => {
                *n.first().unwrap_or(&-1)
            },
            None => {
                -1
            }
        }
    }
}

