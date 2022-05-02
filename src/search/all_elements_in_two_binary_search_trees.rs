//! All Elements in Two Binary Search Trees
//!
//! # Link
//!
//! [1305. All Elements in Two Binary Search Trees](https://leetcode.com/problems/all-elements-in-two-binary-search-trees/)

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        TreeNode {
            val,
            left: left.map(|x| Rc::new(RefCell::new(x))),
            right: right.map(|x| Rc::new(RefCell::new(x))),
        }
    }
}

pub struct AllElementsInTwoBinarySearchTrees;

impl AllElementsInTwoBinarySearchTrees {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut result = Vec::new();

        let mut stack1 = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        let mut stack2 = VecDeque::<Rc<RefCell<TreeNode>>>::new();

        fn traverse_push_node(
            stack: &mut VecDeque<Rc<RefCell<TreeNode>>>,
            node: &Option<Rc<RefCell<TreeNode>>>,
        ) {
            if node.is_none() {
                return;
            }

            let node = node.as_ref().unwrap().clone();
            let mut current = Some(node);
            while let Some(_current) = current {
                current = _current.borrow().left.clone();
                stack.push_back(_current);
            }
        }

        fn traverse_pop_and_record_node(
            stack: &mut VecDeque<Rc<RefCell<TreeNode>>>,
            result: &mut Vec<i32>,
        ) {
            let node = stack.pop_back().unwrap();
            result.push(node.borrow().val);
            traverse_push_node(stack, &node.borrow().right);
        }

        traverse_push_node(&mut stack1, &root1);
        traverse_push_node(&mut stack2, &root2);

        loop {
            let stack1_empty = stack1.is_empty();
            let stack2_empty = stack2.is_empty();

            if stack1_empty && stack2_empty {
                break;
            }

            if stack1_empty {
                traverse_pop_and_record_node(&mut stack2, &mut result);
            } else if stack2_empty {
                traverse_pop_and_record_node(&mut stack1, &mut result);
            } else {
                let node1 = stack1.back().unwrap();
                let node2 = stack2.back().unwrap();
                if node1.borrow().val < node2.borrow().val {
                    traverse_pop_and_record_node(&mut stack1, &mut result);
                } else {
                    traverse_pop_and_record_node(&mut stack2, &mut result);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test1() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(
            2,
            Some(TreeNode::new(1, None, None)),
            Some(TreeNode::new(4, None, None)),
        ))));
        let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(
            1,
            Some(TreeNode::new(0, None, None)),
            Some(TreeNode::new(3, None, None)),
        ))));
        let res = super::AllElementsInTwoBinarySearchTrees::get_all_elements(tree1, tree2);
        assert_eq!(res, [0, 1, 1, 2, 3, 4]);
    }

    #[test]
    fn test2() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(
            1,
            None,
            Some(TreeNode::new(8, None, None)),
        ))));
        let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(
            8,
            Some(TreeNode::new(1, None, None)),
            None,
        ))));
        let res = super::AllElementsInTwoBinarySearchTrees::get_all_elements(tree1, tree2);
        assert_eq!(res, [1, 1, 8, 8]);
    }
}
