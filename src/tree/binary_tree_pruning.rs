//! Binary Tree Pruning
//!
//! # Description
//!
//! Given the `root` of a binary tree, return the same tree where every
//! subtree (of the given tree) not containing a `1` has been removed.
//!
//! A subtree of a node `node` is `node` plus every node that is a
//! descendant of `node`.
//!
//! # Link
//!
//! [814. Binary Tree Pruning](https://leetcode.com/problems/binary-tree-pruning/)

use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

pub struct BinaryTreePruning;

impl BinaryTreePruning {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }

        let root = root.unwrap();
        let mut root_mut = root.borrow_mut();
        root_mut.left = Self::prune_tree(root_mut.left.take());
        root_mut.right = Self::prune_tree(root_mut.right.take());

        if root_mut.left.is_none() && root_mut.right.is_none() {
            if root_mut.val == 0 {
                return None;
            } else {
                drop(root_mut);
                return Some(root);
            }
        }

        drop(root_mut);
        return Some(root);
    }
}
