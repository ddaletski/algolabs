use rand::distributions::uniform::SampleRange;

struct Solution {
    data: Vec<i32>,
    permutation: Vec<usize>,
}

impl Solution {
    fn new(data: Vec<i32>) -> Self {
        let permutation = (0..data.len()).into_iter().collect();
        Self { data, permutation }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let n = self.len();
        for i in 0..n {
            let swap_idx = (i..n).sample_single(&mut rng);
            self.data.swap(i, swap_idx);
            self.permutation[i] = swap_idx;
        }
        self.data.clone()
    }

    fn reset(&mut self) -> Vec<i32> {
        for (i1, i2) in self.permutation.iter_mut().enumerate().rev() {
            self.data.swap(i1, *i2);
            *i2 = i1;
        }

        self.data.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let data = vec![1, 2, 3, 4, 5];
        let mut solution = Solution::new(data.clone());
        let mut shuffled = solution.shuffle();
        assert_ne!(shuffled, data);
        shuffled.sort();
        assert_eq!(shuffled, data);
        assert_eq!(solution.reset(), data);
    }
}
