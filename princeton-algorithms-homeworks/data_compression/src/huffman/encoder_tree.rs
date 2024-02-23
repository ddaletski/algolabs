use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use bitvec::{order::Lsb0, vec::BitVec};

pub enum Node {
    Leaf {
        value: u8,
    },
    Inner {
        left: Box<WeightedNode>,
        right: Box<WeightedNode>,
    },
}

pub struct WeightedNode {
    pub weight: u32,
    pub data: Node,
}

impl PartialEq for WeightedNode {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for WeightedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Eq for WeightedNode {}

impl Ord for WeightedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

pub fn encoding_tree(freq_map: &HashMap<u8, u32>) -> WeightedNode {
    let mut sorted_forest: BinaryHeap<_> = freq_map
        .iter()
        .map(|(&byte, &freq)| {
            Reverse(WeightedNode {
                weight: freq,
                data: Node::Leaf { value: byte },
            })
        })
        .collect();

    while sorted_forest.len() > 1 {
        let right = sorted_forest.pop().unwrap().0;
        let left = sorted_forest.pop().unwrap().0;

        let weight = left.weight + right.weight;
        let merged = WeightedNode {
            weight,
            data: Node::Inner {
                left: Box::new(left),
                right: Box::new(right),
            },
        };

        sorted_forest.push(Reverse(merged));
    }

    sorted_forest.pop().unwrap().0
}

pub fn encoding_map(tree: &Node) -> HashMap<u8, BitVec<u8, Lsb0>> {
    let mut map = HashMap::new();

    fn traverse_dfs(
        node: &Node,
        current_bits: &mut BitVec<u8, Lsb0>,
        map: &mut HashMap<u8, BitVec<u8, Lsb0>>,
    ) {
        match node {
            Node::Leaf { value } => {
                map.insert(*value, current_bits.clone());
            }
            Node::Inner { left, right } => {
                // left branch -> 0
                current_bits.push(false);
                traverse_dfs(&left.data, current_bits, map);
                current_bits.pop();

                // right branch -> 1
                current_bits.push(true);
                traverse_dfs(&right.data, current_bits, map);
                current_bits.pop();
            }
        }
    }

    traverse_dfs(tree, &mut BitVec::new(), &mut map);

    map
}

pub fn dump_tree(tree: &Node, buffer: &mut Vec<u8>) {
    match tree {
        Node::Leaf { value } => {
            // is_leaf
            buffer.push(true as u8);
            buffer.push(*value);
        }
        Node::Inner { left, right } => {
            // !is_leaf
            buffer.push(false as u8);
            dump_tree(&left.data, buffer);
            dump_tree(&right.data, buffer);
        }
    }
}
