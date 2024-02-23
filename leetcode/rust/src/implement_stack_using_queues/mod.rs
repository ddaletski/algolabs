use std::collections::VecDeque;

#[derive(Debug)]
pub struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.queue.is_empty() {
            return 0;
        }

        let replace_count = self.queue.len() - 1;

        for _ in 0..replace_count {
            let value = self.queue.pop_front().unwrap();
            self.queue.push_back(value);
        }
        self.queue.pop_front().unwrap()
    }

    pub fn top(&mut self) -> i32 {
        let top = self.pop();
        self.queue.push_back(top);
        top
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;

    #[test]
    fn case1() {
        let mut stack = MyStack::new();
        assert_returns!(true, MyStack::empty, &stack);

        stack.push(1);
        assert_returns!(false, MyStack::empty, &stack);

        stack.push(2);
        assert_returns!(false, MyStack::empty, &stack);

        assert_returns!(2, MyStack::top, &mut stack);
        assert_returns!(false, MyStack::empty, &stack);

        assert_returns!(2, MyStack::pop, &mut stack);
        assert_returns!(false, MyStack::empty, &stack);

        assert_returns!(1, MyStack::pop, &mut stack);
        assert_returns!(true, MyStack::empty, &stack);
    }
}
