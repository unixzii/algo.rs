//! Array Nesting
//!
//! # Description
//!
//! You are given an integer array `nums` of length `n` where `nums` is a permutation of the
//! numbers in the range `[0, n - 1]`.
//!
//! You should build a set `s[k] = {nums[k], nums[nums[k]], nums[nums[nums[k]]], ... }` subjected
//! to the following rule:
//!
//! * The first element in `s[k]` starts with the selection of the element `nums[k]` of `index = k`.
//! * The next element in `s[k]` should be `nums[nums[k]]`, and then `nums[nums[nums[k]]]`, and so on.
//! * We stop adding right before a duplicate element occurs in `s[k]`.
//!
//! Return the longest length of a set `s[k]`.
//!
//! # Link
//!
//! [565. Array Nesting](https://leetcode.com/problems/array-nesting/)

use std::collections::{HashMap, HashSet};

pub struct ArrayNesting;

impl ArrayNesting {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut cache = HashMap::with_capacity(nums.len());

        nums.iter()
            .map(|num| Self::search(&nums, *num as usize, &mut cache))
            .max()
            .unwrap_or(0) as i32
    }

    fn search(nums: &Vec<i32>, from_pos: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if let Some(&ring_len) = cache.get(&from_pos) {
            return ring_len;
        }

        let mut ring_members = HashSet::new();
        let mut pos = from_pos;
        loop {
            let num = nums[pos];
            if ring_members.contains(&num) {
                break;
            }
            ring_members.insert(num);
            pos = num as usize;
        }

        let ring_len = ring_members.len();
        for num in ring_members {
            cache.insert(num as usize, ring_len);
        }

        ring_len
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(
            super::ArrayNesting::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]),
            4
        )
    }

    #[test]
    fn test2() {
        assert_eq!(super::ArrayNesting::array_nesting(vec![0, 1, 2]), 1)
    }
}
