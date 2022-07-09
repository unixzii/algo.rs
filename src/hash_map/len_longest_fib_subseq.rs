//! Length of Longest Fibonacci Subsequence
//!
//! # Link
//!
//! [873. Length of Longest Fibonacci Subsequence](https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/)

use std::collections::HashMap;

pub struct LengthOfLongestFibonacciSubsequence;

struct Context<'a> {
    elem_pos: &'a HashMap<i32, usize>,
    arr: &'a Vec<i32>,
    memorized: HashMap<(usize, usize), i32>,
}

impl LengthOfLongestFibonacciSubsequence {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let elem_pos: HashMap<i32, usize> = arr
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, elem)| (elem, idx))
            .collect();

        let mut cx = Context {
            elem_pos: &elem_pos,
            arr: &arr,
            memorized: HashMap::new(),
        };

        let mut max_len = 0;
        for i_pos in 0..=(arr.len() - 2) {
            for j_pos in (i_pos + 1)..=(arr.len() - 1) {
                max_len = max_len.max(Self::search(i_pos, j_pos, 2, &mut cx));
            }
        }

        let res = if max_len < 3 { 0 } else { max_len };

        #[cfg(debug_assertions)]
        println!("{:#?} {}", arr, res);

        res
    }

    fn search(i_pos: usize, j_pos: usize, len: i32, cx: &mut Context) -> i32 {
        if let Some(memorized_val) = cx.memorized.get(&(i_pos, j_pos)) {
            #[cfg(debug_assertions)]
            println!("({} {}) hit cache", i_pos, j_pos);
            return memorized_val + len;
        }

        let arr = cx.arr;
        let next = arr[i_pos] + arr[j_pos];
        if let Some(next_pos) = cx.elem_pos.get(&next) {
            let res = Self::search(j_pos, *next_pos, len + 1, cx);
            cx.memorized.insert((i_pos, j_pos), res - len);
            res
        } else {
            len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LengthOfLongestFibonacciSubsequence;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            LengthOfLongestFibonacciSubsequence::len_longest_fib_subseq(nums),
            5
        )
    }

    #[test]
    fn test2() {
        let nums = vec![1, 3, 7, 11, 12, 14, 18];
        assert_eq!(
            LengthOfLongestFibonacciSubsequence::len_longest_fib_subseq(nums),
            3
        )
    }

    #[test]
    fn test3() {
        let nums = vec![2, 4, 5, 6, 7, 8, 11, 13, 14, 15, 21, 22, 34];
        assert_eq!(
            LengthOfLongestFibonacciSubsequence::len_longest_fib_subseq(nums),
            5
        )
    }
}
