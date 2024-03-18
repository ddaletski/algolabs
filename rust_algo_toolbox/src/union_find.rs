mod dense_uf;
mod sparse_uf;
use std::collections::HashSet;

pub use dense_uf::*;
use derivative::Derivative;
pub use sparse_uf::SparseUF;

#[derive(Derivative)]
#[derivative(Default, Debug)]
pub struct Cluster {
    pub id: usize,
    #[derivative(Default(value = "HashSet::new()"))]
    pub nodes: HashSet<usize>,
}

pub trait UnionFind {
    /// get the number of items in set
    fn len(&self) -> usize;
    /// check if the set is empty
    fn is_empty(&self) -> bool;

    /// insert (disconnected) item to the set
    fn insert(&mut self, item: usize);
    /// check if the set contains the item
    fn contains(&self, item: usize) -> bool;
    /// join two items and return id of the cluster index they are in after joining
    /// if some (or both) of the items wasn't present, it's inserted before joining

    fn join(&mut self, item1: usize, item2: usize) -> usize;
    /// check whether items are connected
    fn connected(&mut self, item1: usize, item2: usize) -> bool;

    /// get the id of the item's connected component if it's inserted
    /// otherwise return None
    fn cluster_id_of(&mut self, item: usize) -> Option<usize>;
    /// get all connected components
    fn clusters(&mut self) -> Vec<Cluster>;
    /// get the number of connected cpmponents in the set
    fn clusters_count(&mut self) -> usize {
        self.clusters().len()
    }
}

///////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::assert_returns;

    use super::*;
    use rand::{distributions::Distribution, seq::SliceRandom, thread_rng, Rng};
    use spectral::prelude::*;
    use std::{fmt::Debug, ops::DerefMut};

    use rstest::{fixture, rstest};

    trait PrintableUF: UnionFind + Debug {}
    impl<T> PrintableUF for T where T: UnionFind + Debug {}

    type DynUF = Box<dyn PrintableUF>;
    const MAX_NODES: usize = 100;

    #[fixture]
    fn empty_set_sparse() -> DynUF {
        Box::new(SparseUF::new())
    }

    #[fixture]
    fn empty_set_dense() -> DynUF {
        Box::new(DenseUF::new(MAX_NODES))
    }

    /// sparse uf with disjoin set of values from 0 to 99
    #[fixture]
    fn set_100_orphans_sparse() -> DynUF {
        let mut empty_set_sparse = SparseUF::new();

        for i in 0..100 {
            empty_set_sparse.insert(i);
        }

        Box::new(empty_set_sparse)
    }

    /// dense uf with disjoin set of values from 0 to 99
    #[fixture]
    fn set_100_orphans_dense() -> DynUF {
        let mut empty_set_dense = DenseUF::new(100);

        for i in 0..100 {
            empty_set_dense.insert(i);
        }

        Box::new(empty_set_dense)
    }

    fn rand_permutation(from: usize, to: usize) -> Vec<usize> {
        let mut rng = thread_rng();
        let mut vec: Vec<usize> = (from..to).collect();
        vec.shuffle(&mut rng);

        vec
    }

    fn chain_nodes(nodes: &Vec<usize>) -> Vec<(usize, usize)> {
        assert!(nodes.len() >= 2);

        (0..(nodes.len() - 1))
            .map(|i| (nodes[i], nodes[i + 1]))
            .collect()
    }

    fn randomize_links(mut links: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut rng = thread_rng();

        // shuffle links
        links.shuffle(&mut rng);

        // swap links endpoints randomly
        for link in links.iter_mut() {
            if rng.gen_bool(0.5) {
                std::mem::swap(&mut link.0, &mut link.1);
            }
        }

        links
    }

    #[fixture]
    fn set_10by10_sparse() -> DynUF {
        let mut set = set_100_orphans_sparse();

        for cluster_idx in 0..10 {
            let from = (cluster_idx * 10) as usize;
            let to = ((cluster_idx + 1) * 10) as usize;
            let nodes = rand_permutation(from, to);
            let links = chain_nodes(&nodes);
            let links = randomize_links(links);

            for (a, b) in links {
                set.join(a, b);
            }
        }

        set
    }

    #[fixture]
    fn set_10by10_dense() -> DynUF {
        let mut set = set_100_orphans_dense();

        for cluster_idx in 0..10 {
            let from = (cluster_idx * 10) as usize;
            let to = ((cluster_idx + 1) * 10) as usize;
            let nodes = rand_permutation(from, to);
            let links = chain_nodes(&nodes);
            let links = randomize_links(links);

            for (a, b) in links {
                set.join(a, b);
            }
        }

        set
    }

    ////////////////////////////////////////////////////////////////////

    #[rstest]
    #[case(empty_set_sparse())]
    #[case(empty_set_dense())]
    fn insert_new_increments_size(#[case] empty_set: DynUF) {
        let mut uf = empty_set;
        for i in 0..10 {
            assert_eq!(uf.len(), i);
            uf.insert(i);
            assert_eq!(uf.len(), i + 1);
        }
    }

    #[rstest]
    #[case(empty_set_sparse())]
    #[case(empty_set_dense())]
    fn insert_existing_doesnt_change_size(#[case] empty_set: DynUF) {
        let mut uf = empty_set;
        for i in 0..10 {
            uf.insert(i);
        }

        for i in 0..10 {
            let size_before = uf.len();
            uf.insert(i);
            assert_eq!(uf.len(), size_before);
        }
    }

    #[rstest]
    #[case(set_100_orphans_sparse())]
    #[case(set_100_orphans_sparse())]
    fn inserted_items_are_have_unique_ids(#[case] mut set_100_orphans: DynUF) {
        for i in 0..100 {
            for j in (i + 1)..100 {
                assert_ne!(
                    set_100_orphans.cluster_id_of(i),
                    set_100_orphans.cluster_id_of(j)
                );
            }
        }
    }

    ////////////////////////////////////////////////////////////////////

    #[rstest]
    #[case(empty_set_sparse())]
    #[case(empty_set_dense())]
    fn items_in_empty_set_are_disconnected(#[case] mut empty_set: DynUF) {
        for i in 0..10 {
            for j in (i + 1)..10 {
                assert_returns!(false, UnionFind::connected, &mut *empty_set, i, j);
            }
        }
    }

    #[rstest]
    #[case(set_100_orphans_sparse())]
    #[case(set_100_orphans_dense())]
    fn inserted_items_are_disconnected(#[case] mut set_100_orphans: DynUF) {
        for i in 0..100 {
            for j in (i + 1)..100 {
                assert_returns!(false, UnionFind::connected, &mut *set_100_orphans, i, j);
            }
        }
        assert_returns!(100, UnionFind::clusters_count, &mut *set_100_orphans);
    }

    #[rstest]
    #[case(set_10by10_sparse())]
    #[case(set_10by10_dense())]
    fn connected_returns_true_for_connected_nodes(#[case] mut set_10by10: DynUF) {
        for id1 in 0..100 {
            let cluster_min = (id1 / 10) * 10;
            let cluster_max = ((id1 + 1) / 10) * 10;
            for id2 in cluster_min..cluster_max {
                assert_returns!(true, UnionFind::connected, &mut *set_10by10, id1, id2);
            }
        }
    }

    #[rstest]
    #[case(set_10by10_sparse())]
    #[case(set_10by10_dense())]
    fn clusters_count_is_correct(#[case] mut set_10by10: DynUF) {
        assert_returns!(10, UnionFind::clusters_count, &mut *set_10by10);
    }

    #[rstest]
    #[case(set_10by10_sparse())]
    #[case(set_10by10_dense())]
    fn clusters_content_is_correct(#[case] mut set_10by10: DynUF) {
        let mut clusters: Vec<Cluster> = set_10by10.clusters();
        clusters.sort_by_key(|cluster| cluster.id);

        for i in 0..10 {
            let expected_content: Vec<usize> = ((i * 10)..((i + 1) * 10)).collect();
            assert_that(&clusters[i as usize].nodes).contains_all_of(&expected_content.iter());
        }
    }

    #[rstest]
    #[case(set_10by10_sparse())]
    #[case(set_10by10_dense())]
    fn joining_n_clusters_makes_single_cluster(#[case] mut set_10by10: DynUF) {
        let mut rng = thread_rng();

        let some_node_for_each_cluster: Vec<usize> = (0..10)
            .map(|comp_id| comp_id * 10 + rng.gen_range(0..10))
            .collect();

        let intercluster_links = chain_nodes(&some_node_for_each_cluster);
        let intercluster_links = randomize_links(intercluster_links);

        for (from, to) in intercluster_links {
            set_10by10.join(from, to);
        }

        assert_returns!(1, UnionFind::clusters_count, &mut *set_10by10);
    }

    #[rstest]
    #[case(empty_set_sparse())]
    #[case(empty_set_dense())]
    fn disjoint_set_gives_unit_clusters(#[case] empty_set: DynUF) {
        let mut uf = empty_set;
        for i in 0..10 {
            uf.insert(i);
        }

        let clusters = uf.clusters();
        assert_that!(clusters).has_length(10);
        for c in clusters {
            assert_eq!(c.nodes.len(), 1);
        }
    }

    #[rstest]
    #[case(set_100_orphans_sparse())]
    #[case(set_100_orphans_dense())]
    fn joining_items_makes_their_ids_equal(#[case] mut set_100_orphans: DynUF) {
        let mut rng = thread_rng();
        let id_distr1 = rand::distributions::Uniform::from(0..100);
        let id_distr2 = rand::distributions::Uniform::from(0..100);

        for _ in 0..1000 {
            let item1 = id_distr1.sample(&mut rng);
            let item2 = id_distr2.sample(&mut rng);

            set_100_orphans.join(item1, item2);
            assert_eq!(
                set_100_orphans.cluster_id_of(item1),
                set_100_orphans.cluster_id_of(item2)
            );
        }
    }

    #[rstest]
    #[case(empty_set_sparse())]
    #[case(empty_set_dense())]
    fn item_has_id_of_its_terminal_link(#[case] mut uf: DynUF) {
        uf.join(1, 2);
        uf.join(2, 3);
        uf.join(3, 4);
        uf.join(4, 5);

        for i in 1..=5 {
            assert_returns!(Some(1), UnionFind::cluster_id_of, &mut *uf, i);
        }
    }

    #[rstest]
    #[case(empty_set_sparse())]
    #[case(empty_set_dense())]
    fn clusters_are_valid_manual(#[case] empty_set: DynUF) {
        let mut uf = empty_set;

        uf.join(1, 2);
        assert_returns!(1, UnionFind::clusters_count, uf.deref_mut());
        uf.join(3, 4);
        assert_returns!(2, UnionFind::clusters_count, uf.deref_mut());

        uf.join(5, 6);
        assert_returns!(3, UnionFind::clusters_count, uf.deref_mut());
        uf.join(7, 8);
        assert_returns!(4, UnionFind::clusters_count, uf.deref_mut());

        uf.join(1, 4);
        assert_returns!(3, UnionFind::clusters_count, uf.deref_mut());
        uf.join(7, 6);
        assert_returns!(2, UnionFind::clusters_count, uf.deref_mut());

        uf.join(6, 4);
        assert_returns!(1, UnionFind::clusters_count, uf.deref_mut());

        let cluster_items = &uf.clusters()[0].nodes;

        let all_items: Vec<usize> = (1..=8).collect();
        assert_that(cluster_items).contains_all_of(&all_items.iter());
    }
}
