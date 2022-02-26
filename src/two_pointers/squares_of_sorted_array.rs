//! Squares of a Sorted Array
//!
//! # Description
//!
//! Given an integer array `nums` sorted in **non-decreasing** order, return
//! *an array* of **the squares of each number** sorted in **non-decreasing**
//! order.
//!
//! # Constraints
//!
//! * `1 <= nums.length <= 10^4`
//! * `-10^4 <= nums[i] <= 10^4`
//! * `nums` is sorted in **non-decreasing** order.
//!
//! # Link
//!
//! [977. Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)

pub struct SquaresOfSortedArray;

impl SquaresOfSortedArray {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let mut res = Vec::<i32>::with_capacity(nums_len);

        // Find the index of the element whose absolute value
        // is smallest.
        let min_idx = (|| {
            let mut min_val = i32::MAX;
            for (idx, val) in nums.iter().enumerate() {
                let val = (*val).abs();
                if val > min_val {
                    return idx - 1;
                }
                min_val = val;
            }
            return nums_len - 1;
        })();

        res.push(nums[min_idx].pow(2));

        // Insert other elements by order in a two-pointers fashion.
        let mut left = min_idx as i64 - 1;
        let mut right = min_idx as i64 + 1;

        let right_bound = nums_len as i64;
        while left >= 0 || right < right_bound {
            if left < 0 {
                res.push(nums[right as usize].pow(2));
                right += 1;
            } else if right >= right_bound {
                res.push(nums[left as usize].pow(2));
                left -= 1;
            } else {
                let left_squared = nums[left as usize].pow(2);
                let right_squared = nums[right as usize].pow(2);
                if left_squared < right_squared {
                    res.push(left_squared);
                    left -= 1;
                } else {
                    res.push(right_squared);
                    right += 1;
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let nums = [-4, -1, 0, 3, 10].to_vec();
        assert_eq!(
            super::SquaresOfSortedArray::sorted_squares(nums),
            [0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn test2() {
        let nums = [-7, -3, 2, 3, 11].to_vec();
        assert_eq!(
            super::SquaresOfSortedArray::sorted_squares(nums),
            [4, 9, 9, 49, 121]
        );
    }

    #[test]
    fn test3() {
        let nums = [-5, -4, -3, -2, -1, 0, 1].to_vec();
        assert_eq!(
            super::SquaresOfSortedArray::sorted_squares(nums),
            [0, 1, 1, 4, 9, 16, 25]
        );
    }

    #[test]
    fn test4() {
        let nums = [-1, 0, 1, 1].to_vec();
        assert_eq!(
            super::SquaresOfSortedArray::sorted_squares(nums),
            [0, 1, 1, 1]
        );
    }
}
