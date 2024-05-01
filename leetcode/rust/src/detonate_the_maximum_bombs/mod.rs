struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        if bombs.is_empty() {
            return 0;
        }

        let adj_list = Self::build_graph(bombs);

        Self::max_directed_component(&adj_list) as i32
    }

    fn build_graph(bombs: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let n = bombs.len();
        let mut adj_list = vec![vec![]; n];

        for (i, bomb1) in bombs.iter().enumerate() {
            let x1 = bomb1[0];
            let y1 = bomb1[1];
            let r1 = bomb1[2] as i64;
            for (j, bomb2) in bombs.iter().take(i).enumerate() {
                let x2 = bomb2[0];
                let y2 = bomb2[1];
                let r2 = bomb2[2] as i64;

                let distance_sq = Self::distance_squared(x1, y1, x2, y2);
                if r1 * r1 >= distance_sq {
                    adj_list[i].push(j);
                }
                if r2 * r2 >= distance_sq {
                    adj_list[j].push(i);
                }
            }
        }

        adj_list
    }

    fn max_directed_component(adj_list: &Vec<Vec<usize>>) -> usize {
        let n = adj_list.len();
        let mut visited = vec![false; n];
        let mut stack = Vec::with_capacity(n);
        let mut max_size = 0;

        for start in 0..n {
            visited.fill(false);
            stack.clear();

            let mut size = 0;
            stack.push(start);
            visited[start] = true;
            while let Some(idx) = stack.pop() {
                size += 1;

                for &neighbor in &adj_list[idx] {
                    if !visited[neighbor] {
                        stack.push(neighbor);
                        visited[neighbor] = true;
                    }
                }
            }

            max_size = max_size.max(size);
        }
        max_size
    }

    fn distance_squared(x1: i32, y1: i32, x2: i32, y2: i32) -> i64 {
        let dx = (x1 - x2) as i64;
        let dy = (y1 - y2) as i64;
        dx * dx + dy * dy
    }
}

#[cfg(test)]
mod tests {
    use algo_toolbox::assert_returns;

    use super::*;

    #[test]
    fn case1() {
        let bombs = vec![vec![2, 1, 3], vec![6, 1, 4]];
        assert_returns!(2, Solution::maximum_detonation, bombs);
    }

    #[test]
    fn case2() {
        let bombs = vec![vec![1, 1, 5], vec![10, 10, 5]];
        assert_returns!(1, Solution::maximum_detonation, bombs);
    }

    #[test]
    fn case3() {
        let bombs = vec![
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 3],
            vec![5, 6, 4],
        ];
        assert_returns!(5, Solution::maximum_detonation, bombs);
    }

    #[test]
    fn case4() {
        let bombs = vec![vec![1, 1, 100000], vec![100000, 100000, 1]];
        assert_returns!(1, Solution::maximum_detonation, bombs);
    }
}
