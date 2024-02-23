pub struct Solution;
//////////////////////////

use std::collections::HashSet;
type Board = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn step_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn step_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn step_up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn step_down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
}

impl Solution {
    pub fn exist(board: Board, word: String) -> bool {
        for start_y in 0..board.len() {
            for start_x in 0..(board[0].len()) {
                let mut visited: HashSet<Point> = HashSet::new();
                let mut chars = word.chars().rev().collect();
                if Solution::find_impl(
                    &board,
                    Point {
                        x: start_x as i32,
                        y: start_y as i32,
                    },
                    &mut chars,
                    &mut visited,
                ) {
                    return true;
                }
            }
        }

        false
    }

    fn find_impl(
        board: &Board,
        pos: Point,
        chars_left: &mut Vec<char>,
        visited: &mut HashSet<Point>,
    ) -> bool {
        if chars_left.is_empty() {
            return true;
        }

        if visited.contains(&pos) {
            return false;
        }

        if pos.y < 0
            || pos.x < 0
            || pos.y as usize >= board.len()
            || pos.x as usize >= board[0].len()
        {
            return false;
        }

        let next_char = *chars_left.last().unwrap();

        if board[pos.y as usize][pos.x as usize] != next_char {
            return false;
        }

        chars_left.pop();
        visited.insert(pos.clone());

        let found = false
            || Solution::find_impl(board, pos.step_left(), chars_left, visited)
            || Solution::find_impl(board, pos.step_right(), chars_left, visited)
            || Solution::find_impl(board, pos.step_up(), chars_left, visited)
            || Solution::find_impl(board, pos.step_down(), chars_left, visited);

        visited.remove(&pos);
        chars_left.push(next_char);

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_returns, vec2d};
    use rstest::rstest;

    #[rstest]
    #[case(vec2d![["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCCED", true)]
    #[case(vec2d![["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "SEE", true)]
    #[case(vec2d![["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], "ABCB", false)]
    #[case(vec2d![["a","a","a"],    ["A","A","A"],    ["a","a","a"]],     "aAaaaAaaA", true)]
    #[rstest]
    fn it_works(#[case] board: Vec<Vec<&str>>, #[case] word: &str, #[case] expected_result: bool) {
        let board = board
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();

        assert_returns!(expected_result, Solution::exist, board, word.to_owned());
    }
}
