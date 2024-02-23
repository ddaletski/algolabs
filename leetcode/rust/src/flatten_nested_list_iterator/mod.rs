#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

///////////////////////////////////////////////

#[derive(Debug)]
pub struct NestedIterator {
    stack: Vec<NestedInteger>,
    next_item: Option<i32>,
}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let stack = nested_list
            .into_iter()
            .filter(|val| {
                if let NestedInteger::List(list) = val {
                    !list.is_empty()
                } else {
                    true
                }
            })
            .rev()
            .collect();

        let mut iter = NestedIterator {
            stack,
            next_item: None,
        };

        iter.fetch_next();

        iter
    }

    fn fetch_next(&mut self) {
        if let Some(next_item) = self.stack.pop() {
            match next_item {
                NestedInteger::Int(val) => self.next_item = Some(val),

                NestedInteger::List(list) => {
                    self.stack.extend(
                        list.into_iter()
                            .filter(|val| {
                                if let NestedInteger::List(list) = val {
                                    !list.is_empty()
                                } else {
                                    true
                                }
                            })
                            .rev(),
                    );

                    self.fetch_next();
                }
            }
        } else {
            self.next_item = None;
        }
    }

    pub fn next(&mut self) -> i32 {
        let next_val = self.next_item.take().unwrap();
        self.fetch_next();

        next_val
    }

    pub fn has_next(&self) -> bool {
        self.next_item.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;
    use rstest::rstest;

    impl NestedIterator {
        fn collect(&mut self) -> Vec<i32> {
            let mut flattened = Vec::new();

            while self.has_next() {
                flattened.push(self.next())
            }

            flattened
        }
    }

    macro_rules! i {
        ($val:expr) => {
            NestedInteger::Int($val)
        };
    }

    macro_rules! list {
        ($list:expr) => {
            NestedInteger::List($list)
        };
    }

    #[rstest]
    #[case(vec![list!(vec![i!(1), i!(1)]), i!(2), list!(vec![i!(1), i!(1)])], vec![1, 1, 2, 1, 1])]
    #[case(vec![i!(1), list!(vec![i!(4), list!(vec![i!(6)])])], vec![1, 4, 6])]
    #[case(vec![], vec![])]
    #[case(vec![list!(vec![])], vec![])]
    #[case(vec![list!(vec![i!(1), list!(vec![])])], vec![1])]
    fn result_is_correct(#[case] input: Vec<NestedInteger>, #[case] expected_output: Vec<i32>) {
        let mut iterator = NestedIterator::new(input);
        assert_returns!(expected_output, NestedIterator::collect, &mut iterator);
    }
}
