use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize - 1;

        let mut adj_list = vec![vec![]; n];
        for (src, dst, weight) in times
            .into_iter()
            .map(|v| (v[0] as usize - 1, v[1] as usize - 1, v[2]))
        {
            adj_list[src].push((dst, weight));
        }

        let mut costs = vec![i32::MAX; n];

        let mut pq = BinaryHeap::with_capacity(n);
        pq.push(Reverse((0, k)));

        while let Some(Reverse((cost, node_idx))) = pq.pop() {
            for &(neighbor, neighbor_weight) in &adj_list[node_idx] {
                let new_cost = cost + neighbor_weight;
                if new_cost < costs[neighbor] {
                    costs[neighbor] = new_cost;
                    pq.push(Reverse((new_cost, neighbor)));
                }
            }
        }

        match costs
            .into_iter()
            .enumerate()
            .filter(|&(idx, _)| idx != k)
            .map(|(_, x)| x)
            .max()
        {
            Some(i32::MAX) => -1,
            Some(other) => other,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algo_toolbox::{assert_returns, vec2d};
    use rstest::rstest;

    #[rstest]
    #[case(2, vec2d![[2,1,1],[2,3,1],[3,4,1]], 4, 2)]
    #[case(1, vec2d![[1,2,1]], 2, 1)]
    #[case(-1, vec2d![[1,2,1]], 2, 2)]
    #[case(3, vec2d![[1,2,1],[2,1,3]], 2, 2)]
    #[case(-1, vec2d![[1,2,1],[2,3,2],[1,3,1]], 3, 2)]
    fn test(#[case] expected: i32, #[case] times: Vec<Vec<i32>>, #[case] n: i32, #[case] k: i32) {
        assert_returns!(expected, Solution::network_delay_time, times, n, k);
    }
}
