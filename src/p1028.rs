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

pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    let data_map = split(traversal);

    let mut buckets: HashMap<usize, Rc<RefCell<TreeNode>>> = HashMap::new();

    let mut previous_depth = 0;
    for (depth, value) in data_map {
        if depth == 0 {
            buckets.insert(0, Rc::new(RefCell::new(TreeNode::new(value))));
            continue;
        }

        if depth < previous_depth {

            let highest_index = *buckets.iter().max_by_key(|x| x.0).unwrap().0;

            for i in depth..highest_index {
                let i = highest_index - i + depth;

                let current = buckets.get(&i).unwrap().clone();
                let parent = buckets.get(&(i - 1)).unwrap().clone();

                if parent.borrow().right.is_none() {
                    parent.borrow_mut().left = Some(current.clone());
                } else {
                    parent.borrow_mut().right = Some(current.clone());
                }

                buckets.remove(&i);
            }
        }

        buckets.insert(depth, Rc::new(RefCell::new(TreeNode::new(value))).clone());

        let parent = buckets.get(&(depth - 1)).unwrap().clone();
        if parent.borrow().left.is_none() {
            parent.borrow_mut().left = Some(buckets.get(&depth).unwrap().clone());
        } else {
            parent.borrow_mut().right = Some(buckets.get(&depth).unwrap().clone());
        }

        previous_depth = depth;
    }

    Some(buckets.get(&0).unwrap().clone())
}

pub fn display_map(m: &HashMap<usize, Rc<RefCell<TreeNode>>>) {
    let mut m = m.clone().into_iter().collect::<Vec<(usize, Rc<RefCell<TreeNode>>)>>();
    m.sort_by_key(|x| x.0);
    for (k, v) in m {
        println!("{k} : {v:?}");
    }
}

pub fn split(s: String) -> Vec<(usize, i32)> {
    let mut chars = s.chars();

    let mut result = vec![];
    let mut current = (0, "".to_string());

    let mut previous = false;

    while let Some(c) = chars.next() {
        if c == '-' {
            if previous {
                result.push((current.0, current.1.parse::<i32>().unwrap()));
                current = (0, "".to_string());
            }
            previous = false;
            current.0 += 1;
        } else {
            previous = true;
            current.1 += c.to_string().as_str();
        }
    }

    result.push((current.0, current.1.parse::<i32>().unwrap()));

    result
}

pub fn run() {
    for i in [
        "1-2--3--4-5--6--7".to_string(),
        "1-2--3---4-5--6---7".to_string(),
        "1-401--349---90--88".to_string(),
    ] {
        println!("{:?}", recover_from_preorder(i));
    }
}
