use std::rc::Rc;
use std::cell::RefCell;

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

pub fn run() {
    for i in [
        (vec![1,2,4,5,3,6,7], vec![4,5,2,6,7,3,1])
    ] {
        println!("{:?}", construct_from_pre_post(i.0, i.1));
    }
}

pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut iter = preorder.into_iter();

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![
        Rc::new(RefCell::new(TreeNode::new(iter.next().unwrap())))
    ];

    let mut post_index = 0;

    while let Some(value) = iter.next() {
        let node = Rc::new(RefCell::new(TreeNode::new(value)));

        while stack.last().unwrap().borrow().val == postorder[post_index] {
            stack.pop();
            post_index += 1;
        }

        let previous = stack.last().unwrap();
        if previous.borrow().left.is_none() {
            previous.borrow_mut().left = Some(node.clone());
        } else {
            previous.borrow_mut().right = Some(node.clone());
        }

        stack.push(node.clone());
    }

    Some(stack[0].clone())
}
