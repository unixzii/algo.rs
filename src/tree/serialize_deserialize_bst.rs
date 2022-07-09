//! Serialize and Deserialize BST
//!
//! Serialization is converting a data structure or object into a sequence of
//! bits so that it can be stored in a file or memory buffer, or transmitted
//! across a network connection link to be reconstructed later in the same or
//! another computer environment.
//!
//! Design an algorithm to serialize and deserialize a binary search tree.
//! There is no restriction on how your serialization/deserialization
//! algorithm should work. You need to ensure that a binary search tree can
//! be serialized to a string, and this string can be deserialized to the
//! original tree structure.
//!
//! The encoded string should be as compact as possible.
//!
//! # Constraints
//!
//! * The number of nodes in the tree is in the range `[0, 10^4]`.
//! * `0 <= Node.val <= 10^4`
//! * The input tree is **guaranteed** to be a binary search tree.
//!
//! # Link
//!
//! [449. Serialize and Deserialize BST](https://leetcode.com/problems/serialize-and-deserialize-bst/)

use std::cell::RefCell;
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
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

const EMPTY_TOKEN: &str = "x";
const DELIM_TOKEN: &str = " ";

pub struct Codec;

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();
        Self::serialize_node(&root, &mut result);
        result
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut elems = data.split(DELIM_TOKEN);
        Self::deserialize_node_from(&mut elems).map(|n| Rc::new(RefCell::new(n)))
    }

    fn serialize_node(root: &Option<Rc<RefCell<TreeNode>>>, data: &mut String) {
        let elem = match root.as_ref() {
            Some(node) => {
                let node_val = node.as_ref().borrow().val;
                node_val.to_string()
            }
            None => EMPTY_TOKEN.to_owned(),
        };
        data.push_str(&elem);
        data.push_str(DELIM_TOKEN);

        if let Some(node) = root.as_ref() {
            let node_ref = node.as_ref().borrow();
            Self::serialize_node(&node_ref.left, data);
            Self::serialize_node(&node_ref.right, data);
        }
    }

    fn deserialize_node_from<'a, I>(iter: &mut I) -> Option<TreeNode>
    where
        I: Iterator<Item = &'a str>,
    {
        let elem = iter.next().unwrap(); // Let it crash if failed.

        if elem == EMPTY_TOKEN {
            None
        } else {
            let val: i32 = elem.parse().unwrap();
            let mut node = TreeNode::new(val);
            node.left = Self::deserialize_node_from(iter).map(|n| Rc::new(RefCell::new(n)));
            node.right = Self::deserialize_node_from(iter).map(|n| Rc::new(RefCell::new(n)));
            Some(node)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::{Codec, TreeNode};

    #[test]
    fn test1() {
        let mut tree = TreeNode::new(2);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        tree.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        let codec = Codec::new();
        let data = codec.serialize(Some(Rc::new(RefCell::new(tree))));
        println!("{}", data);

        let ans = codec.deserialize(data).unwrap();

        let tree = ans.as_ref().borrow();
        assert_eq!(tree.val, 2);
        assert_eq!(tree.left.as_ref().unwrap().as_ref().borrow().val, 1);
        assert_eq!(tree.right.as_ref().unwrap().as_ref().borrow().val, 3);
    }

    #[test]
    fn test2() {
        let codec = Codec::new();
        let data = codec.serialize(None);
        println!("{}", data);

        let ans = codec.deserialize(data);
        assert_eq!(ans, None);
    }
}
