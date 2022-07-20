//! Shift 2D Grid
//!
//! # Link
//!
//! [1260. Shift 2D Grid](https://leetcode.com/problems/shift-2d-grid/)

pub struct Shift2DGrid;

impl Shift2DGrid {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len(); // Rows
        let n = grid[0].len(); // Columns
        let size = m * n;
        let actual_moves = (k as usize) % size;

        // Fast path: no movements will happen.
        if actual_moves == 0 {
            return grid;
        }

        let subscripts_from_pos = |pos: usize| -> (usize, usize) {
            let x = pos % n;
            let y = pos / n;
            (x, y)
        };

        let seq: Vec<_> = (0..size)
            .map(|i| {
                let (x, y) = subscripts_from_pos(i);
                return grid[y][x];
            })
            .collect();

        // Re-fill the grid.
        let mut grid_mut = grid;
        for (i, elem) in seq.iter().enumerate() {
            let offset_pos = (i + actual_moves) % size;
            let (x, y) = subscripts_from_pos(offset_pos);
            grid_mut[y][x] = *elem;
        }

        return grid_mut;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = input.clone();
        assert_eq!(super::Shift2DGrid::shift_grid(input, 9), output)
    }

    #[test]
    fn test2() {
        let input = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        let output = vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
        ];
        assert_eq!(super::Shift2DGrid::shift_grid(input, 4), output)
    }
}
