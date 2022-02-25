//! 35. Search Insert Position
//!
//! # Description
//!
//! Given a sorted array of distinct integers and a target value, return the
//! index if the target is found. If not, return the index where it would be if
//! it were inserted in order.
//!
//! You must write an algorithm with O(log n) runtime complexity.
//!
//! # Link
//!
//! [35. Search Insert Position](https://leetcode.com/problems/search-insert-position/)

pub struct SearchInsertPosition;

impl SearchInsertPosition {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // The solution is pretty similar to "704. Binary Search"'s, the only
        // difference is that when we failed finding the target, return the
        // current search bounds instead of `-1`.
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
                    return 0;
                }
            } else {
                start = middle_idx + 1;
            }
        }

        // The target is not found.
        return start as i32;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let nums = [1, 3, 5, 6];
        assert_eq!(
            super::SearchInsertPosition::search_insert(nums.to_vec(), 5),
            2
        );
    }

    #[test]
    fn test2() {
        let nums = [1, 3, 5, 6];
        assert_eq!(
            super::SearchInsertPosition::search_insert(nums.to_vec(), 2),
            1
        );
    }

    #[test]
    fn test3() {
        let nums = [1, 3, 5, 6];
        assert_eq!(
            super::SearchInsertPosition::search_insert(nums.to_vec(), 7),
            4
        );
    }
}
