use std::collections::{HashMap, HashSet};

use super::{Cluster, UnionFind};

#[derive(Clone, Debug)]
pub struct DenseUF {
    cluster_ids: Vec<u32>, // maximum item id is limited by u32 bounds
    cluster_sizes: Vec<u32>,
    size: usize,
}

// public
impl DenseUF {
    /// create an empty disjoint set union with a given capacity
    pub fn new(capacity: usize) -> DenseUF {
        DenseUF {
            cluster_ids: vec![0; capacity + 1],
            cluster_sizes: vec![0; capacity + 1],
            size: 0,
        }
    }

    /// capacity of the DSU.
    /// The maximum ID that can be inserted is `capacity - 1``
    pub fn capacity(&self) -> usize {
        self.cluster_ids.len() - 1
    }
}

// private
impl DenseUF {
    fn cluster_size_of(&mut self, item: usize) -> usize {
        assert!(item < self.capacity());

        let cluster_id = self.cluster_id_of(item);
        if let Some(cluster_id) = cluster_id {
            let size = self.cluster_sizes.get(cluster_id).unwrap_or(&1);

            *size as usize
        } else {
            0
        }
    }
}

impl UnionFind for DenseUF {
    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn insert(&mut self, item: usize) {
        assert!(item < self.capacity());

        if self.cluster_ids[item + 1] == 0 {
            self.cluster_ids[item + 1] = (item + 1) as u32;
            self.size += 1;
        }
    }

    fn contains(&self, id: usize) -> bool {
        if id < self.capacity() && self.cluster_ids[id + 1] != 0 {
            return true;
        }

        false
    }

    fn cluster_id_of(&mut self, item: usize) -> Option<usize> {
        assert!(item < self.capacity());

        let mut current = item + 1;
        let mut parent = self.cluster_ids[current] as usize;
        if parent == 0 {
            return None;
        }

        while current != parent {
            let prev = current;

            current = parent;
            parent = self.cluster_ids[current] as usize;

            // shorten path towards the root skipping one intermediate node
            self.cluster_ids[prev] = parent as u32;
        }

        Some(current - 1)
    }

    fn join(&mut self, id1: usize, id2: usize) -> usize {
        assert!(id1 < self.capacity());
        assert!(id2 < self.capacity());

        self.insert(id1);
        self.insert(id2);

        if id1 == id2 {
            return id1;
        }

        let root1 = self.cluster_id_of(id1).unwrap();
        let root2 = self.cluster_id_of(id2).unwrap();

        if root1 == root2 {
            return root1;
        }

        let size1 = self.cluster_size_of(root1);
        let size2 = self.cluster_size_of(root2);

        if size1 < size2 {
            self.cluster_ids[root1 + 1] = (root2 + 1) as u32;
            self.cluster_sizes[root1] = 0;
            self.cluster_sizes[root2] = (size1 + size2) as u32;
            root2
        } else {
            self.cluster_ids[root2 + 1] = (root1 + 1) as u32;
            self.cluster_sizes[root2] = 0;
            self.cluster_sizes[root1] = (size1 + size2) as u32;
            root1
        }
    }

    fn connected(&mut self, id1: usize, id2: usize) -> bool {
        if let Some(root1) = self.cluster_id_of(id1) {
            if let Some(root2) = self.cluster_id_of(id2) {
                return self.cluster_id_of(root1) == self.cluster_id_of(root2);
            }
        }

        false
    }

    fn clusters(&mut self) -> Vec<Cluster> {
        let mut clusters = HashMap::<usize, HashSet<usize>>::new();

        for id in 0..self.capacity() {
            if let Some(cluster_id_for_node) = self.cluster_id_of(id) {
                clusters
                    .entry(cluster_id_for_node)
                    .or_insert_with(HashSet::new)
                    .insert(id);
            }
        }

        clusters
            .into_iter()
            .map(|(id, nodes)| Cluster { id, nodes })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use proptest::{prop_assert, proptest};

    use crate::union_find::{DenseUF, UnionFind};

    #[test]
    fn capacity_is_correct() {
        let set = DenseUF::new(1000);
        assert_eq!(set.capacity(), 1000);
    }

    proptest! {
        #[test]
        fn new_set_contains_nothing(num in 0..1000) {
            let set = DenseUF::new(1000);

            prop_assert!(!set.contains(num as usize));
        }

        #[test]
        fn cluster_id_is_none_if_item_isnt_inserted(num in 0..1000) {
            let mut set = DenseUF::new(1000);

            prop_assert!(set.cluster_id_of(num as usize).is_none());
        }
    }
}
