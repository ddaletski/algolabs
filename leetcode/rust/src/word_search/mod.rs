pub struct Solution;
//////////////////////////

type Board = Vec<Vec<char>>;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    #[inline]
    fn step_left(self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    #[inline]
    fn step_right(self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    #[inline]
    fn step_up(self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    #[inline]
    fn step_down(self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
}

impl Solution {
    pub fn exist(board: Board, word: String) -> bool {
        if word.len() > board.len() * board[0].len() {
            return false;
        }
        if Self::invalid_board(&board, &word) {
            return false;
        }

        let word_chars: Vec<_> = word.chars().collect();
        let mut visited = vec![false; board.len() * board[0].len()];

        for start_y in 0..board.len() {
            for start_x in 0..(board[0].len()) {
                if board[start_y][start_x] != *word_chars.last().unwrap() {
                    continue;
                }

                if Solution::find_impl(
                    &board,
                    Point {
                        x: start_x as i16,
                        y: start_y as i16,
                    },
                    &word_chars,
                    &mut visited,
                ) {
                    return true;
                }
            }
        }

        false
    }

    fn invalid_board(board: &Board, word: &str) -> bool {
        const ALPHABET_SIZE: usize = 'z' as usize - 'A' as usize + 1;
        let mut board_freqs = vec![0; ALPHABET_SIZE];
        let mut word_freqs = vec![0; ALPHABET_SIZE];

        for row in board {
            for &c in row {
                board_freqs[c as usize - 'A' as usize] += 1;
            }
        }

        for c in word.chars() {
            word_freqs[c as usize - 'A' as usize] += 1;
            if word_freqs[c as usize - 'A' as usize] > board_freqs[c as usize - 'A' as usize] {
                return true;
            }
        }

        false
    }

    fn find_impl(board: &Board, pos: Point, word: &[char], visited: &mut [bool]) -> bool {
        if word.is_empty() {
            return true;
        }

        if pos.y < 0
            || pos.x < 0
            || pos.y as usize >= board.len()
            || pos.x as usize >= board[0].len()
        {
            return false;
        }

        let pos_hash = pos.x as usize * board.len() + pos.y as usize;

        if visited[pos_hash] {
            return false;
        }

        let next_char = *word.last().unwrap();

        if board[pos.y as usize][pos.x as usize] != next_char {
            return false;
        }

        visited[pos_hash] = true;

        let sub_word = &word.split_last().unwrap().1;
        let found = false
            || Solution::find_impl(board, pos.step_left(), sub_word, visited)
            || Solution::find_impl(board, pos.step_right(), sub_word, visited)
            || Solution::find_impl(board, pos.step_up(), sub_word, visited)
            || Solution::find_impl(board, pos.step_down(), sub_word, visited);

        visited[pos_hash] = false;

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_returns, vec2d};
    use criterion::black_box;
    use rand::{Rng, SeedableRng};
    use rstest::rstest;
    use test::Bencher;

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

    fn random_grid(height: usize, width: usize) -> Vec<Vec<char>> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| {
                        let c = rng.gen_range('a'..='z');
                        c
                    })
                    .collect()
            })
            .collect()
    }

    fn random_word(len: usize) -> String {
        let mut rng = rand::rngs::StdRng::seed_from_u64(1);

        (0..len)
            .map(|_| {
                let c = rng.gen_range('a'..='z');
                c
            })
            .collect()
    }

    #[bench]
    fn bench_huge_grid(b: &mut Bencher) {
        let grid = black_box(random_grid(1000, 1000));
        let words: Vec<String> = [3, 5, 10, 100]
            .into_iter()
            .map(|n| random_word(n))
            .collect();

        b.iter(|| {
            for word in &words {
                Solution::exist(grid.clone(), word.clone());
            }
        })
    }
}
