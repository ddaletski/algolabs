use defaults::Defaults;
use std::{collections::HashMap, iter::FromIterator};

#[derive(Defaults)]
struct FastUnionUF {
    cluster_ids: Vec<u32>, // maximum UF size is limited by u32 bounds

    #[def = "HashMap::new()"]
    cluster_sizes: HashMap<u32, u32>,

    #[def = "0"]
    size: usize,
}

impl FastUnionUF {
    fn new(size: usize) -> FastUnionUF {
        FastUnionUF {
            cluster_ids: Vec::from_iter(std::iter::repeat(0).take(size + 1)),
            ..Default::default()
        }
    }

    fn insert(&mut self, id: usize) {
        assert!(id < self.cluster_ids.len() - 1);

        if self.cluster_ids[id + 1] == 0 {
            self.cluster_ids[id + 1] = (id + 1) as u32;
            self.size += 1;
        }
    }

    fn cluster_id(&self, id: usize) -> Option<usize> {
        let parent = self.cluster_ids[id + 1] as usize;
        if parent == id + 1 {
            Some(id)
        } else if parent == 0 {
            None
        } else {
            self.cluster_id(parent - 1)
        }
    }

    fn cluster_size(&self, id: usize) -> usize {
        assert!(id < self.cluster_ids.len() - 1);

        let cluster_id = self.cluster_id(id);
        if let Some(cluster_id) = cluster_id {
            let size = self.cluster_sizes.get(&(cluster_id as u32)).unwrap_or(&1);

            *size as usize
        } else {
            0
        }
    }

    fn union(&mut self, id1: usize, id2: usize) {
        self.insert(id1);
        self.insert(id2);

        let root1 = self.cluster_id(id1).unwrap();
        let root2 = self.cluster_id(id2).unwrap();

        let size1 = self.cluster_size(root1);
        let size2 = self.cluster_size(root2);

        if size1 < size2 {
            self.cluster_ids[root1 + 1] = (root2 + 1) as u32;
        } else {
            self.cluster_ids[root2 + 1] = (root1 + 1) as u32;
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::hash::Hash;

    use itertools::Itertools;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use rstest::*;

    use crate::FastUnionUF;

    #[fixture]
    fn random_uf() -> FastUnionUF {
        let mut rng = rand::thread_rng();
        let mut uf = FastUnionUF::new(1024);
        for _ in 1..100 {
            let id1 = rng.gen_range(0..1024);
            let id2 = rng.gen_range(0..1024);

            uf.union(id1, id2);
        }

        uf
    }

    #[fixture]
    fn clusters() -> Vec<HashSet<usize>> {
        let mut rng = rand::thread_rng();
        let mut all_values: Vec<usize> = (0..1024).collect();
        all_values.shuffle(&mut rng);

        let nclusters = 4;
        let cluster_size = f32::ceil((all_values.len() as f32) / (nclusters as f32)) as usize;

        let mut clusters = Vec::new();
        for cluster_idx in 0..nclusters {
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
        let mut uf = FastUnionUF::new(1024);
        for i in 0..1024 {
            assert_eq!(uf.size(), i);
            uf.insert(i);
            assert_eq!(uf.size(), i + 1);
        }
    }

    #[rstest]
    fn insert_existing_doesnt_change_size() {
        let mut uf = FastUnionUF::new(1024);
        for i in 0..1024 {
            uf.insert(i);
        }

        for i in 0..1024 {
            let size_before = uf.size();
            uf.insert(i);
            assert_eq!(uf.size(), size_before);
        }
    }

    #[rstest]
    fn lol(mut random_uf: FastUnionUF) {
        random_uf.insert(1);
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
    fn union_makes_cluster_id_equal(clusters: Vec<HashSet<usize>>) {
        let mut uf = FastUnionUF::new(1024);
        let first_id = *clusters[0].iter().take(1).next().unwrap();

        for id in clusters.get(0).unwrap() {
            uf.union(*id, first_id);
        }

        let first_cluster_id = uf.cluster_id(first_id).unwrap();
        for id in clusters.get(0).unwrap() {
            assert_eq!(uf.cluster_id(*id).unwrap(), first_cluster_id);
        }
    }

    #[rstest]
    fn different_clusters_have_different_ids(clusters: Vec<HashSet<usize>>) {
        let mut uf = FastUnionUF::new(1024);

        for cluster in clusters.iter() {
            let first_id = *cluster.iter().take(1).next().unwrap();

            for id in cluster.iter() {
                uf.union(*id, first_id);
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
                    let cluster_id1 = uf.cluster_id(*id1);
                    let cluster_id2 = uf.cluster_id(*id2);
                    assert_ne!(cluster_id1, cluster_id2);
                }
            }
        }
    }

    #[rstest]
    fn cluster_size_for_nonexistent_is_zero() {
        let mut uf = FastUnionUF::new(1024);
        for i in 0..1024 {
            assert_eq!(uf.cluster_size(i), 0);
        }
    }

    #[rstest]
    fn cluster_size_for_singleton_is_one() {
        let mut uf = FastUnionUF::new(1024);

        for i in 0..1024 {
            uf.insert(i);
            assert_eq!(uf.cluster_size(i), 1);
        }
    }

    #[rstest]
    fn cluster_size_is_valid_for_connected_set(clusters: Vec<HashSet<usize>>) {
        let mut uf = FastUnionUF::new(1024);

        let cluster = clusters.get(0).unwrap();
        let first_id = *cluster.iter().take(1).next().unwrap();
        for id in cluster.iter() {
            uf.union(*id, first_id);
        }

        for id in cluster {
            assert_eq!(uf.cluster_size(*id), cluster.len());
        }
    }

}
