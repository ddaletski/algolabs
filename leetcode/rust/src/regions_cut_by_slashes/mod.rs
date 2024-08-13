use algo_toolbox::union_find::{DenseUF, UnionFind};

struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();

        let mut uf = DenseUF::new(n * n * 4);

        for i in 0..n {
            for j in 0..(n - 1) {
                let current_triangle = (i * n + j) * 4 + 3;
                let right_triangle = (i * n + j + 1) * 4 + 1;

                uf.join(current_triangle, right_triangle);
            }
        }

        for i in 0..(n - 1) {
            for j in 0..n {
                let current_triangle = (i * n + j) * 4 + 2;
                let bottom_triangle = ((i + 1) * n + j) * 4;

                uf.join(current_triangle, bottom_triangle);
            }
        }


        for (i, row) in grid.into_iter().enumerate() {
            for (j, ch) in row.bytes().enumerate() {
                let block_offset = (i * n + j) * 4;

                let top = block_offset;
                let left = block_offset + 1;
                let bottom = block_offset + 2;
                let right = block_offset + 3;

                match ch {
                    b'/' => {
                        uf.join(top, left);
                        uf.join(bottom, right);
                    }
                    b'\\' => {
                        uf.join(top, right);
                        uf.join(bottom, left);
                    }
                    _ => {
                        uf.join(top, right);
                        uf.join(top, left);
                        uf.join(top, bottom);
                    }
                }
            }
        }

        uf.clusters_count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = vec![" /".to_string(), "/ ".to_string()];

        assert_eq!(Solution::regions_by_slashes(grid), 2);
    }

    #[test]
    fn case2() {
        let grid = vec![" /".to_string(), "  ".to_string()];

        assert_eq!(Solution::regions_by_slashes(grid), 1);
    }

    #[test]
    fn case3() {
        let grid = vec!["/\\".to_string(), "\\/".to_string()];

        assert_eq!(Solution::regions_by_slashes(grid), 5);
    }
}

