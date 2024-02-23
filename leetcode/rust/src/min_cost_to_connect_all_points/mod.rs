use crate::common::weighted_graph::WeightedGraph;

struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut graph = WeightedGraph::<()>::new();

        for id in 0..(points.len()) {
            graph.insert(id, ());
        }

        for (id1, point1) in points.iter().enumerate() {
            let x1 = point1[0];
            let y1 = point1[1];
            for (id2, point2) in points.iter().enumerate() {
                let x2 = point2[0];
                let y2 = point2[1];

                let dist = (x1 - x2).abs() + (y2 - y1).abs();
                graph.connect(id1, id2, dist);
            }
        }

        let mst = graph.mst_prim();
        mst.iter().map(|edge| edge.weight).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2d;

    use super::*;

    #[test]
    fn case1() {
        let points = vec2d![[0,0],[2,2],[3,10],[5,2],[7,0]];
        assert_eq!(Solution::min_cost_connect_points(points), 20);
    }

    #[test]
    fn case2() {
        let points = vec2d![[3,12],[-2,5],[-4,1]];
        assert_eq!(Solution::min_cost_connect_points(points), 18);
    }
}
