use std::borrow::Borrow;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;

const INITIAL_BUCKETS: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}
impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn key<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }
        let bucket_index = self.key(&key);
        let bucket = &mut self.buckets[bucket_index];
        for (k, v) in &mut bucket.iter_mut() {
            if k == &key {
                return Some(std::mem::replace(v, value));
            }
        }
        self.items += 1;
        bucket.push((key, value));
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket_index = self.key(key);
        let bucket = &self.buckets[bucket_index];
        for (k, v) in bucket {
            if k.borrow() == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket_index = self.key(key);
        let bucket = &mut self.buckets[bucket_index];
        if let Some(index) = bucket.iter().position(|(k, _)| k.borrow() == key) {
            self.items -= 1;
            // swap_remove is O(1) because the order of the elements in the bucket doesn't matter.
            Some(bucket.swap_remove(index).1)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKETS,
            n => n * 2,
        };
        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket_index = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket_index].push((key, value));
        }

        let _ = mem::replace(&mut self.buckets, new_buckets);
    }
}
impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.buckets
            .iter()
            .flat_map(|bucket| bucket.iter().map(|(k, v)| (k, v)))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.insert("foo", 42), None);
        assert_eq!(map.insert("foo", 43), Some(42));
    }

    #[test]
    fn get() {
        let mut map = HashMap::new();
        assert_eq!(map.insert("foo", 42), None);
        assert_eq!(map.get("foo"), Some(&42));
        assert_eq!(map.get("bar"), None);
    }

    #[test]
    fn remove() {
        let mut map = HashMap::new();
        assert_eq!(map.insert("foo", 42), None);
        assert_eq!(map.remove("bar"), None);
        assert_eq!(map.remove("foo"), Some(42));
        assert_eq!(map.remove("foo"), None);
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn iter() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        map.insert("bar", 43);
        let mut iter = map.into_iter();
        assert_eq!(iter.next(), Some((&"foo", &42)));
        assert_eq!(iter.next(), Some((&"bar", &43)));
        assert_eq!(iter.next(), None);
    }
}
