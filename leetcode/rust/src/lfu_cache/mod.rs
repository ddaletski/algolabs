use std::collections::{BinaryHeap, HashMap};

type Key = i32;
type Value = i32;
type Freq = usize;

#[derive(Debug, PartialEq, Eq)]
struct PriorityEntry {
    freq: Freq,
    created_at: usize,
    key: Key,
}

impl Ord for PriorityEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.freq, other.created_at).cmp(&(self.freq, self.created_at))
    }
}

impl PartialOrd for PriorityEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default)]
struct LFUCache {
    freqs: HashMap<Key, Freq>,
    updated_at: HashMap<Key, usize>,
    priorities: BinaryHeap<PriorityEntry>,
    kv_store: HashMap<Key, Value>,
    size: usize,
    capacity: usize,
    timestamp: usize,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            ..Default::default()
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.kv_store.contains_key(&key) {
            return -1;
        }

        self.bump_priority(key);
        *self.kv_store.get(&key).unwrap()
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(val) = self.kv_store.get_mut(&key) {
            *val = value;
            self.bump_priority(key);
            return;
        }

        if self.size == self.capacity {
            while let Some(PriorityEntry {
                key: to_remove,
                created_at,
                ..
            }) = self.priorities.pop()
            {
                // just pop all outdated entries
                if self.updated_at[&to_remove] != created_at {
                    continue;
                }

                // remove the lfu entry
                self.kv_store.remove(&to_remove);
                self.freqs.remove(&to_remove);
                self.updated_at.remove(&to_remove);
                self.size -= 1;
                break;
            }
        }

        self.size += 1;
        self.kv_store.insert(key, value);
        self.bump_priority(key);
    }

    fn bump_priority(&mut self, key: Key) -> usize {
        let freq = *self.freqs.entry(key).and_modify(|x| *x += 1).or_insert(1);
        self.updated_at.insert(key, self.timestamp);

        self.priorities.push(PriorityEntry {
            freq,
            created_at: self.timestamp,
            key,
        });

        self.timestamp += 1;

        freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handle_command(cache: &mut LFUCache, command: &str, args: Vec<i32>) -> Option<i32> {
        match command {
            "put" => {
                cache.put(args[0], args[1]);
                None
            }
            "get" => Some(cache.get(args[0])),
            _ => panic!("Unknown command: {}", command),
        }
    }

    #[test]
    fn case1() {
        let commands = vec![
            ("put", vec![1, 1]),
            ("put", vec![2, 2]),
            ("get", vec![1]),
            ("put", vec![3, 3]),
            ("get", vec![2]),
            ("get", vec![3]),
            ("put", vec![4, 4]),
            ("get", vec![1]),
            ("get", vec![2]),
            ("get", vec![3]),
            ("get", vec![4]),
        ];

        let expected = vec![
            None,
            None,
            Some(1),
            None,
            Some(-1),
            Some(3),
            None,
            Some(-1),
            Some(-1),
            Some(3),
            Some(4),
        ];

        let mut cache = LFUCache::new(2);

        let result = commands
            .iter()
            .map(|(command, args)| handle_command(&mut cache, command, args.clone()))
            .collect::<Vec<_>>();

        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        let commands = vec![
            ("put", vec![2, 2]), // freqs 2: 1
            ("put", vec![1, 1]), // freqs 2: 1, 1: 1
            ("get", vec![2]),    // ret 2. freqs: 2: 2, 1: 1
            ("get", vec![1]),    // ret 1. freqs: 1: 2, 2: 2
            ("get", vec![2]),    // ret 2. freqs: 2: 3, 1: 2
            ("put", vec![3, 3]), // freqs: 2: 3, 1: 2, 3: 1
            ("put", vec![4, 4]), // removing 3. freqs: 2: 3, 1: 2, 4: 1
            ("get", vec![3]),    // ret -1
            ("get", vec![2]),    // ret 2. freqs: 2: 4, 1: 2, 4: 1
            ("get", vec![1]),    // ret 1. freqs: 2: 4, 1: 3, 4: 1
            ("get", vec![4]),    // ret 4. freqs: 2: 4, 1: 3, 4: 2
        ];

        let expected = vec![
            None,
            None,
            Some(2),
            Some(1),
            Some(2),
            None,
            None,
            Some(-1),
            Some(2),
            Some(1),
            Some(4),
        ];

        let mut cache = LFUCache::new(3);

        let result = commands
            .iter()
            .map(|(command, args)| handle_command(&mut cache, command, args.clone()))
            .collect::<Vec<_>>();

        assert_eq!(result, expected);
    }
}
