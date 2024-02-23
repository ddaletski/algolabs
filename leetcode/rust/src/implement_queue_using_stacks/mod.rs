#[derive(Debug)]
struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            stack1: vec![],
            stack2: vec![],
        }
    }

    pub fn push(&mut self, x: i32) {
        self.switch_to_push();
        self.stack1.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.switch_to_pop();
        self.stack2.pop().unwrap_or(0)
    }

    pub fn peek(&mut self) -> i32 {
        self.switch_to_pop();
        let top = self.stack2.get(self.stack2.len() - 1).cloned();
        top.unwrap_or(0)
    }

    pub fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }

    fn switch_to_push(&mut self) {
        while let Some(value) = self.stack2.pop() {
            self.stack1.push(value);
        }
    }

    fn switch_to_pop(&mut self) {
        while let Some(value) = self.stack1.pop() {
            self.stack2.push(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_returns;

    #[test]
    fn case1() {
        let mut queue = MyQueue::new();
        assert_returns!(true, MyQueue::empty, &queue);

        queue.push(1);
        assert_returns!(false, MyQueue::empty, &queue);

        queue.push(2);
        assert_returns!(false, MyQueue::empty, &queue);

        assert_returns!(1, MyQueue::peek, &mut queue);
        assert_returns!(false, MyQueue::empty, &queue);

        assert_returns!(1, MyQueue::pop, &mut queue);
        assert_returns!(false, MyQueue::empty, &queue);

        assert_returns!(2, MyQueue::pop, &mut queue);
        assert_returns!(true, MyQueue::empty, &queue);
    }
}
