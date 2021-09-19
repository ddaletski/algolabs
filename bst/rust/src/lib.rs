type Link<K, V> = Option<Box<BSTNode<K, V>>>;
#[derive(Debug)]
struct BSTNode<K, V> {
    pub key: K,
    pub value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: Ord, V> BSTNode<K, V> {
    fn new(key: K, value: V) -> BSTNode<K, V> {
        BSTNode {
            key: key,
            value: value,
            left: None,
            right: None
        }
    }

    fn insert(&mut self, key: K, value: V) {
        match key.cmp(&self.key) {
            std::cmp::Ordering::Less => {
                if let Some(node) = &mut self.left {
                    node.insert(key, value);
                } else {
                    self.left = Some(Box::new(BSTNode::new(key, value)));
                }
            },
            std::cmp::Ordering::Greater => {
                if let Some(node) = &mut self.right {
                    node.insert(key, value);
                } else {
                    self.right = Some(Box::new(BSTNode::new(key, value)));
                }
            },
            _ => {
                self.value = value;
            }
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        match key.cmp(&self.key) {
            std::cmp::Ordering::Less => {
                if let Some(node) = &self.left {
                    node.get(key)
                } else {
                    None
                }
            },
            std::cmp::Ordering::Greater => {
                if let Some(node) = &self.right {
                    node.get(key)
                } else {
                    None
                }
            },
            _ => {
                Some(&self.value)
            },
        }
    }

    fn size(&self) -> usize {
        let left_size = if let Some(node) = &self.left {
            node.size()
        } else {
            0
        };
        let right_size = if let Some(node) = &self.right {
            node.size()
        } else {
            0
        };

        1 + left_size + right_size
    }
}

#[derive(Debug)]
pub struct BST<K, V> {
    root: Link<K, V>,
}

impl<K: Ord, V> BST<K, V> {
    pub fn new() -> BST<K, V> {
        BST { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(root) = &mut self.root {
            root.insert(key, value);
        } else {
            self.root = Some(Box::new(BSTNode::new(key, value)));
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(root) = &self.root {
            root.get(key)
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        if let Some(root) = &self.root {
            root.size()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::seq::SliceRandom;
    use rand::{random, Rng};

    use crate::BST;
    type Tree = BST<usize, usize>;
    const N_NODES: usize = 1000;

    fn random_ints(count: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut ints = (0..count).collect_vec();
        ints.shuffle(&mut rng);

        ints
    }

    #[test]
    fn cant_find_in_empty_bst() {
        let bst = Tree::new();
        for i in 0..N_NODES {
            assert_eq!(bst.get(&i), None);
        }
    }

    #[test]
    fn can_find_all_inserted_vals() {
        let ints = random_ints(N_NODES);

        let mut bst = Tree::new();

        for i in ints.iter() {
            bst.insert(*i, i + N_NODES);
        }

        let ints = random_ints(N_NODES);

        for k in ints.iter() {
            let v = bst.get(k);
            assert!(v.is_some());
            assert_eq!(*v.unwrap(), k + N_NODES);
        }
    }


    #[test]
    fn replace_works() {
        let mut bst = Tree::new();

        let ints = random_ints(N_NODES);

        // insert f(x) = x + N_NODES
        for i in ints.iter() {
            bst.insert(*i, i + N_NODES);
        }

        // insert f(x) = x + 2 * N_NODES
        for i in ints.iter() {
            bst.insert(*i, i + 2 * N_NODES);
        }

        // check if f(x) == x + 2 * N_NODES
        for i in ints.iter() {
            assert_eq!(*bst.get(&i).unwrap(), i + 2 * N_NODES);
        }
    }
}
