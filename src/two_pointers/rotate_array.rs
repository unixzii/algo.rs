//! Rotate Array
//!
//! # Description
//!
//! Given an array, rotate the array to the right by `k` steps, where `k` is
//! non-negative.
//!
//! # Constraints
//!
//! * `1 <= nums.length <= 10^5`
//! * `-2^31 <= nums[i] <= 2^31 - 1`
//! * `0 <= k <= 10^5`
//!
//! # Link
//!
//! [189. Rotate Array](https://leetcode.com/problems/rotate-array/)

pub struct RotateArray;

impl RotateArray {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let nums_len = nums.len();
        let k_idx = (k as usize) % nums_len;

        // Fast path for no rotations or input with
        // only one element.
        if k_idx == 0 || nums_len == 1 {
            return;
        }

        nums.reverse();

        nums[..=(k_idx - 1)].reverse();
        nums[k_idx..].reverse();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let mut nums = [1, 2, 3, 4, 5, 6, 7].to_vec();
        super::RotateArray::rotate(&mut nums, 3);
        assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test2() {
        let mut nums = [-1, -100, 3, 99].to_vec();
        super::RotateArray::rotate(&mut nums, 2);
        assert_eq!(nums, [3, 99, -1, -100]);
    }

    #[test]
    fn test3() {
        let mut nums = [1].to_vec();
        super::RotateArray::rotate(&mut nums, 0);
        assert_eq!(nums, [1]);
    }

    #[test]
    fn test4() {
        let mut nums = [1, 2].to_vec();
        super::RotateArray::rotate(&mut nums, 0);
        assert_eq!(nums, [1, 2]);
    }

    #[test]
    fn test5() {
        let mut nums = [1, 2].to_vec();
        super::RotateArray::rotate(&mut nums, 7);
        assert_eq!(nums, [2, 1]);
    }
}
