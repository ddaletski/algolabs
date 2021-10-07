use std::{ops::Index};

#[derive(Debug)]
struct Node<K, V, const DIM: usize> {
    key: K,
    value: V,
    level: usize,
    prev: Option<Box<Node<K, V, DIM>>>,
    next: Option<Box<Node<K, V, DIM>>>,
}

impl<K, V, const DIM: usize> Node<K, V, DIM>
where
    K: Index<usize>,
    K::Output: Ord,
{
    fn new(key: K, value: V, level: usize) -> Node<K, V, DIM> {
        Node {
            key,
            value,
            level,
            prev: None,
            next: None,
        }
    }

    fn insert(&mut self, key: K, value: V, level: usize) {
        match key[level].cmp(&self.key[level]) {
            std::cmp::Ordering::Less => {
                if let Some(node) = &mut self.prev {
                    node.insert(key, value, level.wrapping_add(1) % DIM);
                } else {
                    self.prev = Some(Box::new(Node::new(key, value, level)));
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(node) = &mut self.next {
                    node.insert(key, value, level.wrapping_add(1) % DIM);
                } else {
                    self.next = Some(Box::new(Node::new(key, value, level)));
                }
            }
            _ => {
                self.value = value;
            }
        }
    }

    fn get(&self, key: &K, level: usize) -> Option<&V> {
        match key[level].cmp(&self.key[level]) {
            std::cmp::Ordering::Less => {
                if let Some(node) = &self.prev {
                    node.get(key, level.wrapping_add(1) % DIM)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(node) = &self.next {
                    node.get(key, level.wrapping_add(1) % DIM)
                } else {
                    None
                }
            }
            _ => Some(&self.value),
        }
    }
}

#[derive(Debug)]
pub struct KDTree<K, V, const DIM: usize> {
    root: Option<Node<K, V, DIM>>,
}

impl<K, V, const DIM: usize> KDTree<K, V, DIM>
where
    K: Index<usize>,
    K::Output: Ord,
{
    pub fn new() -> Self {
        KDTree { root: None }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(node) = &self.root {
            node.get(key, 0)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(node) = &mut self.root {
            node.insert(key, value, 0);
        } else {
            self.root = Some(Node::new(key, value, 0));
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::seq::SliceRandom;

    use crate::*;
    const N_NODES: usize = 10000;

    type Point2i = [i32; 2];
    type Tree = KDTree<Point2i, i32, 2>;

    fn random_ints(count: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut ints = (0..(count as i32)).collect_vec();
        ints.shuffle(&mut rng);

        ints
    }

    fn random_points(count: usize) -> Vec<Point2i> {
        let x = random_ints(count);
        let y = random_ints(count);

        x.into_iter().zip(y).map(|(x, y)| [x, y]).collect()
    }

    #[test]
    fn cant_find_in_empty_tree() {
        let tree: Tree = KDTree::new();
        for p in random_points(N_NODES) {
            assert_eq!(tree.get(&p), None);
        }
    }

    #[test]
    fn can_find_all_inserted_vals() {
        let mut points = random_points(N_NODES);

        let mut tree = Tree::new();

        for p in points.iter() {
            let sum = p[0] + p[1];
            tree.insert(*p, sum);
        }

        let mut rng = rand::thread_rng();
        points.shuffle(&mut rng);

        for p in points.iter() {
            let val = tree.get(p);
            assert!(val.is_some());

            assert_eq!(*val.unwrap(), p[0] + p[1]);
        }
    }

    #[test]
    fn cant_find_not_inserted_keys() {
        let points = random_points(N_NODES);

        let mut tree = Tree::new();

        for p in points.iter().take(N_NODES/2) {
            let sum = p[0] + p[1];
            tree.insert(*p, sum);
        }

        for p in points.iter().skip(N_NODES/2) {
            let val = tree.get(p);
            assert!(val.is_none());
        }
    }

    #[test]
    fn replace_works() {
        let mut tree = Tree::new();

        let mut points = random_points(N_NODES);
        // f(x, y) == x + y
        for p in points.iter() {
            let sum = p[0] + p[1];
            tree.insert(*p, sum);
        }

        let mut rng = rand::thread_rng();
        points.shuffle(&mut rng);

        // f(x, y) == x + y + 2 * N_NODES
        for p in points.iter() {
            let sum = p[0] + p[1] + 2 * (N_NODES as i32);
            tree.insert(*p, sum);
        }

        // check if f(x, y) == x + y + 2 * N_NODES
        for p in points.iter() {
            assert_eq!(*tree.get(&p).unwrap(), p[0] + p[1] + 2 * (N_NODES as i32));
        }
    }
}
