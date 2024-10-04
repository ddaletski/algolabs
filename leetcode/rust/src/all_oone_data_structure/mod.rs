use algo_toolbox::ordered_counter::OrderedCounter;

struct AllOne {
    counter: OrderedCounter<String>,
}

impl AllOne {
    fn new() -> Self {
        Self {
            counter: OrderedCounter::new(),
        }
    }

    fn inc(&mut self, key: String) {
        self.counter.inc(&key);
    }

    fn dec(&mut self, key: String) {
        self.counter.dec(&key);
    }

    fn get_max_key(&self) -> String {
        self.counter.max().cloned().unwrap_or("".into())
    }

    fn get_min_key(&self) -> String {
        self.counter.min().cloned().unwrap_or("".into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // cmd: ["inc","inc","getMaxKey","getMinKey","inc","getMaxKey","getMinKey"]
    // arg: [["hello"],["hello"],[],[],["leet"],[],[]]

    #[test]
    fn case1() {
        let mut obj = AllOne::new();
        obj.inc("hello".to_string());
        obj.inc("hello".to_string());
        assert_eq!(obj.get_max_key(), "hello".to_string());
        assert_eq!(obj.get_min_key(), "hello".to_string());

        obj.inc("leet".to_string());
        assert_eq!(obj.get_max_key(), "hello".to_string());
        assert_eq!(obj.get_min_key(), "leet".to_string());
    }
}