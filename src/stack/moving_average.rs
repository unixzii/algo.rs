//! Moving Average from Data Stream
//!
//! # Link
//!
//! [346. Moving Average from Data Stream](https://leetcode.com/problems/moving-average-from-data-stream/)

use std::collections::VecDeque;

struct MovingAverage {
    window: VecDeque<i32>,
    window_sum: i64,
    window_size: usize,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        let window_size = size as usize;
        Self {
            window: VecDeque::with_capacity(size as usize),
            window_sum: 0,
            window_size,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.window.len() == self.window_size {
            // The window is full, evict one leading element.
            let elem = self.window.pop_front().unwrap();
            self.window_sum -= elem as i64;
        }

        self.window.push_back(val);
        self.window_sum += val as i64;

        self.window_sum as f64 / self.window.len() as f64
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let mut moving_average = super::MovingAverage::new(3);
        assert!((moving_average.next(1) - 1f64).abs() < f64::EPSILON);
        assert!((moving_average.next(10) - 5.5f64).abs() < f64::EPSILON);
        assert!((moving_average.next(3) - 4.666666666666667f64).abs() < f64::EPSILON);
        assert!((moving_average.next(5) - 6f64).abs() < f64::EPSILON);
    }
}
