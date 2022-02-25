//! 704. Binary Search
//!
//! # Description
//!
//! Given an array of integers nums which is sorted in ascending order, and an
//! integer target, write a function to search target in nums. If target exists,
//! then return its index. Otherwise, return -1.
//!
//! You must write an algorithm with O(log n) runtime complexity.
//!
//! # Link
//!
//! [704. Binary Search](https://leetcode.com/problems/binary-search/)

pub struct BinarySearch;

impl BinarySearch {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Fast path for empty array.
        if nums.is_empty() {
            return -1;
        }

        let mut start = 0;
        let mut end = nums.len() - 1;
        while start <= end {
            let middle_idx = (start + end) / 2;
            let middle = nums[middle_idx];
            if middle == target {
                return middle_idx as i32;
            } else if middle > target {
                // In case that `middle_idx` is 0 and it will be underflow.
                if let Some(next_end) = middle_idx.checked_sub(1) {
                    end = next_end;
                } else {
                    return -1;
                }
            } else {
                start = middle_idx + 1;
            }
        }

        // The target is not found.
        return -1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let nums = [-1, 0, 3, 5, 9, 12];
        assert_eq!(super::BinarySearch::search(nums.to_vec(), 9), 4);
    }

    #[test]
    fn test2() {
        let nums = [-1, 0, 3, 5, 9, 12];
        assert_eq!(super::BinarySearch::search(nums.to_vec(), 2), -1);
    }

    #[test]
    fn test3() {
        let nums = [5];
        assert_eq!(super::BinarySearch::search(nums.to_vec(), 5), 0);
    }

    #[test]
    fn test4() {
        let nums = [5];
        assert_eq!(super::BinarySearch::search(nums.to_vec(), -5), -1);
    }
}
