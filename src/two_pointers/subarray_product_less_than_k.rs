//! Subarray Product Less Than K
//!
//! # Description
//!
//! Given an array of integers nums and an integer k, return the number
//! of contiguous subarrays where the product of all the elements in the
//! subarray is strictly less than k.
//!
//! # Constraints
//!
//! * `1 <= nums.length <= 3 * 10^4`
//! * `1 <= nums[i] <= 1000`
//! * `0 <= k <= 10^6`
//!
//! # Link
//!
//! [713. Subarray Product Less Than K](https://leetcode.com/problems/subarray-product-less-than-k/)

pub struct SubarrayProductLessThanK;

impl SubarrayProductLessThanK {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k < 2 {
            return 0;
        }

        let mut left_idx = 0;
        let mut right_idx = 0;
        let mut count: i32 = 0;
        let mut acc = 1;

        let nums_len = nums.len();
        while right_idx < nums_len {
            acc *= nums[right_idx];
            while acc >= k {
                acc /= nums[left_idx];
                left_idx += 1;
            }

            right_idx += 1;
            count += (right_idx - left_idx) as i32;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let nums = [10, 5, 2, 6].to_vec();
        let res = super::SubarrayProductLessThanK::num_subarray_product_less_than_k(nums, 100);
        assert_eq!(res, 8);
    }

    #[test]
    fn test2() {
        let nums = [1, 2, 3].to_vec();
        let res = super::SubarrayProductLessThanK::num_subarray_product_less_than_k(nums, 0);
        assert_eq!(res, 0);
    }

    #[test]
    fn test3() {
        let nums = [10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3].to_vec();
        let res = super::SubarrayProductLessThanK::num_subarray_product_less_than_k(nums, 19);
        assert_eq!(res, 18);
    }
}
