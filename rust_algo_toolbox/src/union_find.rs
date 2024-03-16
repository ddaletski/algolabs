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
}
