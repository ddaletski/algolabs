use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, words: Vec<String>) -> i32 {
        let mut queue = VecDeque::with_capacity(words.len());
        queue.push_back((begin_word.as_str(), 0));

        let mut words_left: Vec<&str> = words.iter().map(|w| w.as_str()).collect();

        while let Some((next_word, distance)) = queue.pop_front() {
            if next_word == &end_word {
                return distance + 1;
            }

            for i in (0..words_left.len()).rev() {
                if Self::adjacent(next_word, words_left[i]) {
                    queue.push_back((words_left[i], distance + 1));
                    words_left.swap_remove(i);
                }
            }
        }

        0
    }

    fn adjacent(a: &str, b: &str) -> bool {
        let mut diffs = 0;

        for (ch1, ch2) in std::iter::zip(a.bytes(), b.bytes()) {
            if ch1 != ch2 {
                diffs += 1;
                if diffs > 1 {
                    return false;
                }
            }
        }

        diffs == 1
    }
}

#[cfg(test)]
mod tests {
    use algo_toolbox::assert_returns;
    use rand::{Rng, SeedableRng};
    use tap::Pipe;

    use super::*;

    #[test]
    fn case1() {
        let word_list = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ];
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        assert_returns!(5, Solution::ladder_length, begin_word, end_word, word_list);
    }

    #[test]
    fn case2() {
        let word_list =
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
            ];
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        assert_returns!(0, Solution::ladder_length, begin_word, end_word, word_list);
    }

    #[bench]
    fn large_test(b: &mut test::Bencher) {
        let n = 1000;
        let word_len = 10;
        let words_list: Vec<String> =
            rand::rngs::StdRng::seed_from_u64(0).pipe(|mut rng| {
                (0..n)
                    .map(|_| {
                        (0..word_len)
                            .map(|_| rng.gen_range('a'..='z'))
                            .collect::<String>()
                    })
                    .collect()
            });

        let begin_word = words_list[0].clone();
        let end_word = words_list[1].clone();
        b.iter(|| Solution::ladder_length(begin_word.clone(), end_word.clone(), words_list.clone()));
    }
}
