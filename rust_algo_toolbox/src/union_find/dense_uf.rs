use derivative::Derivative;
use itertools::Itertools;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    iter::FromIterator,
};

use super::{Cluster, UnionFind};

#[derive(Derivative, Clone)]
#[derivative(Default)]
pub struct DenseUF {
    cluster_ids: Vec<u32>, // maximum DSU size is limited by u32 bounds

    #[derivative(Default(value = "HashMap::new()"))]
    cluster_sizes: HashMap<u32, u32>,

    #[derivative(Default(value = "0"))]
    size: usize,
}

// public
impl DenseUF {
    /// create an empty disjoint set union with a given capacity
    pub fn new(capacity: usize) -> DenseUF {
        DenseUF {
            cluster_ids: Vec::from_iter(std::iter::repeat(0).take(capacity + 1)),
            ..Default::default()
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
            let size = self.cluster_sizes.get(&(cluster_id as u32)).unwrap_or(&1);

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

        // // change to this to test perf w/o path compression
        // let root1 = self.cluster_id(id1).unwrap();
        // let root2 = self.cluster_id(id2).unwrap();
        let root1 = self.cluster_id_of(id1).unwrap();
        let root2 = self.cluster_id_of(id2).unwrap();

        if root1 == root2 {
            return root1;
        }

        let size1 = self.cluster_size_of(root1);
        let size2 = self.cluster_size_of(root2);

        if size1 < size2 {
            self.cluster_ids[root1 + 1] = (root2 + 1) as u32;
            self.cluster_sizes.remove(&(root1 as u32));
            self.cluster_sizes
                .insert(root2 as u32, (size1 + size2) as u32);
            root2
        } else {
            self.cluster_ids[root2 + 1] = (root1 + 1) as u32;
            self.cluster_sizes.remove(&(root2 as u32));
            self.cluster_sizes
                .insert(root1 as u32, (size1 + size2) as u32);
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

        for id in 0..self.len() {
            if let Some(cluster_id_for_node) = self.cluster_id_of(id) {
                match clusters.entry(cluster_id_for_node) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => v.insert(HashSet::new()),
                }
                .insert(id);
            }
        }

        clusters
            .into_iter()
            .map(|(id, nodes)| Cluster { id, nodes })
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use rstest::*;
    use std::collections::HashSet;

    use crate::union_find::UnionFind;

    use super::DenseUF;

    const N_NODES: usize = 1024;
    const N_EDGES: usize = N_NODES * 2;
    const N_CLUSTERS: usize = 4;

    #[fixture]
    fn random_dsu() -> DenseUF {
        let mut rng = rand::thread_rng();
        let mut dsu = DenseUF::new(N_NODES);
        for _ in 0..N_EDGES {
            let id1 = rng.gen_range(0..N_NODES);
            let id2 = rng.gen_range(0..N_NODES);

            dsu.join(id1, id2);
        }

        dsu
    }

    #[fixture]
    fn clusters() -> Vec<HashSet<usize>> {
        let mut rng = rand::thread_rng();
        let mut all_values: Vec<usize> = (0..N_NODES).collect();
        all_values.shuffle(&mut rng);

        let cluster_size = f32::ceil((all_values.len() as f32) / (N_CLUSTERS as f32)) as usize;

        let mut clusters = Vec::new();
        for cluster_idx in 0..N_CLUSTERS {
            let mut set = HashSet::<usize>::new();
            for id in all_values
                .iter()
                .skip(cluster_idx * cluster_size)
                .take(cluster_size)
            {
                set.insert(id.clone());
            }
            clusters.push(set);
        }

        clusters
    }

    #[rstest]
    fn insert_new_increments_size() {
        let mut dsu = DenseUF::new(N_NODES);
        for i in 0..N_NODES {
            assert_eq!(dsu.len(), i);
            dsu.insert(i);
            assert_eq!(dsu.len(), i + 1);
        }
    }

    #[rstest]
    fn insert_existing_doesnt_change_size() {
        let mut dsu = DenseUF::new(N_NODES);
        for i in 0..N_NODES {
            dsu.insert(i);
        }

        for i in 0..N_NODES {
            let size_before = dsu.len();
            dsu.insert(i);
            assert_eq!(dsu.len(), size_before);
        }
    }

    #[rstest]
    fn clusters_fixture_nonoverlaping(clusters: Vec<HashSet<usize>>) {
        for (c1, c2) in clusters
            .iter()
            .enumerate()
            .cartesian_product(clusters.iter().enumerate())
            .filter(|((i1, _), (i2, _))| i1 != i2)
            .map(|((_, c1), (_, c2))| (c1, c2))
        {
            for id1 in c1 {
                for id2 in c2 {
                    assert_ne!(id1, id2);
                }
            }
        }
    }

    #[rstest]
    fn join_makes_cluster_id_equal(clusters: Vec<HashSet<usize>>) {
        let mut dsu = DenseUF::new(N_NODES);
        let first_id = *clusters[0].iter().take(1).next().unwrap();

        for id in clusters.get(0).unwrap() {
            dsu.join(*id, first_id);
        }

        let first_cluster_id = dsu.cluster_id_of(first_id).unwrap();
        for id in clusters.get(0).unwrap() {
            assert_eq!(dsu.cluster_id_of(*id).unwrap(), first_cluster_id);
        }
    }

    #[rstest]
    fn different_clusters_have_different_ids(clusters: Vec<HashSet<usize>>) {
        let mut dsu = DenseUF::new(N_NODES);

        for cluster in clusters.iter() {
            let first_id = *cluster.iter().take(1).next().unwrap();

            for id in cluster.iter() {
                dsu.join(*id, first_id);
            }
        }

        for (c1, c2) in clusters
            .iter()
            .enumerate()
            .cartesian_product(clusters.iter().enumerate())
            .filter(|((i1, _), (i2, _))| i1 != i2)
            .map(|((_, c1), (_, c2))| (c1, c2))
        {
            for id1 in c1.iter().take(10) {
                for id2 in c2.iter().take(10) {
                    let cluster_id1 = dsu.cluster_id_of(*id1);
                    let cluster_id2 = dsu.cluster_id_of(*id2);
                    assert_ne!(cluster_id1, cluster_id2);
                }
            }
        }
    }


    #[rstest]
    fn empty_dsu_has_no_clusters() {
        let mut dsu = DenseUF::new(N_NODES);

        assert_eq!(dsu.clusters().len(), 0);
    }

    #[rstest]
    fn clusters_count_is_valid(clusters: Vec<HashSet<usize>>) {
        let mut dsu = DenseUF::new(N_NODES);

        for (i, cluster) in clusters.iter().enumerate() {
            let first_id = *cluster.iter().take(1).next().unwrap();

            for id in cluster.iter() {
                dsu.join(*id, first_id);
            }
            assert_eq!(dsu.clusters().len(), i + 1);
        }
    }

    #[rstest]
    fn non_entries_arent_connected() {
        let mut dsu = DenseUF::new(N_NODES);

        for (i, j) in (0..N_NODES).cartesian_product(0..N_NODES) {
            assert!(!dsu.connected(i, j));
            assert!(!dsu.connected(j, i));
        }
    }

    #[rstest]
    fn connected_works_for_disconnected_nodes(clusters: Vec<HashSet<usize>>) {
        let mut dsu = DenseUF::new(N_NODES);

        let in_dsu = clusters.get(0).unwrap();
        let not_in_dsu = clusters.get(1).unwrap();

        let first_id = *in_dsu.iter().take(1).next().unwrap();
        for id in in_dsu {
            dsu.join(*id, first_id);
        }

        for id1 in in_dsu {
            for id2 in not_in_dsu {
                assert!(!dsu.connected(*id1, *id2));
                assert!(!dsu.connected(*id2, *id1));
            }
        }
    }

    #[rstest]
    fn connected_works_for_connected_nodes(clusters: Vec<HashSet<usize>>) {
        let mut dsu = DenseUF::new(N_NODES);

        let in_dsu = clusters.get(0).unwrap();

        let first_id = *in_dsu.iter().take(1).next().unwrap();
        for id in in_dsu {
            dsu.join(*id, first_id);
        }

        for id1 in in_dsu {
            for id2 in in_dsu {
                assert!(dsu.connected(*id1, *id2));
            }
        }
    }
}
