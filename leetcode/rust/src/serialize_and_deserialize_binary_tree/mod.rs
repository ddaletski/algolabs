use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::common::binary_tree::TreeNode<i32>;

/////////////////////////////////////////////////////////

type NodeLink = Option<Rc<RefCell<TreeNode>>>;

pub fn new_link(val: i32, left: NodeLink, right: NodeLink) -> NodeLink {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let result_list = self.to_vec(root);

        format!("[{}]", result_list.join(", "))
    }

    pub fn to_vec(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result_list: Vec<String> = Vec::new();

        match root {
            Some(node_ptr) => {
                let node = node_ptr.borrow();
                result_list.push(format!("{}", node.val));
                result_list.append(&mut self.to_vec(node.left.clone()));
                result_list.append(&mut self.to_vec(node.right.clone()));
            }
            None => {
                result_list.push("null".into());
            }
        }
        result_list
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let values_list = data[1..data.len() - 1].split(", ").map(|s| s.to_owned());

        self.from_iter(values_list)
    }

    pub fn from_iter<IterType: Iterator<Item = String>>(
        &self,
        mut values: IterType,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        self.from_iter_impl(&mut values)
    }

    fn from_iter_impl<IterType: Iterator<Item = String>>(
        &self,
        values: &mut IterType,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root_str = values
            .next()
            .expect("seems like the list representation of tree is invalid");

        match root_str.as_str() {
            "null" => return None,
            _ => {
                let val = i32::from_str_radix(&root_str, 10).unwrap();
                let left = self.from_iter_impl(values);
                let right = self.from_iter_impl(values);

                return new_link(val, left, right);
            }
        }
    }
}

/////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    type NodeLink = crate::common::binary_tree::NodeLink<i32>;

    #[fixture]
    fn codec() -> Codec {
        Codec::new()
    }

    #[fixture]
    fn expected_tree() -> NodeLink {
        TreeNode::new_link(
            5, // root
            TreeNode::new_link(
                1, // left
                None,
                TreeNode::new_link(
                    3, // left.right,
                    None, None,
                ),
            ),
            TreeNode::new_link(
                9, // right
                TreeNode::new_link(
                    7, // right.left
                    None, None,
                ),
                TreeNode::new_link(
                    15, // right.right
                    None,
                    TreeNode::new_link(
                        20, // right.right.right
                        None, None,
                    ),
                ),
            ),
        )
    }

    #[fixture]
    fn expected_string() -> String {
        "[5, 1, null, 3, null, null, 9, 7, null, null, 15, null, 20, null, null]".into()
    }

    #[rstest]
    fn serialization_works(codec: Codec, expected_string: String, expected_tree: NodeLink) {
        let serialized = codec.serialize(expected_tree);
        assert_eq!(serialized, expected_string);
    }

    #[rstest]
    fn deserialization_works(codec: Codec, expected_string: String, expected_tree: NodeLink) {
        let deserialized = codec.deserialize(expected_string);
        assert_eq!(deserialized, expected_tree);
    }
}
