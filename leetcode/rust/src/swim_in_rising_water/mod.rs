use std::collections::{BinaryHeap, HashSet};

struct Solution;

#[derive(PartialEq, Eq, Debug)]
struct QueueEntry {
    min_time: i32,
    x: i8,
    y: i8,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.min_time.cmp(&self.min_time)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let rows = grid.len() as i8;
        let cols = grid[0].len() as i8;

        let mut visited = HashSet::with_capacity(rows as usize * cols as usize);
        let mut pq = BinaryHeap::with_capacity(rows as usize * cols as usize);
        pq.push(QueueEntry {
            min_time: grid[0][0],
            y: 0,
            x: 0,
        });

        while let Some(QueueEntry { min_time, y, x }) = pq.pop() {
            if x == cols - 1 && y == rows - 1 {
                return min_time;
            }

            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let x = x + dx;
                let y = y + dy;
                if x < 0 || y < 0 || x >= cols || y >= rows {
                    continue;
                }

                let min_time = grid[y as usize][x as usize].max(min_time);
                pq.push(QueueEntry { min_time, x, y })
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use algo_toolbox::{assert_returns, vec2d};
    use itertools::Itertools;

    use crate::common::random_uniform_list;

    use super::*;

    #[test]
    fn case1() {
        let grid = vec2d![[0, 2], [1, 3]];
        assert_returns!(3, Solution::swim_in_water, grid);
    }

    #[test]
    fn case2() {
        let grid =
            vec2d![
                [0, 1, 2, 3, 4],
                [24, 23, 22, 21, 5],
                [12, 13, 14, 15, 16],
                [11, 17, 18, 19, 20],
                [10, 9, 8, 7, 6]
            ];
        assert_returns!(16, Solution::swim_in_water, grid);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        let n = 100;
        let grid = (0..n).map(|_| random_uniform_list(n, 0, 30)).collect_vec();

        b.iter(|| Solution::swim_in_water(grid.clone()));
    }
}
