use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let mut bank: HashSet<_> = bank.into_iter().collect();

        let mut queue = VecDeque::with_capacity(bank.len());
        queue.push_back((0, start_gene.to_string()));

        while let Some((distance, gene)) = queue.pop_front() {
            if gene == end_gene {
                return distance;
            }

            for neighbor in Self::adjacent_genes(&gene, &bank) {
                bank.remove(&neighbor);
                queue.push_back((distance + 1, neighbor));
            }
        }

        -1
    }

    fn adjacent_genes(gene: &str, bank: &HashSet<String>) -> Vec<String> {
        let mut result = vec![];
        for (letter_idx, letter) in gene.bytes().enumerate() {
            let mut other_gene = gene.to_string();

            [b'A', b'C', b'G', b'T']
                .into_iter()
                .filter(|&l| l != letter)
                .for_each(|other_letter| {
                    unsafe {
                        other_gene.as_bytes_mut()[letter_idx] = other_letter;
                    }
                    if bank.contains(&other_gene) {
                        result.push(other_gene.clone());
                    }
                });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use algo_toolbox::assert_returns;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn case1() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AACCGGTA".to_string();
        let bank = vec!["AACCGGTA".to_string()];
        assert_returns!(1, Solution::min_mutation, start_gene, end_gene, bank);
    }

    #[test]
    fn case2() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AAACGGTA".to_string();
        let bank = ["AACCGGTA", "AACCGCTA", "AAACGGTA"]
            .into_iter()
            .map(|s| s.to_string())
            .collect_vec();
        assert_returns!(2, Solution::min_mutation, start_gene, end_gene, bank);
    }
}
