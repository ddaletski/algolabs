use std::collections::{HashMap, HashSet};

pub trait UnionFind {
    fn join(&mut self, item1: usize, item2: usize) -> usize;
    fn connected(&mut self, item1: usize, item2: usize) -> bool;
}

#[derive(Debug, Default)]
pub struct HashMapDSU {
    mapping: HashMap<usize, usize>,
}

impl HashMapDSU {
    /// create a new empty set
    pub fn new() -> Self {
        HashMapDSU::default()
    }

    /// get the number of items in the set
    pub fn len(&self) -> usize {
        self.mapping.len()
    }

    /// check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    /// check if the set contains the item
    pub fn contains(&self, item: usize) -> bool {
        self.mapping.contains_key(&item)
    }

    /// get the id of the item's component if it's inserted
    /// otherwise return None
    pub fn component_id_of(&self, item: usize) -> Option<usize> {
        let mut curr_item = item;
        while let Some(&next_item) = self.mapping.get(&curr_item) {
            if next_item == curr_item {
                return Some(curr_item);
            }
            curr_item = next_item;
        }
        None
    }

    /// get the id of the item's component and optimize the whole ids chain while doing it
    fn get_id_and_optimize(&mut self, item: usize) -> Option<usize> {
        let chain_of_ids = |item: usize| {
            let mut chain = vec![];

            let mut curr_item = item;
            while let Some(&next_item) = self.mapping.get(&curr_item) {
                if next_item == curr_item {
                    chain.push(curr_item);
                    return chain;
                }
                chain.push(curr_item);
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

    /// insert the item if it isn't present
    /// return it's component id
    pub fn insert(&mut self, item: usize) -> usize {
        if let Some(id) = self.component_id_of(item) {
            id
        } else {
            self.mapping.insert(item, item);
            item
        }
    }

    /// get all connected components of the set
    pub fn components(&self) -> HashMap<usize, Vec<usize>> {
        self.mapping.keys().fold(HashMap::new(), |mut map, &item| {
            let id = self.component_id_of(item).unwrap();
            map.entry(id).or_insert(vec![]).push(item);
            map
        })
    }

    /// get the number of connected components in the set
    pub fn components_count(&self) -> usize {
        self.mapping
            .keys()
            .map(|&k| self.component_id_of(k).unwrap())
            .fold(HashSet::new(), |mut set, item| {
                set.insert(item);
                set
            })
            .len()
    }
}

impl UnionFind for HashMapDSU {
    /// join two items and return id of the connected component they are in after joining
    /// if some (or both) of the items wasn't present, it's inserted before joining
    fn join(&mut self, item1: usize, item2: usize) -> usize {
        match (
            self.get_id_and_optimize(item1),
            self.get_id_and_optimize(item2),
        ) {
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
        let id1 = self.get_id_and_optimize(item1);
        let id2 = self.get_id_and_optimize(item2);

        id1 == id2 && id1.is_some()
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
    fn empty_set() -> HashMapDSU {
        HashMapDSU::new()
    }

    #[fixture]
    fn set_100_orphans(mut empty_set: HashMapDSU) -> HashMapDSU {
        for i in 0..100 {
            empty_set.insert(i);
        }

        empty_set
    }

    #[fixture]
    fn set_1to5_linear(mut empty_set: HashMapDSU) -> HashMapDSU {
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
    fn set_10by10(set_100_orphans: HashMapDSU) -> HashMapDSU {
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
    fn inserting_new_item_increments_id(empty_set: HashMapDSU) {
        let mut set = empty_set;

        for i in 0..1000 {
            assert_returns!(i as usize, HashMapDSU::insert, &mut set, i);
        }
    }

    #[rstest]
    fn inserted_items_are_have_unique_ids(set_100_orphans: HashMapDSU) {
        for i in 0..100 {
            for j in (i + 1)..100 {
                assert_ne!(set_100_orphans.component_id_of(i), set_100_orphans.component_id_of(j));
            }
        }
    }

    #[rstest]
    fn items_in_empty_set_are_disconnected(mut empty_set: HashMapDSU) {
        for i in 0..10 {
            for j in (i + 1)..10 {
                assert_returns!(
                    false,
                    HashMapDSU::connected,
                    &mut empty_set,
                    i,
                    j
                );
            }
        }
    }

    #[rstest]
    fn inserted_items_are_disconnected(mut set_100_orphans: HashMapDSU) {
        for i in 0..100 {
            for j in (i + 1)..100 {
                assert_returns!(
                    false,
                    HashMapDSU::connected,
                    &mut set_100_orphans,
                    i,
                    j
                );
            }
        }
        assert_returns!(100, HashMapDSU::components_count, &set_100_orphans);
    }

    #[rstest]
    fn components_count_is_correct(set_10by10: HashMapDSU) {
        assert_returns!(10, HashMapDSU::components_count, &set_10by10);
    }

    #[rstest]
    fn components_content_is_correct(set_10by10: HashMapDSU) {
        let mut components: Vec<Vec<usize>> = set_10by10.components().values().cloned().collect();
        components.sort_by_key(|values| values[0]);

        for i in 0..10 {
            let expected_content: Vec<usize> = ((i * 10)..((i + 1) * 10)).collect();
            assert_that(&components[i as usize]).contains_all_of(&expected_content.iter());
        }
        assert_returns!(10, HashMapDSU::components_count, &set_10by10);
    }

    #[rstest]
    fn joining_n_components_makes_single_component(mut set_10by10: HashMapDSU) {
        let mut rng = thread_rng();

        let some_node_for_each_component: Vec<usize> = (0..10)
            .map(|comp_id| comp_id * 10 + rng.gen_range(0..10))
            .collect();

        let intercomponent_links = chain_nodes(&some_node_for_each_component);
        let intercomponent_links = randomize_links(intercomponent_links);

        for (from, to) in intercomponent_links {
            set_10by10.join(from, to);
        }

        assert_returns!(1, HashMapDSU::components_count, &set_10by10);
    }

    #[rstest]
    fn joining_items_makes_their_ids_equal(mut set_100_orphans: HashMapDSU) {
        let mut rng = thread_rng();
        let id_distr1 = rand::distributions::Uniform::from(0..100);
        let id_distr2 = rand::distributions::Uniform::from(0..100);

        for _ in 0..1000 {
            let item1 = id_distr1.sample(&mut rng);
            let item2 = id_distr2.sample(&mut rng);

            set_100_orphans.join(item1, item2);
            assert_eq!(set_100_orphans.component_id_of(item1), set_100_orphans.component_id_of(item2));
        }
    }

    #[rstest]
    fn item_has_id_of_its_terminal_link(set_1to5_linear: HashMapDSU) {
        for i in 1..=5 {
            assert_returns!(Some(1), HashMapDSU::component_id_of, &set_1to5_linear, i);
        }
    }

    #[rstest]
    fn components_are_valid_manual() {
        let mut set = HashMapDSU::new();

        set.join(1, 2);
        assert_returns!(1, HashMapDSU::components_count, &set);
        set.join(3, 4);
        assert_returns!(2, HashMapDSU::components_count, &set);

        set.join(5, 6);
        assert_returns!(3, HashMapDSU::components_count, &set);
        set.join(7, 8);
        assert_returns!(4, HashMapDSU::components_count, &set);

        set.join(1, 4);
        assert_returns!(3, HashMapDSU::components_count, &set);
        set.join(7, 6);
        assert_returns!(2, HashMapDSU::components_count, &set);

        set.join(6, 4);
        assert_returns!(1, HashMapDSU::components_count, &set);

        let component = set.components().values().next().cloned().unwrap();

        let all_items: Vec<usize> = (1..=8).collect();
        assert_that(&component).contains_all_of(&all_items.iter());
    }

    proptest! {
        #[test]
        fn new_set_contains_nothing(num in 0..1000) {
            let set = HashMapDSU::new();

            prop_assert!(!set.contains(num as usize));
        }

        #[test]
        fn id_of_returns_none_if_item_isnt_inserted(num in 0..1000) {
            let set = HashMapDSU::new();

            prop_assert!(set.component_id_of(num as usize).is_none());
        }
    }
}
