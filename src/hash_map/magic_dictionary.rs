//! Implement Magic Dictionary
//!
//! # Description
//!
//! Design a data structure that is initialized with a list of **different** words.
//! Provided a string, you should determine if you can change exactly one character
//! in this string to match any word in the data structure.
//!
//! Implement the `MagicDictionary` class:
//!
//! `MagicDictionary()` Initializes the object.
//! `void buildDict(String[] dictionary)` Sets the data structure with an array of
//! distinct strings `dictionary`.
//! `bool search(String searchWord)` Returns `true` if you can change exactly one
//! character in `searchWord` to match any string in the data structure, otherwise
//! returns `false`.
//!
//! # Link
//!
//! [676. Implement Magic Dictionary](https://leetcode.com/problems/implement-magic-dictionary/)

use std::collections::{HashMap, LinkedList};

pub struct MagicDictionary {
    shards: HashMap<usize, LinkedList<String>>,
}

impl MagicDictionary {
    fn new() -> Self {
        Self {
            shards: HashMap::new(),
        }
    }

    fn build_dict(&mut self, dict: Vec<String>) {
        let mut dict_mut = dict;
        while let Some(word) = dict_mut.pop() {
            self.shards
                .entry(word.len())
                .or_insert(LinkedList::new())
                .push_back(word);
        }
    }

    fn search(&self, search_word: String) -> bool {
        let maybe_shard = self.shards.get(&search_word.len());
        if maybe_shard.is_none() {
            return false;
        }

        // Cache chars to a vector to accelerate the accessing of Unicode scalars.
        let search_word_chars_cache: Vec<_> = search_word.chars().collect();
        maybe_shard
            .unwrap()
            .iter()
            .position(|candidate| {
                let diff = candidate
                    .chars()
                    .zip(search_word_chars_cache.iter())
                    .fold(0, |acc, (lhs, rhs)| acc + if lhs != *rhs { 1 } else { 0 });
                diff == 1
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::MagicDictionary;
    #[test]
    fn test() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
        assert_eq!(false, dict.search("hello".to_string()));
        assert_eq!(true, dict.search("hhllo".to_string()));
        assert_eq!(false, dict.search("hell".to_string()));
        assert_eq!(false, dict.search("leetcoded".to_string()));
    }
}
