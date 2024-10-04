use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ptr::null_mut,
    usize,
};

use crate::stateful_iterator::StatefulIterator;

#[inline]
fn leak<T>(val: T) -> *mut T {
    Box::leak(Box::new(val))
}

struct Node<T> {
    freq: usize,
    keys: HashSet<T>,
    prev: *mut Node<T>,
    next: *mut Node<T>,
    parent: *mut FreqList<T>,
}

struct FreqList<T> {
    // end is head and tail at the same time, it always exist to avoid dealing with too many corner cases
    end: *mut Node<T>, // dummy node with 0 freq
}

impl<T: Eq + Hash> FreqList<T> {
    fn new() -> *mut Self {
        let list = leak(Self { end: null_mut() });

        let end = leak(Node {
            freq: 0,
            keys: HashSet::new(),
            prev: null_mut(),
            next: null_mut(),
            parent: list,
        });

        unsafe {
            let list = list.as_mut().unwrap();
            list.end = end;

            let end = end.as_mut().unwrap();

            end.next = list.end;
            end.prev = list.end;
        }

        list
    }
}

pub struct OrderedCounter<T> {
    freq_list: *mut FreqList<T>,
    map: HashMap<T, *mut Node<T>>,
}

impl<T: Eq + Hash + Clone> OrderedCounter<T> {
    pub fn new() -> Self {
        Self {
            freq_list: FreqList::new(),
            map: HashMap::new(),
        }
    }

    /// increment the frequency of `key`
    pub fn inc(&mut self, key: &T) -> usize {
        let node_ptr = {
            if let Some(node) = self.map.get(key) {
                *node
            } else {
                // just point it to the end (which conveniently has 0 frequency)
                self.map.insert(key.clone(), self.end());
                self.end()
            }
        };

        let node = unsafe { node_ptr.as_mut().unwrap() };
        node.keys.remove(key);

        let Some(next) = (unsafe { node.next.as_mut() }) else {
            unreachable!(); // dummy end node should always be after any real node
        };

        if next.freq == node.freq + 1 {
            // next frequency node exists, updating its content
            next.keys.insert(key.clone());
            *self.map.get_mut(key).unwrap() = next;
        } else {
            // inserting a new freq node
            let mut keys = HashSet::with_capacity(1);
            keys.insert(key.clone());

            let new_node = leak(Node {
                freq: node.freq + 1,
                keys,
                prev: node,
                next: node.next,
                parent: node.parent,
            });

            next.prev = new_node;
            node.next = new_node;

            *self.map.get_mut(key).unwrap() = new_node;
        }

        if node.keys.is_empty() {
            self.remove_node(node_ptr);
        }

        node.freq + 1
    }

    /// decrement the frequency of `key`
    pub fn dec(&mut self, key: &T) -> usize {
        let Some(&node_ptr) = self.map.get(key) else {
            return 0;
        };

        let node = unsafe { node_ptr.as_mut().unwrap() };

        node.keys.remove(key);

        let Some(prev) = (unsafe { node.prev.as_mut() }) else {
            unreachable!(); // dummy end node should always be after any real node
        };

        if node.freq == 1 {
            // counter becomes 0, so we remove the entry from the mapping
            self.map.remove(key);
        } else if prev.freq == node.freq - 1 {
            // prev frequency node exists, updating its content
            prev.keys.insert(key.clone());
            *self.map.get_mut(key).unwrap() = prev;
        } else {
            // inserting a new freq node
            let mut keys = HashSet::with_capacity(1);
            keys.insert(key.clone());

            let new_node = leak(Node {
                freq: node.freq - 1,
                keys,
                prev: node.prev,
                next: node,
                parent: node.parent,
            });

            prev.next = new_node;
            node.prev = new_node;

            *self.map.get_mut(key).unwrap() = new_node;
        }

        if node.keys.is_empty() {
            self.remove_node(node_ptr);
        }

        node.freq - 1
    }

    /// get the frequency of `key`
    pub fn get(&self, key: &T) -> Option<usize> {
        let node = self.map.get(key)?;
        let node = unsafe { node.as_ref().unwrap() };
        Some(node.freq)
    }

    /// get the key with the lowest frequency
    pub fn min(&self) -> Option<&T> {
        let end = self.end();
        let node = unsafe { end.as_ref().unwrap() }.next;
        if node == end {
            return None;
        }

        let keys = unsafe { &node.as_ref().unwrap().keys };
        keys.iter().next()
    }

    /// get the key with the highest frequency
    pub fn max(&self) -> Option<&T> {
        let end = self.end();
        let node = unsafe { end.as_ref().unwrap() }.prev;
        if node == end {
            return None;
        }

        let keys = unsafe { &node.as_ref().unwrap().keys };
        keys.iter().next()
    }

    /// get an iterator of the keys sorted by frequency in increasing order
    pub fn sorted_increasing(&self) -> impl Iterator<Item = (&T, usize)> + '_ {
        self.sorted_generic::<false>()
    }

    /// get an iterator of the keys sorted by frequency in decreasing order
    pub fn sorted_decreasing(&self) -> impl Iterator<Item = (&T, usize)> + '_ {
        self.sorted_generic::<true>()
    }

    fn end(&self) -> *mut Node<T> {
        unsafe { self.freq_list.as_ref().unwrap().end }
    }

    fn remove_node(&self, node: *mut Node<T>) {
        unsafe {
            if node == self.end() {
                return;
            }
            let node = node.as_mut().unwrap();
            let prev = node.prev.as_mut().unwrap();
            let next = node.next.as_mut().unwrap();

            prev.next = next;
            next.prev = prev;
        }
    }

    fn sorted_generic<const BACKWARD: bool>(&self) -> impl Iterator<Item = (&T, usize)> + '_ {
        struct IterState<'a, T, const BACKWARD: bool> {
            collection: &'a OrderedCounter<T>,
            node: *mut Node<T>,
            keys_remained: Box<dyn Iterator<Item = &'a T> + 'a>,
        }

        if self.map.is_empty() {
            return StatefulIterator::new(
                IterState::<T, BACKWARD> {
                    collection: self,
                    node: null_mut(),
                    keys_remained: Box::new(std::iter::empty()),
                },
                |_| None,
            );
        }

        let first_node = if BACKWARD {
            unsafe { self.end().as_ref() }.unwrap().prev
        } else {
            unsafe { self.end().as_ref() }.unwrap().next
        };

        let initial_state = IterState {
            collection: self,
            node: first_node,
            keys_remained: Box::new(unsafe { first_node.as_ref() }.unwrap().keys.iter()),
        };

        fn transform<T: Eq + Clone + Hash, const BACKWARD: bool>(
            mut state: IterState<T, BACKWARD>,
        ) -> Option<((&T, usize), IterState<T, BACKWARD>)> {
            if state.node == state.collection.end() {
                return None;
            }

            let node = unsafe { state.node.as_ref().unwrap() };
            let next_key = state.keys_remained.next();

            if let Some(key) = next_key {
                Some(((key, node.freq), state))
            } else {
                state.node = if BACKWARD { node.prev } else { node.next };
                state.keys_remained = Box::new(unsafe { state.node.as_ref() }.unwrap().keys.iter());

                transform(state)
            }
        }

        StatefulIterator::new(initial_state, transform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn data() -> Vec<i32> {
        vec![1, 2, 3, 2, 1, 2, 2, 4, 4, 1]
    }

    #[fixture]
    fn counter(data: Vec<i32>) -> OrderedCounter<i32> {
        let mut counter = OrderedCounter::new();
        for num in data {
            counter.inc(&num);
        }
        counter
    }

    #[rstest]
    fn get_existing(counter: OrderedCounter<i32>) {
        assert_eq!(counter.get(&1), Some(3));
        assert_eq!(counter.get(&2), Some(4));
        assert_eq!(counter.get(&3), Some(1));
        assert_eq!(counter.get(&4), Some(2));
    }

    #[rstest]
    fn get_missing(counter: OrderedCounter<i32>) {
        assert_eq!(counter.get(&0), None);
        assert_eq!(counter.get(&-2), None);
        assert_eq!(counter.get(&123), None);
    }

    #[rstest]
    fn min_valid(counter: OrderedCounter<i32>) {
        assert_eq!(counter.min(), Some(&3));
    }

    #[rstest]
    fn max_valid(counter: OrderedCounter<i32>) {
        assert_eq!(counter.max(), Some(&2));
    }

    #[rstest]
    fn decrement(mut counter: OrderedCounter<i32>) {
        assert_eq!(counter.dec(&2), 3);
        assert_eq!(counter.get(&2), Some(3));

        assert_eq!(counter.dec(&2), 2);
        assert_eq!(counter.get(&2), Some(2));

        assert_eq!(counter.dec(&2), 1);
        assert_eq!(counter.get(&2), Some(1));

        assert_eq!(counter.dec(&2), 0);
        assert_eq!(counter.get(&2), None);

        assert_eq!(counter.dec(&2), 0);
        assert_eq!(counter.get(&2), None);

        assert_eq!(counter.max(), Some(&1));
    }

    #[rstest]
    fn sorted_increasing(counter: OrderedCounter<i32>) {
        let mut sorted = counter.sorted_increasing();
        assert_eq!(sorted.next(), Some((&3, 1)));
        assert_eq!(sorted.next(), Some((&4, 2)));
        assert_eq!(sorted.next(), Some((&1, 3)));
        assert_eq!(sorted.next(), Some((&2, 4)));
        assert_eq!(sorted.next(), None);
    }

    #[rstest]
    fn sorted_decreasing(counter: OrderedCounter<i32>) {
        let mut sorted = counter.sorted_decreasing();
        assert_eq!(sorted.next(), Some((&2, 4)));
        assert_eq!(sorted.next(), Some((&1, 3)));
        assert_eq!(sorted.next(), Some((&4, 2)));
        assert_eq!(sorted.next(), Some((&3, 1)));
        assert_eq!(sorted.next(), None);
    }
}
