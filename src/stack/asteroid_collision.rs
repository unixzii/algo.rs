//! Asteroid Collision
//!
//! # Link
//!
//! [735. Asteroid Collision](https://leetcode.com/problems/asteroid-collision/)

use std::collections::LinkedList;

struct AsteroidCollision;

impl AsteroidCollision {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = LinkedList::new();
        let mut res = Vec::new();
        'outer: for asteroid in asteroids.into_iter() {
            if asteroid > 0 {
                stack.push_back(asteroid);
                continue;
            }

            while let Some(top) = stack.back().as_deref() {
                let top = *top;
                let asteroid_abs = asteroid.abs();
                if top >= asteroid_abs {
                    if top == asteroid_abs {
                        stack.pop_back();
                    }
                    continue 'outer;
                } else {
                    stack.pop_back();
                }
            }
            res.push(asteroid);
        }

        res.extend(stack);
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(
            super::AsteroidCollision::asteroid_collision(vec![5, 10, -5]),
            vec![5, 10]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            super::AsteroidCollision::asteroid_collision(vec![8, -8]),
            vec![]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            super::AsteroidCollision::asteroid_collision(vec![10, 2, -5]),
            vec![10]
        );
    }
}
