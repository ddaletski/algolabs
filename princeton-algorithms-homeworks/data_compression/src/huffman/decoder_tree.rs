pub enum Node {
    Leaf { value: u8 },
    Inner { left: Box<Node>, right: Box<Node> },
}

pub fn decoding_tree<'a>(data: &mut impl Iterator<Item = &'a u8>) -> Node {
    fn load_rec<'a>(data: &mut impl Iterator<Item = &'a u8>) -> Box<Node> {
        let is_leaf = *data.next().unwrap() != 0;

        if is_leaf {
            Box::new(Node::Leaf {
                value: *data.next().unwrap(),
            })
        } else {
            let left = load_rec(data);
            let right = load_rec(data);
            Box::new(Node::Inner { left, right })
        }
    }

    let tree = load_rec(data);

    *tree
}
