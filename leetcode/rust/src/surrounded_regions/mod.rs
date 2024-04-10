use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        Self::replace_o_with_a_at_borders(board);

        for row in board {
            for ch in row {
                match *ch {
                    'O' => *ch = 'X',
                    'A' => *ch = 'O',
                    _ => {}
                }
            }
        }
    }

    fn replace_o_with_a_at_borders(board: &mut Vec<Vec<char>>) {
        let height = board.len() as i32;
        let width = board[0].len() as i32;

        let mut visited = HashSet::with_capacity((width * height) as usize);
        let mut queue = VecDeque::with_capacity((width * height) as usize);

        for start_x in 0..width {
            queue.push_back((0, start_x));
            queue.push_back((height - 1, start_x));
        }
        for start_y in 1..(height - 1) {
            queue.push_back((start_y, 0));
            queue.push_back((start_y, width - 1));
        }
        while let Some(pos @ (y, x)) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            let ch = &mut board[y as usize][x as usize];
            if *ch != 'O' {
                continue;
            }

            *ch = 'A';

            [(y, x + 1), (y, x - 1), (y - 1, x), (y + 1, x)]
                .into_iter()
                .filter(|&(y, x)| x >= 0 && x < width && y >= 0 && y < height)
                .for_each(|neighbor| {
                    queue.push_back(neighbor);
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algo_toolbox::vec2d;

    #[test]
    fn case1() {
        let mut board = vec2d![
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'O', 'X'],
            ['X', 'X', 'O', 'X'],
            ['X', 'O', 'X', 'X']
        ];
        let expected = vec2d![
            ['X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'X', 'X']
        ];

        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn case2() {
        let mut board = vec2d![['O']];
        let expected = vec2d![['O']];

        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn case3() {
        let mut board = vec2d![
            ['X', 'O', 'X', 'O', 'X', 'O'],
            ['O', 'X', 'O', 'X', 'O', 'X'],
            ['X', 'O', 'X', 'O', 'X', 'O'],
            ['O', 'X', 'O', 'X', 'O', 'X']
        ];
        let expected = vec2d![
            ['X', 'O', 'X', 'O', 'X', 'O'],
            ['O', 'X', 'X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X', 'X', 'O'],
            ['O', 'X', 'O', 'X', 'O', 'X']
        ];

        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }
}
