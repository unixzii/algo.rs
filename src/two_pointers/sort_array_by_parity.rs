//! Sort Array By Parity
//!
//! # Description
//!
//! Given an integer array `nums`, move all the even integers at the beginning
//! of the array followed by all the odd integers.
//!
//! Return **any array** that satisfies this condition.
//!
//! # Constraints
//!
//! * `1 <= nums.length <= 5000`
//! * `0 <= nums[i] <= 5000`
//!
//! # Link
//!
//! [905. Sort Array By Parity](https://leetcode.com/problems/sort-array-by-parity/)

pub struct SortArrayByParity;

impl SortArrayByParity {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        // Fast path for empty or single element input.
        if nums.len() <= 1 {
            return nums;
        }

        let mut nums_mut = nums;
        let mut forward_idx = 0;
        let mut backward_idx = nums_mut.len() - 1;
        while forward_idx < backward_idx {
            let forward_val = nums_mut[forward_idx];
            let backward_val = nums_mut[backward_idx];
            if forward_val % 2 == 1 && backward_val % 2 == 0 {
                nums_mut[forward_idx] = backward_val;
                nums_mut[backward_idx] = forward_val;
                forward_idx += 1;
                backward_idx -= 1;
                continue;
            }

            if forward_val % 2 == 0 {
                forward_idx += 1;
            }

            if backward_val % 2 == 1 {
                backward_idx -= 1;
            }
        }

        nums_mut
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let nums = vec![3, 1, 2, 4];
        assert_eq!(
            super::SortArrayByParity::sort_array_by_parity(nums),
            vec![4, 2, 1, 3]
        );
    }

    #[test]
    fn test2() {
        let nums = vec![0];
        assert_eq!(
            super::SortArrayByParity::sort_array_by_parity(nums),
            vec![0]
        );
    }
}
