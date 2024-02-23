pub struct Solution;

////////////////////////////////////

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        points.into_iter().take(k as usize).collect()
    }
}

///////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use spectral::prelude::*;

    #[rstest]
    #[case(vec![[1,3],[-2,2]], 1, vec![[-2,2]])]
    #[case(vec![[3,3],[5,-1],[-2,4]], 2, vec![[3,3],[-2,4]])]
    fn it_works(
        #[case] points: Vec<[i32; 2]>,
        #[case] k: i32,
        #[case] expected_result: Vec<[i32; 2]>,
    ) {
        let result_ = Solution::k_closest(points.into_iter().map(|p| Vec::from(p)).collect(), k);
        let result: Vec<[i32; 2]> = result_.into_iter().map(|p| [p[0], p[1]]).collect();

        assert_that(&result).contains_all_of(&expected_result.iter());
    }
}
