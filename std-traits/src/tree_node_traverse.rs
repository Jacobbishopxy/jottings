//! TreeNode Traverse
//!
//! LeetCode problem

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn iter_left_right_level_order(self) -> IntoIteratorLR {
        TreeNodeByLeftRightLevelOrder(self).into_iter()
    }

    pub fn iter_zigzag_level_order(self) -> IntoIteratorZZ {
        TreeNodeByZigZagLevelOrder(self).into_iter()
    }
}

#[macro_export]
macro_rules! new_node {
    ($num:expr) => {
        Some(Rc::new(RefCell::new(TreeNode::new($num))))
    };
}

// ================================================================================================
// from left to right, level order
// ================================================================================================

// method 1
#[allow(dead_code)]
fn left_right_level_order1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn left_right_level_order_traverse(
        root: Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        res: &mut Vec<Vec<i32>>,
    ) {
        if let Some(rt) = root {
            if depth >= res.len() {
                res.push(vec![]);
            }
            res[depth].push(rt.borrow().val);
            left_right_level_order_traverse(rt.borrow().left.clone(), depth + 1, res);
            left_right_level_order_traverse(rt.borrow().right.clone(), depth + 1, res);
        }
    }

    let mut res = vec![];

    left_right_level_order_traverse(root, 0, &mut res);

    res
}

// method 2
#[allow(dead_code)]
fn left_right_level_order2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if root.is_none() {
        return res;
    }

    let mut que = VecDeque::new();
    que.push_back(root.unwrap());

    while !que.is_empty() {
        let mut level = vec![];
        for _ in 0..que.len() {
            let node = que.pop_front().unwrap();
            level.push(node.borrow().val);
            if node.borrow().left.is_some() {
                que.push_back(node.borrow().left.clone().unwrap());
            }
            if node.borrow().right.is_some() {
                que.push_back(node.borrow().right.clone().unwrap());
            }
        }
        res.push(level);
    }

    res
}

pub struct TreeNodeByLeftRightLevelOrder(TreeNode);

pub struct IntoIteratorLR {
    que: VecDeque<Rc<RefCell<TreeNode>>>,
}

impl Iterator for IntoIteratorLR {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.que.is_empty() {
            None
        } else {
            let node = self.que.pop_front().unwrap();
            if node.borrow().left.is_some() {
                self.que.push_back(node.borrow().left.clone().unwrap());
            }
            if node.borrow().right.is_some() {
                self.que.push_back(node.borrow().right.clone().unwrap());
            }

            return Some(node.borrow().val);
        }
    }
}

impl IntoIterator for TreeNodeByLeftRightLevelOrder {
    type Item = i32;
    type IntoIter = IntoIteratorLR;

    fn into_iter(self) -> Self::IntoIter {
        let mut que = VecDeque::new();
        que.push_back(Rc::new(RefCell::new(self.0)));

        IntoIteratorLR { que }
    }
}

#[test]
fn level_order_success() {
    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: new_node!(4),
            right: new_node!(3),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: new_node!(3),
            right: new_node!(4),
        }))),
    };

    println!(
        "{:?}",
        left_right_level_order1(Some(Rc::new(RefCell::new(root.clone()))))
    );

    println!(
        "{:?}",
        left_right_level_order2(Some(Rc::new(RefCell::new(root.clone()))))
    );

    for e in root.iter_left_right_level_order() {
        println!("{:?}", e);
    }
}

// ================================================================================================
// zigzag type, level order
// ================================================================================================

// method 1
#[allow(dead_code)]
fn zigzag_level_order1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn zigzag_level_order_traverse(
        root: Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        res: &mut Vec<Vec<i32>>,
    ) {
        if let Some(rt) = root {
            if depth >= res.len() {
                res.push(Vec::new());
            }

            if depth % 2 == 0 {
                res[depth].push(rt.borrow().val);
            } else {
                res[depth].insert(0, rt.borrow().val);
            }

            zigzag_level_order_traverse(rt.borrow().right.clone(), depth + 1, res);
            zigzag_level_order_traverse(rt.borrow().left.clone(), depth + 1, res);
        }
    }

    let mut res = vec![];

    zigzag_level_order_traverse(root, 0, &mut res);

    res
}

// method 2
#[allow(dead_code)]
fn zigzag_level_order2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if root.is_none() {
        return res;
    }

    let mut que = VecDeque::new();
    que.push_back(root.unwrap());
    let mut is_order_left = true;

    while !que.is_empty() {
        let mut level = VecDeque::new();
        for _ in 0..que.len() {
            let node = que.pop_front().unwrap();
            if is_order_left {
                level.push_back(node.borrow().val);
            } else {
                level.push_front(node.borrow().val)
            }
            if node.borrow().right.is_some() {
                que.push_back(node.borrow().right.clone().unwrap());
            }
            if node.borrow().left.is_some() {
                que.push_back(node.borrow().left.clone().unwrap());
            }
        }
        res.push(Vec::from(level));
        is_order_left = !is_order_left;
    }

    res
}

pub struct TreeNodeByZigZagLevelOrder(TreeNode);

pub struct IntoIteratorZZ {
    que: VecDeque<Rc<RefCell<TreeNode>>>,
    is_order_left: bool,
    len: usize,
    stp: usize,
    flg: bool,
}

impl Iterator for IntoIteratorZZ {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // dbg!(&self.que);
        if self.que.is_empty() {
            None
        } else {
            self.stp += 1;

            if self.stp > self.len {
                self.is_order_left = !self.is_order_left;
                self.stp = 0;
                self.flg = true;
            }

            let node;

            if self.is_order_left {
                node = self.que.pop_front().unwrap();

                if let Some(n) = &node.borrow().left {
                    self.que.push_back(n.clone());
                    if self.flg {
                        self.len += 1;
                    }
                }
                if let Some(n) = &node.borrow().right {
                    self.que.push_back(n.clone());
                    if self.flg {
                        self.len += 1;
                    }
                }
            } else {
                node = self.que.pop_back().unwrap();

                if let Some(n) = &node.borrow().left {
                    self.que.push_front(n.clone());
                    if self.flg {
                        self.len += 1;
                    }
                }
                if let Some(n) = &node.borrow().right {
                    self.que.push_front(n.clone());
                    if self.flg {
                        self.len += 1;
                    }
                }
            };

            self.flg = false;
            return Some(node.borrow().val);
        }
    }
}

impl IntoIterator for TreeNodeByZigZagLevelOrder {
    type Item = i32;
    type IntoIter = IntoIteratorZZ;

    fn into_iter(self) -> Self::IntoIter {
        let mut que = VecDeque::new();
        que.push_back(Rc::new(RefCell::new(self.0)));
        let is_order_left = true;

        IntoIteratorZZ {
            que,
            is_order_left,
            len: 0,
            stp: 0,
            flg: false,
        }
    }
}

#[test]
fn zigzag_level_order_success() {
    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: new_node!(3),
            right: new_node!(4),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: new_node!(6),
            right: new_node!(7),
        }))),
    };

    println!(
        "{:?}",
        zigzag_level_order1(Some(Rc::new(RefCell::new(root.clone()))))
    );

    println!(
        "{:?}",
        zigzag_level_order2(Some(Rc::new(RefCell::new(root.clone()))))
    );

    let mut v = vec![];
    for e in root.iter_zigzag_level_order() {
        println!("{:?}", e);
        v.push(e);
    }

    println!("{:?}", v);
}
