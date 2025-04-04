pub fn run() {
    for i in [
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left:Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            right:Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None
                })))
            })))
        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        }))),
    ] {
        println!("{:?}\n\n", lca_deepest_leaves(i));
    }
}

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub fn postorder(map: &mut HashMap<i32, i32>, node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match node {
        None => {
            return None;
        },
        Some(n) => {
            let l_depth = get_depth(n.borrow().left.clone(), map);
            let r_depth = get_depth(n.borrow().right.clone(), map);

            if l_depth > r_depth {
                return postorder(map, n.borrow().left.clone());
            } else if r_depth > l_depth {
                return postorder(map, n.borrow().right.clone());
            } else {
                return Some(n);
            }
        }
    }
}

pub fn get_depth(node: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) -> i32 {
    match node {
        None => {
            return 0;
        },
        Some(n) => {
            let n = n.borrow();
            match map.get(&n.val) {
                Some(v) => {
                    return *v;
                },
                None => {
                    let d = get_depth(n.left.clone(), map).max(get_depth(n.right.clone(), map)) + 1;

                    map.insert(n.val, d);

                    return d;
                }
            }
        }
    }
}

pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut map = HashMap::new();

    postorder(&mut map, root)
}
