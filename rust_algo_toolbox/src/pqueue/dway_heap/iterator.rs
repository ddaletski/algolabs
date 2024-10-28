use crate::pqueue::MaxPriorityQueue;

use super::DWayMaxHeap;

pub struct DWayMaxHeapIter<T, const D: usize> {
    heap: DWayMaxHeap<T, D>,
}

impl<T: Ord, const D: usize> Iterator for DWayMaxHeapIter<T, D> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.heap.len(), Some(self.heap.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop_max()
    }
}

impl<T: Ord, const D: usize> IntoIterator for DWayMaxHeap<T, D> {
    type Item = T;
    type IntoIter = DWayMaxHeapIter<T, D>;

    fn into_iter(self) -> Self::IntoIter {
        DWayMaxHeapIter { heap: self }
    }
}
