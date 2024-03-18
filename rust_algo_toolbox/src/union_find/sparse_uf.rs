use std::collections::{HashMap, HashSet};

use super::{Cluster, UnionFind};

#[derive(Debug, Default)]
pub struct SparseUF {
    mapping: HashMap<usize, usize>,
}

impl SparseUF {
    /// create a new empty set
    pub fn new() -> Self {
        SparseUF::default()
    }
}

impl UnionFind for SparseUF {
    fn len(&self) -> usize {
        self.mapping.len()
    }

    fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    fn join(&mut self, item1: usize, item2: usize) -> usize {
        match (self.cluster_id_of(item1), self.cluster_id_of(item2)) {
            (None, None) => {
                self.mapping.insert(item1, item1);
                self.mapping.insert(item2, item1);
                item1
            }

            (Some(id1), None) => {
                self.mapping.insert(item2, id1);
                id1
            }

            (None, Some(id2)) => {
                self.mapping.insert(item1, id2);
                id2
            }

            (Some(id1), Some(id2)) => {
                self.mapping.insert(id2, id1);
                id1
            }
        }
    }

    fn connected(&mut self, item1: usize, item2: usize) -> bool {
        let id1 = self.cluster_id_of(item1);
        let id2 = self.cluster_id_of(item2);

        id1 == id2 && id1.is_some()
    }

    fn contains(&self, item: usize) -> bool {
        self.mapping.contains_key(&item)
    }

    fn insert(&mut self, item: usize) {
        if let None = self.cluster_id_of(item) {
            self.mapping.insert(item, item);
        }
    }

    fn cluster_id_of(&mut self, item: usize) -> Option<usize> {
        let chain_of_ids = |item: usize| {
            let mut chain = vec![];

            let mut curr_item = item;
            while let Some(&next_item) = self.mapping.get(&curr_item) {
                chain.push(curr_item);
                if next_item == curr_item {
                    return chain;
                }
                curr_item = next_item;
            }

            chain
        };

        if let Some((last_id, other_ids)) = chain_of_ids(item).split_last() {
            for id in other_ids {
                self.mapping.insert(*id, *last_id);
            }
            Some(*last_id)
        } else {
            None
        }
    }

    fn clusters(&mut self) -> Vec<Cluster> {
        let mut cluster_id_to_items = HashMap::new();
        for &item in self.mapping.clone().keys() {
            cluster_id_to_items
                .entry(self.cluster_id_of(item).unwrap())
                .or_insert(HashSet::new())
                .insert(item);
        }

        cluster_id_to_items
            .into_iter()
            .map(|(cluster_id, items)| Cluster {
                id: cluster_id,
                nodes: items,
            })
            .collect()
    }
}

///////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prop_assert, proptest};

    proptest! {
        #[test]
        fn new_set_contains_nothing(num in 0..1000) {
            let set = SparseUF::new();

            prop_assert!(!set.contains(num as usize));
        }

        #[test]
        fn cluster_id_is_none_if_item_isnt_inserted(num in 0..1000) {
            let mut set = SparseUF::new();

            prop_assert!(set.cluster_id_of(num as usize).is_none());
        }
    }
}
