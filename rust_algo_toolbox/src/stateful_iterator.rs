use std::marker::PhantomData;

pub struct StatefulIterator<'a, T, State> {
    transform: Box<dyn Fn(State) -> Option<(T, State)> + 'a>,
    last_state: Option<State>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T, State> StatefulIterator<'a, T, State> {
    pub fn new(initial_state: State, transform: impl Fn(State) -> Option<(T, State)> + 'a) -> Self {
        Self {
            transform: Box::new(transform),
            last_state: Some(initial_state),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, State> Iterator for StatefulIterator<'a, T, State> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let last_state = self.last_state.take().unwrap();

        let Some((next_item, next_state)) = (self.transform)(last_state) else {
            return None;
        };

        self.last_state = Some(next_state);

        Some(next_item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut actual = StatefulIterator::<i32, ()>::new((), |_| None);
        assert!(actual.next().is_none());
    }

    #[test]
    fn simple_list() {
        let mut actual = StatefulIterator::new(vec![1, 2, 3], |mut list| {
            if list.is_empty() {
                None
            } else {
                let next = list.pop().unwrap();
                Some((next, list))
            }
        });

        assert_eq!(actual.next(), Some(3));
        assert_eq!(actual.next(), Some(2));
        assert_eq!(actual.next(), Some(1));
        assert!(actual.next().is_none());
    }
}
