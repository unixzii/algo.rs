//! Pacific Atlantic Water Flow
//!
//! # Link
//!
//! [417. Pacific Atlantic Water Flow](https://leetcode-cn.com/problems/pacific-atlantic-water-flow/)

use std::collections::VecDeque;

const ID_PACIFIC: u8 = 1;
const ID_ATLANTIC: u8 = 2;
const ID_BOTH: u8 = 3;

struct State {
    heights: Vec<Vec<i32>>,
    mat_width: usize,
    mat_height: usize,
    bfs_queue: VecDeque<(usize, usize)>,
    memorized_state: Vec<u8>,
}

impl State {
    fn solve(&mut self) -> Vec<Vec<i32>> {
        // Visit all the left-side cells.
        for y in 0..self.mat_height {
            self.bfs_visit(0, y, ID_PACIFIC);
        }
        // Visit all the top-side cells.
        for x in 0..self.mat_width {
            self.bfs_visit(x, 0, ID_PACIFIC);
        }
        // Visit all the right-side cells.
        let x = self.mat_width - 1;
        for y in 0..self.mat_height {
            self.bfs_visit(x, y, ID_ATLANTIC);
        }
        // Visit all the bottom-side cells.
        let y = self.mat_height - 1;
        for x in 0..self.mat_width {
            self.bfs_visit(x, y, ID_ATLANTIC);
        }

        return self
            .memorized_state
            .iter()
            .enumerate()
            .filter(|(_, s)| **s & ID_BOTH == ID_BOTH)
            .map(|(idx, _)| {
                vec![
                    idx as i32 / self.mat_width as i32,
                    idx as i32 % self.mat_width as i32,
                ]
            })
            .collect();
    }

    fn bfs_visit(&mut self, initial_x: usize, initial_y: usize, kind: u8) {
        self.bfs_queue.push_back((initial_x, initial_y));

        while !self.bfs_queue.is_empty() {
            let (x, y) = self.bfs_queue.pop_front().unwrap();

            let kind_visited_bit: u8 = kind << 2;
            let index = y * self.mat_width + x;
            let mut state = self.memorized_state[index];
            if state & kind_visited_bit != 0 {
                // This cell has been visited.
                continue;
            }

            state |= kind_visited_bit;
            state |= kind;
            self.memorized_state[index] = state;

            let height = self.heights[y][x];

            if x > 0 {
                self.maybe_enqueue_cell(x - 1, y, height);
            }
            if y > 0 {
                self.maybe_enqueue_cell(x, y - 1, height);
            }
            self.maybe_enqueue_cell(x + 1, y, height);
            self.maybe_enqueue_cell(x, y + 1, height);
        }
    }

    fn maybe_enqueue_cell(&mut self, x: usize, y: usize, from_height: i32) {
        if x >= self.mat_width || y >= self.mat_height {
            return;
        }

        let height = self.heights[y][x];
        if height < from_height {
            // Only visit from the lower cells to the higher cells.
            return;
        }

        self.bfs_queue.push_back((x, y));
    }
}

pub struct PacificAtlanticWaterFlow;

impl PacificAtlanticWaterFlow {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.is_empty() {
            return vec![];
        }

        let mat_width = heights[0].len();
        let mat_height = heights.len();

        let mut state = State {
            heights,
            mat_width,
            mat_height,
            bfs_queue: VecDeque::new(),
            memorized_state: vec![0 as u8; mat_width * mat_height],
        };

        return state.solve();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let nums = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let res = super::PacificAtlanticWaterFlow::pacific_atlantic(nums.to_vec());
        // TODO: add vector equality check.
        println!("{:?}", res);
        assert_eq!(res.len(), 7);
    }

    #[test]
    fn test2() {
        let nums = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let res = super::PacificAtlanticWaterFlow::pacific_atlantic(nums.to_vec());
        // TODO: add vector equality check.
        println!("{:?}", res);
        assert_eq!(res.len(), 7);
    }
}
