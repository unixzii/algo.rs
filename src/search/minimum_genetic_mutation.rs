//! Minimum Genetic Mutation
//!
//! # Link
//!
//! [433. Minimum Genetic Mutation](https://leetcode.com/problems/minimum-genetic-mutation/)

use std::collections::HashSet;
use std::collections::VecDeque;

const GENE_ELEMENTS: [&str; 4] = ["A", "C", "G", "T"];

pub struct MinimumGeneticMutation;

impl MinimumGeneticMutation {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        if start == end {
            return 0;
        }

        let mut opened_paths: HashSet<String> = HashSet::from_iter(bank);
        let mut bfs_queue = VecDeque::<String>::new();

        bfs_queue.push_back(start);

        let mut mutations = 0;
        while !bfs_queue.is_empty() {
            let next_paths_num = bfs_queue.len();
            for _ in 0..next_paths_num {
                let cur = bfs_queue.pop_front().unwrap();
                if cur == end {
                    return mutations;
                }

                // The gene has been visited, remove it from the opened paths.
                opened_paths.remove(&cur);

                // Visit its all mutations.
                Self::mutate_and_visit(cur, &opened_paths, &mut bfs_queue);
            }

            if bfs_queue.is_empty() {
                // Destination not reachable, bailout.
                return -1;
            } else {
                // Current generation can continue to advance.
                mutations += 1;
            }
        }

        assert_eq!(mutations, 0); // Just, for invariant.
        -1
    }

    fn mutate_and_visit(
        element: String,
        opened_paths: &HashSet<String>,
        bfs_queue: &mut VecDeque<String>,
    ) {
        for mutation_point in element.chars().enumerate() {
            let pos = mutation_point.0;
            let elem = mutation_point.1;

            for target_elem in GENE_ELEMENTS {
                if target_elem.chars().nth(0).unwrap() == elem {
                    continue;
                }
                let mut mutated_element = element.clone();
                mutated_element.replace_range(pos..(pos + 1), target_elem);
                if opened_paths.contains(&mutated_element) {
                    bfs_queue.push_back(mutated_element);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let res = super::MinimumGeneticMutation::min_mutation(
            "AACCGGTT".to_owned(),
            "AACCGGTA".to_owned(),
            vec!["AACCGGTA".to_owned()],
        );
        assert_eq!(res, 1);
    }

    #[test]
    fn test2() {
        let res = super::MinimumGeneticMutation::min_mutation(
            "AACCGGTT".to_owned(),
            "AAACGGTA".to_owned(),
            vec![
                "AACCGGTA".to_owned(),
                "AACCGCTA".to_owned(),
                "AAACGGTA".to_owned(),
            ],
        );
        assert_eq!(res, 2);
    }

    #[test]
    fn test3() {
        let res = super::MinimumGeneticMutation::min_mutation(
            "AAAAACCC".to_owned(),
            "AACCCCCC".to_owned(),
            vec![
                "AAAACCCC".to_owned(),
                "AAACCCCC".to_owned(),
                "AACCCCCC".to_owned(),
            ],
        );
        assert_eq!(res, 3);
    }

    #[test]
    fn test4() {
        let res = super::MinimumGeneticMutation::min_mutation(
            "AACCGGTT".to_owned(),
            "AACCGGTA".to_owned(),
            vec![],
        );
        assert_eq!(res, -1);
    }

    #[test]
    fn test5() {
        let res = super::MinimumGeneticMutation::min_mutation(
            "AACCTTGG".to_owned(),
            "AATTCCGG".to_owned(),
            vec![
                "AATTCCGG".to_owned(),
                "AACCTGGG".to_owned(),
                "AACCCCGG".to_owned(),
                "AACCTACC".to_owned(),
            ],
        );
        assert_eq!(res, -1);
    }
}
