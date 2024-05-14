#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    fn new_link(val: T, next: Option<Box<Self>>) -> Option<Box<Self>> {
        Some(Box::new(ListNode { val, next }))
    }

    #[inline]
    pub fn from_vec(values: Vec<T>) -> Option<Box<Self>> {
        vec_to_list(values)
    }
}

pub fn vec_to_list<T>(mut values: Vec<T>) -> Option<Box<ListNode<T>>> {
    let mut head = None;

    while !values.is_empty() {
        let last_val = values.pop().unwrap();
        head = ListNode::new_link(last_val, head)
    }
    head
}

pub fn list_to_vec<T>(list: Option<Box<ListNode<T>>>) -> Vec<T> {
    let mut result = vec![];

    let mut head = list;
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }

    result
}

pub trait IntoVec<T> {
    fn into_vec(self) -> Vec<T>;
}

impl<T> IntoVec<T> for Option<Box<ListNode<T>>> {
    fn into_vec(self) -> Vec<T> {
        list_to_vec(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::linked_list::{list_to_vec, vec_to_list};
    use proptest::{prop_assert_eq, proptest};

    proptest! {
        #[test]
        fn parse_dump(input in proptest::collection::vec(-100..100, 0..5)) {
            let parsed = vec_to_list::<i32>(input.clone());
            let dumped = list_to_vec(parsed);

            prop_assert_eq!(input, dumped);
        }
    }
}
