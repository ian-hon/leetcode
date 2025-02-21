use std::{cell::RefCell, collections::HashSet, rc::Rc};

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
struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
    map: HashSet<i32>
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut result = HashSet::new();

        match &root {
            Some(r) => {
                r.borrow_mut().val = 0;
            },
            None => {}
        }
        Self::generate(root.clone(), &mut result);

        println!("{result:?}");

        FindElements {
            root,
            map: result
        }
    }

    fn generate(node: Option<Rc<RefCell<TreeNode>>>, map: &mut HashSet<i32>) {
        match node {
            Some(n) => {
                let n = n.borrow();
                map.insert(n.val);

                match &n.left {
                    Some(l) => {
                        l.borrow_mut().val = (2 * n.val) + 1;
                        Self::generate(Some(l.clone()), map);
                    },
                    None => {}
                }
                match &n.right {
                    Some(r) => {
                        r.borrow_mut().val = (2 * n.val) + 2;
                        Self::generate(Some(r.clone()), map);
                    },
                    None => {}
                }
            },
            None => {
                return
            }
        }
    }
    
    fn find(&self, target: i32) -> bool {
        self.map.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
pub fn run() {
    let root = Some(Rc::new(RefCell::new(TreeNode { val: -1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -1,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -1,
                left: None,
                right: None
            })))
        })))
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: -1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: None
        })))
    })));
    let obj = FindElements::new(root);
}