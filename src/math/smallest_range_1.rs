//! Smallest Range I
//!
//! # Description
//!
//! You are given an integer array nums and an integer k.
//!
//! In one operation, you can choose any index i where `0 <= i < nums.length`
//! and change `nums[i]` to `nums[i] + x` where x is an integer from the range
//! `[-k, k]`. You can apply this operation **at most once** for each index i.
//!
//! The score of nums is the difference between the maximum and minimum
//! elements in nums.
//!
//! Return the minimum score of nums after applying the mentioned operation
//! at most once for each index in it.
//!
//! # Constraints
//!
//! * `1 <= nums.length <= 10^4`
//! * `0 <= nums[i] <= 10^4`
//! * `0 <= k <= 10^4`
//!
//! # Link
//!
//! [908. Smallest Range I](https://leetcode.com/problems/smallest-range-i/)

pub struct SmallestRange1;

impl SmallestRange1 {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        const MAX_VALUE: i32 = 10_000;
        let (min_val, max_val) = nums.iter().fold((MAX_VALUE, 0), |acc, x| {
            let (min, max) = acc;
            let val = *x;
            (min.min(val), max.max(val))
        });
        (max_val - min_val - 2 * k).max(0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let nums = vec![1];
        assert_eq!(super::SmallestRange1::smallest_range_i(nums, 0), 0);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 10];
        assert_eq!(super::SmallestRange1::smallest_range_i(nums, 2), 6);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 3, 6];
        assert_eq!(super::SmallestRange1::smallest_range_i(nums, 3), 0);
    }
}
