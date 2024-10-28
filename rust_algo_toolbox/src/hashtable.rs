use std::collections::LinkedList;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::ops::Index;


#[derive(Debug)]
struct BucketNode<K, V> {
    key: K,
    value: V
}

struct Bucket<K, V> {
    list: LinkedList<BucketNode<K, V>>
}

impl<K, V> Default for Bucket<K, V> {
    fn default() -> Bucket<K, V> {
        Bucket{ list: LinkedList::new() }
    }
}

const DEFAULT_CAPACITY: usize = 16;

pub struct HashTable<K, V>
{
    table: Vec<Bucket<K, V>>,
    count: usize
}

impl<K, V> HashTable<K, V>
where K: Eq + std::hash::Hash
{
    pub fn new() -> HashTable<K, V> {
        let mut container: Vec<Bucket<K, V>> = Vec::new();
        container.resize_with(DEFAULT_CAPACITY, Default::default);

        HashTable{ table: container, count: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let hash = HashTable::<K, V>::hash_for(&key);

        let pos_in_table = hash as usize % self.nbuckets();
        let bucket = &mut self.table[pos_in_table];

        if let Some(node) = bucket.list.iter_mut().find(|node| { node.key == key }) {
            *node = BucketNode {key, value};
        } else {
            bucket.list.push_front(BucketNode {key, value});
            self.count += 1
        }
    }


    pub fn len(&self) -> usize {
        self.count
    }

    pub fn nbuckets(&self) -> usize {
        self.table.len()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = HashTable::<K, V>::hash_for(&key);

        let pos_in_table = hash as usize % self.nbuckets();
        let bucket = &self.table[pos_in_table];

        if let Some(node) = bucket.list.iter().find(|node| { node.key == *key }) {
            Some(&node.value)
        } else {
            None
        }
    }

    fn hash_for(key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}

impl<K, V> Index<&K> for HashTable<K, V>
where K: Eq + std::hash::Hash
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("entry not found")
    }
}



#[cfg(test)]
mod tests {
    use super::HashTable;
    use rand;
    use rand::Rng;

    #[test]
    fn default_empty() {
        let ht = HashTable::<i32, i32>::new();
        assert_eq!(ht.count, 0);
        assert_eq!(ht.nbuckets(), super::DEFAULT_CAPACITY);
    }

    #[test]
    fn insert_changes_len() {
        let mut ht = HashTable::<i32, i32>::new();
        let n = 128;
        for i in 0..n {
            ht.insert(i, i*2+1);
            assert_eq!(ht.len(), (i + 1) as usize);
        }
    }

    #[test]
    fn insert_replaces_if_found() {
        let unique_keys = 30;

        let mut ht = HashTable::<i32, i32>::new();
        let n = 128;
        for i in 0..n {
            ht.insert(i % unique_keys, i*2+1);
            assert_eq!(ht.len(), std::cmp::min(i + 1, unique_keys) as usize);
        }
    }

    #[test]
    fn get_existing_returns_link() {
        let rng = 1..=10;
        let keys: Vec<i32> = rng.collect();
        let values: Vec<i32> = keys.iter().map(|k| k * 10 + 1).collect();

        let mut ht = HashTable::<i32, i32>::new();
        for (k, v) in keys.iter().zip(values.iter()) {
            ht.insert(*k, *v);
        }

        let pairs: Vec<(&i32, &i32)> = keys.iter().zip(values.iter()).collect();

        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            let idx = rng.gen_range(0..10);
            let (k, v) = pairs[idx];

            let value_in_table_get = ht.get(k);
            let value_in_table_index = ht[k];

            assert_eq!(value_in_table_get, Some(v));
            assert_eq!(value_in_table_index, *v);
        }
    }

    #[test]
    fn get_nonexisting_returns_none() {
        let rng = 1..=10;
        let keys: Vec<i32> = rng.collect();
        let values: Vec<i32> = keys.iter().map(|k| k * 10 + 1).collect();

        let mut ht = HashTable::<i32, i32>::new();
        for (k, v) in keys.iter().zip(values.iter()) {
            ht.insert(*k, *v);
        }

        for ne_key in (1..=10).map(|k| k * 31) {
            let expected_none = ht.get(&ne_key);

            assert_eq!(expected_none, None);
        }
    }

    #[test]
    #[should_panic(expected = "entry not found")]
    fn index_nonexisting_panics() {
        let ht = HashTable::<i32, i32>::new();

        ht[&10];
    }
}
