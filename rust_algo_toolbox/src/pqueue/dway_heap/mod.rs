//! D-way max-heap implementation

pub mod iterator;

use super::MaxPriorityQueue;

/// D-way max-heap
#[derive(Debug, Clone)]
pub struct DWayMaxHeap<T, const D: usize = 4> {
    data: Vec<T>,
}

impl<T: Ord, const D: usize> DWayMaxHeap<T, D> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn from_slice(data: &[T]) -> Self
    where
        T: Clone,
    {
        Self::from_iter(data.iter().cloned())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.data.len());

        for _ in 0..self.data.len() {
            result.push(self.pop_max().unwrap());
        }

        result
    }

    fn _bubble_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }

        let parent_idx = (idx - 1) / D;

        let val = &self.data[idx];
        let parent_val = &self.data[parent_idx];

        if val <= parent_val {
            return;
        }

        self.data.swap(idx, parent_idx);
        self._bubble_up(parent_idx);
    }

    fn _bubble_down(&mut self, idx: usize) {
        let slice_start = idx * D + 1;
        let slice_end = (slice_start + D).min(self.data.len());
        if slice_end <= slice_start {
            return;
        }

        let mut max_val = &self.data[idx];
        let mut max_idx = idx;
        for i in slice_start..slice_end {
            if &self.data[i] > max_val {
                max_idx = i;
                max_val = &self.data[i];
            }
        }

        if idx == max_idx {
            return;
        }

        self.data.swap(idx, max_idx);

        self._bubble_down(max_idx);
    }
}

impl<T: Ord, const D: usize> FromIterator<T> for DWayMaxHeap<T, D> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::new();
        for val in iter {
            heap.push(val);
        }
        heap
    }
}

impl<T: Ord, const D: usize> MaxPriorityQueue<T> for DWayMaxHeap<T, D> {
    fn max(&self) -> Option<&T> {
        self.data.first()
    }

    fn pop_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let max = self.data.swap_remove(0);

        self._bubble_down(0);

        Some(max)
    }

    fn push(&mut self, val: T) {
        let idx = self.data.len();
        self.data.push(val);
        self._bubble_up(idx);
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};

    use super::*;

    const D: usize = 4;

    #[rstest::fixture]
    fn rng() -> impl Rng {
        rand::rngs::StdRng::from_seed([0; 32])
    }

    #[rstest::fixture]
    fn data(rng: impl Rng) -> Vec<i32> {
        let distr = rand::distributions::Uniform::new_inclusive(-1000, 1000);
        rng.sample_iter(distr).take(1000).collect()
    }

    #[rstest::rstest]
    fn max_returns_biggest_val(data: Vec<i32>) {
        let heap: DWayMaxHeap<i32, D> = data.iter().cloned().collect();
        assert!(data.iter().all(|x| x <= heap.max().unwrap()));
    }

    #[rstest::rstest]
    fn pop_max_removes_biggest_val(data: Vec<i32>) {
        let max_val = *data.iter().max().unwrap();

        let mut heap: DWayMaxHeap<i32, D> = data.iter().cloned().collect();
        assert_eq!(heap.pop_max(), Some(max_val));
        assert_eq!(heap.len(), data.len() - 1);
    }

    #[rstest::rstest]
    fn pop_max_returns_none_on_empty_heap() {
        let mut heap: DWayMaxHeap<i32, D> = DWayMaxHeap::new();
        assert_eq!(heap.pop_max(), None);
    }

    #[rstest::rstest]
    fn max_peeks_biggest_val(data: Vec<i32>) {
        let heap: DWayMaxHeap<i32, D> = data.iter().cloned().collect();
        assert_eq!(heap.max(), Some(data.iter().max().unwrap()));
        assert_eq!(heap.len(), data.len());
    }

    #[rstest::rstest]
    fn heap_is_sorted(data: Vec<i32>) {
        let heap: DWayMaxHeap<i32, D> = data.iter().cloned().collect();
        let heap_sorted = heap.into_iter().collect::<Vec<_>>();
        let sorted = {
            let mut sorted = data;
            sorted.sort_unstable_by_key(|v| -v);
            sorted
        };

        assert_eq!(heap_sorted, sorted);
    }
}