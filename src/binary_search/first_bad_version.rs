//! 278. First Bad Version
//!
//! # Description
//!
//! You are a product manager and currently leading a team to develop a new
//! product. Unfortunately, the latest version of your product fails the quality
//! check. Since each version is developed based on the previous version, all
//! the versions after a bad version are also bad.
//!
//! Suppose you have n versions [1, 2, ..., n] and you want to find out the
//! first bad one, which causes all the following ones to be bad.
//!
//! You are given an API bool isBadVersion(version) which returns whether
//! version is bad. Implement a function to find the first bad version. You
//! should minimize the number of calls to the API.
//!
//! # Link
//!
//! [278. First Bad Version](https://leetcode.com/problems/first-bad-version/)

use std::cell::Cell;

pub struct FirstBadVersion {
    first_bad_version_idx: i32,
    query_count: Cell<i32>,
}

impl FirstBadVersion {
    pub fn new(first_bad_version_idx: i32) -> Self {
        Self {
            first_bad_version_idx,
            query_count: Cell::new(0),
        }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        // Fast path for the case that there is only one version.
        if n == 1 {
            return 1;
        }

        let mut start = 1;
        let mut end = n;

        while start <= end {
            if start == end {
                if self.is_bad_version(start) {
                    return start;
                } else {
                    return start + 1;
                }
            }

            // To avoid overflow when `n` is very large.
            let offset = (end - start) / 2;
            let mid = start + std::cmp::max(1, offset);
            if self.is_bad_version(mid) {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        return start;
    }

    fn is_bad_version(&self, n: i32) -> bool {
        self.query_count.update(|x| x + 1);
        n >= self.first_bad_version_idx
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let target_index = 4;
        let state = super::FirstBadVersion::new(target_index);
        assert_eq!(state.first_bad_version(5), target_index);
        assert_eq!(state.query_count.get(), 3);
    }

    #[test]
    fn test2() {
        let target_index = 1;
        let state = super::FirstBadVersion::new(target_index);
        assert_eq!(state.first_bad_version(1), target_index);
        assert_eq!(state.query_count.get(), 0);
    }

    #[test]
    fn test3() {
        let target_index = 1;
        let state = super::FirstBadVersion::new(target_index);
        assert_eq!(state.first_bad_version(3), target_index);
        assert_eq!(state.query_count.get(), 2);
    }

    #[test]
    fn test4() {
        let target_index = 3;
        let state = super::FirstBadVersion::new(target_index);
        assert_eq!(state.first_bad_version(3), target_index);
        assert_eq!(state.query_count.get(), 2);
    }

    #[test]
    fn test5() {
        let target_index = 1702766719;
        let state = super::FirstBadVersion::new(target_index);
        assert_eq!(state.first_bad_version(2126753390), target_index);
        assert_eq!(state.query_count.get(), 31);
    }
}
