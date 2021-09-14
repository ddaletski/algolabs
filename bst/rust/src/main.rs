use std::borrow::{Borrow};
use rand::{Rng};

#[derive(Debug)]
pub struct BSTNode<T> {
    val: T,
    left: Option<Box<BSTNode<T>>>,
    right: Option<Box<BSTNode<T>>>
}

impl<T> BSTNode<T> where T : Ord {
    fn new(value: T) -> BSTNode<T> {
        BSTNode{val: value, left: None, right: None}
    }

    fn insert(node: Option<Box<BSTNode<T>>>, value: T) -> BSTNode<T> {
        if let Some(node) = node {
            if value < node.val {
                let inserted = Some(Box::new(BSTNode::insert(node.left, value)));
                BSTNode{val: node.val, left: inserted, right: node.right}
            } else if value > node.val {
                let inserted = Some(Box::new(BSTNode::insert(node.right, value)));
                BSTNode{val: node.val, left: node.left, right: inserted}
            } else {
                BSTNode{val: node.val, left: node.left, right: node.right}
            }
        } else {
            BSTNode::new(value)
        }
    }

    fn find(node: &Option<Box<BSTNode<T>>>, value: T) -> &Option<Box<BSTNode<T>>> {
        if let Some(found) = node {
            if value < found.val {
                BSTNode::find(&found.left, value)
            } else if value > found.val {
                BSTNode::find(&found.right, value)
            } else {
                node
            }
        } else {
            node
        }
    }

}

impl<T> BSTNode<T> where T : Copy {
    fn clone(node: &Option<Box<BSTNode<T>>>) -> Option<Box<BSTNode<T>>> {
        if let Some(node) = node {
            Some(Box::new(BSTNode{
                val: node.val,
                left: BSTNode::clone(&node.left),
                right: BSTNode::clone(&node.right)
            }))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BST<T> {
    root: Option<Box<BSTNode<T>>>
}

impl<T> BST<T> where T: Ord {
    pub fn new() -> BST<T> {
        BST{root: None}
    }

    pub fn insert(self, value: T) -> BST<T> {
        BST{root: Some(Box::new(BSTNode::insert(self.root, value)))}
    }

    pub fn find(&self, value: T) -> Option<&BSTNode<T>> {
        if let Some(found) = BSTNode::find(&self.root, value) {
            Some(found.borrow())
        } else {
            None
        }
    }
}

impl<T> BST<T> where T: Ord + Copy {
    pub fn subtree_from(&self, value: T) -> BST<T> {
        let found = BSTNode::find(&self.root, value);
        let cloned_node = BSTNode::clone(found);

        BST{root: cloned_node}
    }
}

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();

    let random_values = (0..100).map(|_| rng.gen_range(0..100));

    let mut tree = BST::new();

    tree = tree.insert(100);
    for val in random_values {
        tree = tree.insert(val);
    }
    tree = tree.insert(150);

    let n1 = tree.find(100);
    let n2 = tree.find(150);
    let n3 = tree.find(200);

    println!("{:?}", n1);
    println!("{:?}", n2);
    println!("{:?}", n3);
}
