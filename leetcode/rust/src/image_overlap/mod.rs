pub struct Solution;

impl Solution {
    fn transform_row(row: &[i32]) -> i32 {
        row.iter()
            .enumerate()
            .map(|(idx, value)| value << idx)
            .sum()
    }

    fn transform_to_binary(img: Vec<Vec<i32>>) -> Vec<i32> {
        img.into_iter()
            .map(|row| Solution::transform_row(&row))
            .collect()
    }

    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len() as i32;

        let img1 = Solution::transform_to_binary(img1);
        let img2 = Solution::transform_to_binary(img2);

        let mut max_overlap = 0;

        for yshift in -n + 1..n {
            let min_y = i32::max(0, -yshift);
            let max_y = n + i32::min(0, -yshift);

            for xshift in -n + 1..n {
                let mut overlap = 0;

                for y in min_y..max_y {
                    let row1 = img1[y as usize];

                    let mut row2 = img2[(y + yshift) as usize];
                    if xshift < 0 {
                        row2 <<= -xshift;
                    } else {
                        row2 >>= xshift;
                    }

                    overlap += (row1 & row2).count_ones();
                }

                max_overlap = u32::max(overlap, max_overlap);
            }
        }

        max_overlap as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_returns, vec2d};

    #[test]
    fn case_1() {
        let img1 = vec2d![[1, 1, 0], [0, 1, 0], [0, 1, 0]];
        let img2 = vec2d![[0, 0, 0], [0, 1, 1], [0, 0, 1]];

        assert_returns!(3, Solution::largest_overlap, img1, img2);
    }
}
