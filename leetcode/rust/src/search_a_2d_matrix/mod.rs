pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false;
        }

        if matrix[0].is_empty() {
            return false;
        }

        Solution::search_impl(&matrix, 0, matrix.len() * matrix[0].len(), target)
    }

    fn search_impl(matrix: &Vec<Vec<i32>>, from: usize, to: usize, target: i32) -> bool {
        if from == to {
            return false;
        }

        let mid = (from + to) / 2;

        let w = matrix[0].len();
        let mid_val = matrix[mid / w][mid % w];

        match target.cmp(&mid_val) {
            std::cmp::Ordering::Less => {
                Solution::search_impl(matrix, from, mid, target)
            }
            std::cmp::Ordering::Greater => {
                Solution::search_impl(matrix, mid + 1, to, target)
            }
            _ => {
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::{assert_returns, vec2d};
    use rstest::{fixture, rstest};

    #[rstest]
    fn empty() {
        assert_returns!(false, Solution::search_matrix, vec![] as Vec<Vec<i32>>, 1);
    }

    #[rstest]
    fn empty2() {
        assert_returns!(false, Solution::search_matrix, vec2d![[], []] as Vec<Vec<i32>>, 1);
    }

    #[fixture]
    fn matrix() -> Vec<Vec<i32>> {
        vec2d![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]]
    }

    #[rstest]
    #[case(1, true)]
    #[case(5, true)]
    #[case(16, true)]
    #[case(60, true)]
    #[case(23, true)]
    #[case(-1, false)]
    #[case(2, false)]
    #[case(9, false)]
    #[case(18, false)]
    #[case(100, false)]
    fn case1(matrix: Vec<Vec<i32>>, #[case] value: i32, #[case] expected_result: bool) {
        assert_returns!(expected_result, Solution::search_matrix, matrix, value);
    }
}
