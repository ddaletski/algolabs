struct Solution;

fn grid_iter(height: usize, width: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |y| (0..width).map(move |x| (y, x)))
}

impl Solution {
    pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
        let height = mat.len();
        let width = mat[0].len();

        let mut max_distance_to = vec![vec![-1; width]; height];

        let start_positions = {
            let mut positions: Vec<_> = grid_iter(height, width).collect();
            positions.sort_unstable_by_key(|&(y, x)| mat[y][x]);
            positions
        };
        let mut stack = Vec::with_capacity(width * height / 4);
        for (start_y, start_x) in start_positions {
            stack.push(((start_y, start_x), 0));
            while let Some(((y, x), dist)) = stack.pop() {
                if dist <= max_distance_to[y][x] {
                    continue;
                }
                max_distance_to[y][x] = dist;

                for x1 in (0..width)
                    .filter(|&x1| x1 != x)
                    .filter(|&x1| mat[y][x1] > mat[y][x])
                {
                    stack.push(((y, x1), dist + 1));
                }

                for y1 in (0..height)
                    .filter(|&y1| y1 != y)
                    .filter(|&y1| mat[y1][x] > mat[y][x])
                {
                    stack.push(((y1, x), dist + 1));
                }
            }
        }

        max_distance_to.into_iter().flatten().max().unwrap_or(-1) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_returns, common, vec2d};
    use rstest::rstest;

    #[rstest]
    #[case(vec2d![[1, 1], [1, 1]], 1)]
    #[case(vec2d![[3, 1, 6], [-9, 5, 7]], 4)]
    fn case(#[case] mat: Vec<Vec<i32>>, #[case] expected: i32) {
        assert_returns!(expected, Solution::max_increasing_cells, mat);
    }

    #[bench]
    fn random_matrix(b: &mut test::Bencher) {
        let height = 50;
        let width = 50;

        let mat: Vec<Vec<i32>> = (0..height)
            .map(|_| common::random_uniform_list(width, -10000, 10000))
            .collect();

        b.iter(|| Solution::max_increasing_cells(mat.clone()));
    }
}
