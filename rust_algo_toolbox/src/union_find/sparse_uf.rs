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

    /// get the number of connected components in the set
    pub fn clusters_count(&mut self) -> usize {
        let mut cluster_ids = HashSet::new();

        for &item in self.mapping.clone().keys() {
            cluster_ids.insert(self.cluster_id_of(item).unwrap());
        }

        cluster_ids.len()
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
///
#[cfg(test)]
mod tests {
    use crate::assert_returns;

    use super::*;
    use proptest::{prop_assert, proptest};
    use rand::{distributions::Distribution, seq::SliceRandom, thread_rng, Rng};
    use spectral::prelude::*;

    use rstest::{fixture, rstest};

    #[fixture]
    fn empty_set() -> SparseUF {
        SparseUF::new()
    }

    /// uf with disjoin set of values from 0 to 99
    #[fixture]
    fn set_100_orphans(mut empty_set: SparseUF) -> SparseUF {
        for i in 0..100 {
            empty_set.insert(i);
        }

        empty_set
    }

    #[fixture]
    fn set_1to5_linear(mut empty_set: SparseUF) -> SparseUF {
        for i in 1..5 {
            empty_set.join(i, i + 1);
        }

        empty_set
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
    fn set_10by10(set_100_orphans: SparseUF) -> SparseUF {
        let mut set = set_100_orphans;

        for component_idx in 0..10 {
            let from = (component_idx * 10) as usize;
            let to = ((component_idx + 1) * 10) as usize;
            let nodes = rand_permutation(from, to);
            let links = chain_nodes(&nodes);
            let links = randomize_links(links);

            for (a, b) in links {
                set.join(a, b);
            }
        }

        set
    }

    /////////////////////////////////////

    #[rstest]
    fn inserted_items_are_have_unique_ids(mut set_100_orphans: SparseUF) {
        for i in 0..100 {
            for j in (i + 1)..100 {
                assert_ne!(
                    set_100_orphans.cluster_id_of(i),
                    set_100_orphans.cluster_id_of(j)
                );
            }
        }
    }

    #[rstest]
    fn items_in_empty_set_are_disconnected(mut empty_set: SparseUF) {
        for i in 0..10 {
            for j in (i + 1)..10 {
                assert_returns!(false, SparseUF::connected, &mut empty_set, i, j);
            }
        }
    }

    #[rstest]
    fn inserted_items_are_disconnected(mut set_100_orphans: SparseUF) {
        for i in 0..100 {
            for j in (i + 1)..100 {
                assert_returns!(false, SparseUF::connected, &mut set_100_orphans, i, j);
            }
        }
        assert_returns!(100, SparseUF::clusters_count, &mut set_100_orphans);
    }

    #[rstest]
    fn components_count_is_correct(mut set_10by10: SparseUF) {
        assert_returns!(10, SparseUF::clusters_count, &mut set_10by10);
    }

    #[rstest]
    fn components_content_is_correct(mut set_10by10: SparseUF) {
        let mut components: Vec<Cluster> = set_10by10.clusters();
        components.sort_by_key(|cluster| cluster.id);

        println!("components: {components:#?}");

        for i in 0..10 {
            let expected_content: Vec<usize> = ((i * 10)..((i + 1) * 10)).collect();
            assert_that(&components[i as usize].nodes).contains_all_of(&expected_content.iter());
        }
    }

    #[rstest]
    fn joining_n_components_makes_single_component(mut set_10by10: SparseUF) {
        let mut rng = thread_rng();

        let some_node_for_each_component: Vec<usize> = (0..10)
            .map(|comp_id| comp_id * 10 + rng.gen_range(0..10))
            .collect();

        let intercomponent_links = chain_nodes(&some_node_for_each_component);
        let intercomponent_links = randomize_links(intercomponent_links);

        for (from, to) in intercomponent_links {
            set_10by10.join(from, to);
        }

        assert_returns!(1, SparseUF::clusters_count, &mut set_10by10);
    }

    #[rstest]
    fn disjoint_set_gives_unit_clusters() {
        let mut uf = SparseUF::new();
        for i in 0..10 {
            uf.insert(i);
        }

        let clusters = uf.clusters();
        assert_eq!(clusters.len(), 10);
        for c in clusters {
            assert_eq!(c.nodes.len(), 1);
        }
    }

    #[rstest]
    fn joining_items_makes_their_ids_equal(mut set_100_orphans: SparseUF) {
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
    fn item_has_id_of_its_terminal_link(mut set_1to5_linear: SparseUF) {
        for i in 1..=5 {
            assert_returns!(Some(1), SparseUF::cluster_id_of, &mut set_1to5_linear, i);
        }
    }

    #[rstest]
    fn components_are_valid_manual() {
        let mut set = SparseUF::new();

        set.join(1, 2);
        assert_returns!(1, SparseUF::clusters_count, &mut set);
        set.join(3, 4);
        assert_returns!(2, SparseUF::clusters_count, &mut set);

        set.join(5, 6);
        assert_returns!(3, SparseUF::clusters_count, &mut set);
        set.join(7, 8);
        assert_returns!(4, SparseUF::clusters_count, &mut set);

        set.join(1, 4);
        assert_returns!(3, SparseUF::clusters_count, &mut set);
        set.join(7, 6);
        assert_returns!(2, SparseUF::clusters_count, &mut set);

        set.join(6, 4);
        assert_returns!(1, SparseUF::clusters_count, &mut set);

        let component_items = &set.clusters()[0].nodes;

        let all_items: Vec<usize> = (1..=8).collect();
        assert_that(component_items).contains_all_of(&all_items.iter());
    }

    proptest! {
        #[test]
        fn new_set_contains_nothing(num in 0..1000) {
            let set = SparseUF::new();

            prop_assert!(!set.contains(num as usize));
        }

        #[test]
        fn id_of_returns_none_if_item_isnt_inserted(num in 0..1000) {
            let mut set = SparseUF::new();

            prop_assert!(set.cluster_id_of(num as usize).is_none());
        }
    }
}
