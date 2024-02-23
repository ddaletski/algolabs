use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::disjoint_sets_union::{HashMapDSU, UnionFind};

type VertexID = usize;

#[derive(Debug, PartialEq, Eq)]
pub struct Vertex<T> {
    pub id: VertexID,
    pub value: T,
}

impl<T> Vertex<T> {
    pub fn new(id: VertexID, value: T) -> Self {
        Vertex { id, value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge<Weight> {
    pub from: VertexID,
    pub to: VertexID,
    pub weight: Weight,
}

impl<Weight> Edge<Weight> {
    pub fn new(from: VertexID, to: VertexID, weight: Weight) -> Self {
        Edge { from, to, weight }
    }
}

pub struct WeightedGraph<T, Weight = i32> {
    vertices: HashMap<VertexID, T>,
    adjacencies: HashMap<VertexID, HashMap<VertexID, Weight>>,
}

impl<T, Weight: Copy> WeightedGraph<T, Weight> {
    /// create a new empty graph
    pub fn new() -> Self {
        WeightedGraph {
            vertices: HashMap::new(),
            adjacencies: HashMap::new(),
        }
    }

    /// add a new vertex to the graph
    pub fn add_vertex(&mut self, vertex: Vertex<T>) {
        self.vertices.insert(vertex.id, vertex.value);
    }

    /// insert a new vertex into the graph
    pub fn insert(&mut self, id: VertexID, value: T) {
        self.add_vertex(Vertex { id, value })
    }

    /// check if the graph contains a vertex with a given id
    pub fn contains(&self, id: VertexID) -> bool {
        self.vertices.contains_key(&id)
    }

    /// add an edge to the graph
    pub fn add_edge(&mut self, edge: Edge<Weight>) {
        assert!(
            self.contains(edge.from),
            "can't connect non-existent vertex"
        );
        assert!(self.contains(edge.to), "can't connect non-existent vertex");

        let id1 = edge.from;
        let id2 = edge.to;
        let weight = edge.weight;

        self.adjacencies.entry(id1).or_default().insert(id2, weight);
        self.adjacencies.entry(id2).or_default().insert(id1, weight);
    }

    /// connect two vertices with an edge of a given weight
    pub fn connect(&mut self, from: VertexID, to: VertexID, weight: Weight) {
        self.add_edge(Edge { from, to, weight })
    }

    /// check if two vertices are connected
    pub fn connected(&self, this: VertexID, that: VertexID) -> bool {
        let Some(links) = self.adjacencies.get(&this) else {
            return false;
        };

        links.contains_key(&that)
    }

    /// number of vertices in the graph
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// check if the graph is empty
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// number of edges in the graph
    pub fn edges_count(&self) -> usize {
        self.adjacencies
            .values()
            .map(|links| links.len())
            .sum::<usize>()
            / 2
    }

    /// iterator over all vertices in the graph
    pub fn vertices(&self) -> impl Iterator<Item = Vertex<&T>> {
        self.vertices
            .iter()
            .map(|(k, v)| Vertex { id: *k, value: v })
    }

    /// iterator over all edges in the graph
    pub fn edges(&self) -> impl Iterator<Item = Edge<Weight>> + '_ {
        self.adjacencies.iter().flat_map(|(from, links)| {
            links.iter().filter_map(|(to, weight)| {
                if *from < *to {
                    Some(Edge {
                        from: *from,
                        to: *to,
                        weight: *weight,
                    })
                } else {
                    None
                }
            })
        })
    }

    /// iterator over all vertices which are adjacent to the vertex with a given id
    pub fn adjacent_vertices(&self, id: VertexID) -> impl Iterator<Item = Vertex<&T>> + '_ {
        let iterator = self.adjacent_edges(id).filter_map(|edge| {
            self.vertices
                .get(&edge.to)
                .map(|value| Vertex { id: edge.to, value })
        });

        iterator
    }

    // iterator over all edges which are adjacent to the vertex with a given id
    pub fn adjacent_edges(&self, id: VertexID) -> Box<dyn Iterator<Item = Edge<Weight>> + '_> {
        let Some(links) = self.adjacencies.get(&id) else {
            return Box::new(std::iter::empty());
        };

        let iterator = links.iter().map(move |(&other_id, &weight)| Edge {
            from: id,
            to: other_id,
            weight,
        });

        Box::new(iterator)
    }

    /// Compute minimum spanning tree using Kruskal's algorithm
    pub fn mst_kruskal(&self) -> Vec<Edge<Weight>>
    where
        Weight: Ord,
    {
        let mut result = vec![];
        let mut dsu = HashMapDSU::new();

        let edges: BinaryHeap<_> = self.edges().map(|it| std::cmp::Reverse(it)).collect();

        for edge in edges.into_iter().map(|rev| rev.0) {
            if result.len() == self.len() - 1 {
                break;
            }

            if dsu.connected(edge.from, edge.to) {
                continue;
            }

            dsu.join(edge.from, edge.to);
            result.push(edge);
        }

        result
    }

    /// Compute minimum spanning tree using Prim's algorithm
    pub fn mst_prim(&self) -> Vec<Edge<Weight>>
    where
        Weight: Ord,
    {
        let mut result = vec![];

        let mut visited_vertices = HashSet::new();

        let first_vertex = self.vertices().next().unwrap();
        visited_vertices.insert(first_vertex.id);

        let mut edges: BinaryHeap<_> = self
            .adjacent_edges(first_vertex.id)
            .map(|it| std::cmp::Reverse(it))
            .collect();

        while !edges.is_empty() {
            let edge = edges.pop().unwrap().0;
            if visited_vertices.contains(&edge.to) {
                continue;
            }

            for edge in self.adjacent_edges(edge.to) {
                edges.push(std::cmp::Reverse(edge));
            }

            visited_vertices.insert(edge.to);
            result.push(edge);
        }

        result
    }
}

impl<Weight: PartialOrd> PartialOrd for Edge<Weight> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<Weight: Ord> Ord for Edge<Weight> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let mut graph: WeightedGraph<_, i32> = WeightedGraph::new();
        assert_eq!(graph.len(), 0);

        graph.insert(0, "A");
        assert_eq!(graph.len(), 1);

        graph.insert(1, "B");
        assert_eq!(graph.len(), 2);
    }

    #[test]
    fn test_contains() {
        let mut graph: WeightedGraph<_, i32> = WeightedGraph::new();
        graph.insert(5, "A");
        graph.insert(2, "B");

        assert!(graph.contains(5));
        assert!(graph.contains(2));
        assert!(!graph.contains(0));
        assert!(!graph.contains(1));
        assert!(!graph.contains(8));
    }

    #[test]
    fn test_connected() {
        let mut graph = WeightedGraph::new();
        graph.insert(0, "A");
        graph.insert(1, "B");
        graph.add_edge(Edge::new(0, 1, 1.0));

        assert!(graph.connected(0, 1));
        assert!(graph.connected(1, 0));
        assert!(!graph.connected(0, 2));
    }

    #[test]
    fn test_vertices() {
        let mut graph: WeightedGraph<_, i32> = WeightedGraph::new();
        graph.insert(0, "A");
        graph.insert(1, "B");
        graph.insert(2, "C");

        let vertices: Vec<_> = graph.vertices().collect();
        assert_eq!(vertices.len(), 3);
        assert!(vertices.contains(&Vertex::new(0, &"A")));
        assert!(vertices.contains(&Vertex::new(1, &"B")));
        assert!(vertices.contains(&Vertex::new(2, &"C")));
    }

    #[test]
    fn test_edges() {
        let mut graph = WeightedGraph::new();
        graph.insert(0, "A");
        graph.insert(1, "B");
        graph.insert(2, "C");
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(1, 2, 2.0));

        assert_eq!(graph.edges_count(), 2);

        let edges: Vec<_> = graph.edges().collect();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&Edge::new(0, 1, 1.0)));
        assert!(edges.contains(&Edge::new(1, 2, 2.0)));
    }

    #[test]
    fn test_adjacent_vertices() {
        let mut graph = WeightedGraph::new();
        graph.insert(0, "A");
        graph.insert(1, "B");
        graph.insert(2, "C");
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(1, 2, 2.0));

        let vertices: Vec<_> = graph.adjacent_vertices(1).collect();
        assert_eq!(vertices.len(), 2);
        assert!(vertices.contains(&Vertex::new(0, &"A")));
        assert!(vertices.contains(&Vertex::new(2, &"C")));
    }

    #[test]
    fn test_adjacent_edges() {
        let mut graph = WeightedGraph::new();
        graph.insert(0, "A");
        graph.insert(1, "B");
        graph.insert(2, "C");
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(1, 2, 2.0));

        let edges: Vec<_> = graph.adjacent_edges(1).collect();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&Edge::new(1, 0, 1.0)));
        assert!(edges.contains(&Edge::new(1, 2, 2.0)));
    }

    #[rstest::fixture]
    fn graph_for_mst() -> WeightedGraph<&'static str> {
        let mut graph = WeightedGraph::new();
        graph.insert(0, "A");
        graph.insert(1, "B");
        graph.insert(2, "C");
        graph.insert(3, "D");
        graph.add_edge(Edge::new(0, 1, 1));
        graph.add_edge(Edge::new(0, 2, 2));
        graph.add_edge(Edge::new(0, 3, 3));
        graph.add_edge(Edge::new(1, 2, 4));
        graph.add_edge(Edge::new(1, 3, 5));
        graph.add_edge(Edge::new(2, 3, 6));

        graph
    }

    #[rstest::rstest]
    fn test_mst_kruskal(graph_for_mst: WeightedGraph<&'static str>) {
        let mst = graph_for_mst.mst_kruskal();
        assert_eq!(mst.len(), 3);
        assert!(mst.contains(&Edge::new(0, 1, 1)));
        assert!(mst.contains(&Edge::new(0, 2, 2)));
        assert!(mst.contains(&Edge::new(0, 3, 3)));
    }

    #[rstest::rstest]
    fn test_mst_prim(graph_for_mst: WeightedGraph<&'static str>) {
        let mst = graph_for_mst.mst_prim();
        assert_eq!(mst.len(), 3);
        assert!(mst.contains(&Edge::new(0, 1, 1)));
        assert!(mst.contains(&Edge::new(0, 2, 2)));
        assert!(mst.contains(&Edge::new(0, 3, 3)));
    }
}
