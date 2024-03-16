pub struct Solution {}
pub struct Solution2 {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let even = (n % 2 == 0) as usize;

        for radius in 1..=n / 2 {
            for i in 0..(radius * 2 - even) {
                let xy1 = n / 2 - radius;
                let xy2 = n / 2 + radius - even;

                let a = matrix[xy1][xy1 + i];
                let b = matrix[xy1 + i][xy2];
                let c = matrix[xy2][xy2 - i];
                let d = matrix[xy2 - i][xy1];

                matrix[xy1][xy1 + i] = d;
                matrix[xy1 + i][xy2] = a;
                matrix[xy2][xy2 - i] = b;
                matrix[xy2 - i][xy1] = c;
            }
        }
    }
}

impl Solution2 {
    fn flip(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for y in 0..n/2 {
            matrix.swap(y, n-y-1);
        }
    }

    fn transpose(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for y in 0..n {
            for x in 0..y {
                let a = matrix[y][x];
                let b = matrix[x][y];

                matrix[y][x] = b;
                matrix[x][y] = a;
            }
        }
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Solution2::flip(matrix);
        Solution2::transpose(matrix);
    }
}


#[cfg(test)]
mod tests {
    use crate::{vec2d, parse_2d_array};
    use super::*;

    #[test]
    fn test1_1() {
        let mut matrix = vec2d![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let expected = vec2d![[7, 4, 1], [8, 5, 2], [9, 6, 3]];

        Solution::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }

    #[test]
    fn test1_2() {
        let input_str = "[[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]";
        let expected_str = "[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]";

        let mut matrix = parse_2d_array(input_str);
        let expected = parse_2d_array(expected_str);

        Solution::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }

    #[test]
    fn test2_1() {
        let mut matrix = vec2d![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let expected = vec2d![[7, 4, 1], [8, 5, 2], [9, 6, 3]];

        Solution2::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }

    #[test]
    fn test2_2() {
        let input_str = "[[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]";
        let expected_str = "[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]";

        let mut matrix = parse_2d_array(input_str);
        let expected = parse_2d_array(expected_str);

        Solution2::rotate(&mut matrix);

        assert_eq!(matrix, expected);
    }
}
