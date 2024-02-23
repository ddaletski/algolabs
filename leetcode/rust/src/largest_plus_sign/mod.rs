pub struct Solution {}

impl Solution {
    pub fn fill_mines_grid(n: usize, mines: Vec<Vec<i32>>) -> Vec<bool> {
        let mut mines_grid: Vec<bool> = vec![];
        mines_grid.resize((n * n) as usize, false);
        for mine in mines {
            mines_grid[(mine[0] * n as i32 + mine[1]) as usize] = true;
        }
        mines_grid
    }

    pub fn fill_orders_grid(n: usize) -> Vec<i32> {
        let mut orders_grid: Vec<i32> = vec![];
        orders_grid.resize(n * n, n as i32);
        orders_grid
    }

    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mines_grid = Solution::fill_mines_grid(n, mines);
        let mut orders_grid = Solution::fill_orders_grid(n);

        // for each line:
        // - find segments containing only 1s
        // - use segment endpoints to filter possible orders of all crosses with centers inside the segment
        for y in 0..n {
            let mut start = 0;
            while start < n {
                // find segment containing only 1s
                while start < n && mines_grid[y * n + start] {
                    orders_grid[y * n + start] = 0;
                    start += 1;
                }
                if start == n {
                    break;
                }
                let mut end = start + 1;
                while end < n && !mines_grid[(y * n + end) as usize] {
                    end += 1;
                }

                for x in start..(start + end) / 2 {
                    let order = &mut orders_grid[(y * n + x) as usize];
                    *order = (*order).min((x - start + 1) as i32);
                }

                for x in (start + end) / 2..end {
                    let order = &mut orders_grid[(y * n + x) as usize];
                    *order = (*order).min((end - x) as i32);
                }

                start = end;
            }
        }

        // the same for vertical lines
        for x in 0..n {
            let mut start = 0;
            while start < n {
                while start < n && mines_grid[(start * n + x) as usize] {
                    start += 1;
                }
                if start == n {
                    break;
                }
                let mut end = start + 1;
                while end < n && !mines_grid[(end * n + x) as usize] {
                    end += 1;
                }

                for y in start..(start + end) / 2 {
                    let order = &mut orders_grid[(y * n + x) as usize];
                    *order = (*order).min((y - start + 1) as i32);
                }

                for y in (start + end) / 2..end {
                    let order = &mut orders_grid[(y * n + x) as usize];
                    *order = (*order).min((end - y) as i32);
                }

                start = end;
            }
        }

        orders_grid.iter().max().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_returns, vec2d};
    use rstest::rstest;

    #[rstest]
    #[case(5, vec2d![[4,2]], 2)]
    #[case(1, vec2d![[0,0]], 0)]
    #[case(5, vec2d![[0,1],[0,2],[1,0],[1,2],[1,4],[2,0],[2,2],[3,0],[3,1],[4,0],[4,1],[4,3],[4,4]], 1)]
    fn it_works(#[case] n: i32, #[case] mines: Vec<Vec<i32>>, #[case] expected: i32) {
        assert_returns!(
            expected,
            Solution::order_of_largest_plus_sign,
            n,
            mines.clone()
        );
    }
}
