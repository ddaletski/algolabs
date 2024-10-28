pub mod dway_heap;

pub trait MaxPriorityQueue<T> {
    fn max(&self) -> Option<&T>;
    fn pop_max(&mut self) -> Option<T>;
    fn push(&mut self, val: T);
}

pub trait MinPriorityQueue<T> {
    fn min(&self) -> Option<&T>;
    fn pop_min(&mut self) -> Option<T>;
    fn push(&mut self, val: T);
}