use algo_toolbox::union_find::{SparseUF, UnionFind};

struct Solution;

impl Solution {
    fn neighbors(row: i32, col: i32) -> [(i32, i32); 8] {
        [
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]
    }

    // top and bottom are disconnected if left and right sides are 8-connected by water
    // the approach is to use DSU to connect water cells day by day
    // and check if at the end of the day left and right sides are connected by water
    pub fn latest_day_to_cross(rows: i32, cols: i32, cells: Vec<Vec<i32>>) -> i32 {
        let ndays = cells.len();

        // dummy cell to which every water cell on the left side is connected
        let left_id = (rows * cols + 1) as usize;
        // dummy cell to which every water cell on the right side is connected
        let right_id = (rows * cols + 2) as usize;

        let mut uf = SparseUF::new();

        for (day, (row, col)) in cells.into_iter().map(|v| (v[0] - 1, v[1] - 1)).enumerate() {
            let id = (row * cols + col) as usize;
            uf.insert(id);

            for (n_row, n_col) in Solution::neighbors(row, col) {
                if n_row < 0 || n_row >= rows || n_col < 0 || n_col >= cols {
                    continue;
                }

                let n_id = (n_row * cols + n_col) as usize;

                if !uf.contains(n_id) {
                    continue;
                }

                uf.join(id, n_id);
            }

            if col == 0 {
                uf.join(left_id, id);
            }

            if col == cols - 1 {
                uf.join(right_id, id);
            }

            if uf.connected(left_id, right_id) {
                return day as i32;
            }
        }

        ndays as i32
    }
}

#[cfg(test)]
mod test {
    use crate::vec2d;

    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::latest_day_to_cross(2, 2, vec2d![[1, 1], [2, 1], [1, 2], [2, 2]]),
            2
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::latest_day_to_cross(2, 2, vec2d![[1, 1], [1, 2], [2, 1], [2, 2]]),
            1
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::latest_day_to_cross(
                3,
                3,
                vec2d![
                    [1, 2],
                    [2, 1],
                    [3, 3],
                    [2, 2],
                    [1, 1],
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [3, 1]
                ]
            ),
            3
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::latest_day_to_cross(
                3,
                3,
                vec2d![
                    [1, 1],
                    [2, 1],
                    [3, 1],
                    [1, 3],
                    [2, 3]
                ]
            ),
            5
        );
    }
    #[test]
    fn case5() {
        assert_eq!(
            Solution::latest_day_to_cross(
                2,
                6,
                vec2d![
                    [1, 4],
                    [1, 3],
                    [2, 1],
                    [2, 5],
                    [2, 2],
                    [1, 5],
                    [2, 4],
                    [1, 2],
                    [1, 6],
                    [2, 3],
                    [2, 6],
                    [1, 1]
                ]
            ),
            8
        );
    }
}
