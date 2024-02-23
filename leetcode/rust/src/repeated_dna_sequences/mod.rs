use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }

        let mask: u32 = (1 << 20) - 1;
        let mut sec_hash = 0;

        let mut freq_map: HashMap<u32, u32> = HashMap::new();
        for (idx, c) in s.chars().enumerate() {
            let bit = match c {
                'A' => 0,
                'C' => 1,
                'G' => 2,
                'T' => 3,
                _ => unreachable!(),
            };
            sec_hash = (sec_hash << 2) | bit;

            if idx >= 9 {
                *(freq_map.entry(sec_hash & mask).or_default()) += 1;
            }
        }

        freq_map
            .into_iter()
            .filter(|(_, freq)| *freq > 1)
            .map(|(mut hash, _)| {
                let mut seq = vec![];
                for _ in 0..10 {
                    let bit = hash & 0b11;
                    let c = match bit {
                        0 => 'A',
                        1 => 'C',
                        2 => 'G',
                        3 => 'T',
                        _ => unreachable!(),
                    };
                    seq.push(c);
                    hash >>= 2;
                }

                seq.into_iter().rev().collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
        let expected = vec!["AAAAACCCCC", "CCCCCAAAAA"];
        let result = Solution::find_repeated_dna_sequences(s);
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let s = "AAAAAAAAAAAAA".to_string();
        let expected = vec!["AAAAAAAAAA"];
        let result = Solution::find_repeated_dna_sequences(s);
        assert_eq!(expected, result);
    }
}
