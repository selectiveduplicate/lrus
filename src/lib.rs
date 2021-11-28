use std::collections::HashMap;
use std::collections::LinkedList;
use std::hash::Hash;

#[derive(Debug, Clone)]
/// An LRU cache using hashmap and doubly-linked list.
pub struct LRUCache<K, V>
where
    K: Eq + PartialEq + Copy + Hash,
{
    storage: HashMap<K, V>,
    order: LinkedList<K>,
    capacity: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + PartialEq + Copy + Hash,
{
    /// Creates a new `LRUCache` with specified capacity.
    /// Capacity is always a positive number.
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            storage: HashMap::with_capacity(capacity),
            order: LinkedList::new(),
            capacity,
        }
    }

    /// Inserts an item into the cache. 
    /// Each item is represented by a key-value pair.
    /// If the `key` already exists in the cache, 
    /// its corresponding value is updated.
    pub fn insert(&mut self, key: K, value: V) -> Option<K> {
        // If the list contains this key, then put it as the
        // front (most recent) element and insert into storage.
        // If the corresponding is value is new, it'll be updated.
        if self.order.contains(&key) {
            let mut updated_list = LinkedList::new();
            //let mut found_index: Option<usize> = None;
            let found_index = self.order.iter().enumerate().find(|(_, &element)| element == key).map(|found| found.0);
            updated_list.push_front(key);
            
            // Update the list
            let mut splitted_from_found = self.order.split_off(found_index.unwrap());
            splitted_from_found.pop_front();
            self.order.append(&mut splitted_from_found);
            updated_list.append(&mut self.order);
            self.order = updated_list;
            // Update storage
            self.storage.insert(key, value);
        } else {
            // It's a new key.
            //
            // If length has become equal to the capacity, we need to evict
            // the "back" (LRU) member, both from the list and storage.
            if self.order.len() == self.capacity as usize {
                let evicted = self.order.pop_back();
                self.storage.remove(&evicted.unwrap());
            }
            // Insert the new item
            self.storage.insert(key, value);
            self.order.push_front(key);
        }
        key.into()
    }

    /// Returns a reference to the value corresponding to the `key`.
    pub fn get(&mut self, key: K) -> Option<&V> {
        // If the list contains this key, then put it as the
        // front (most recent) element
        if self.order.contains(&key) {
            let mut updated_list = LinkedList::new();
            updated_list.push_front(key);
            let found_index = self.order.iter().enumerate().find(|(_, &element)| element == key).map(|found| found.0);
            
            // Update the list
            let mut splitted_from_found = self.order.split_off(found_index.unwrap());
            splitted_from_found.pop_front();
            self.order.append(&mut splitted_from_found);
            updated_list.append(&mut self.order);
            self.order = updated_list;
            self.storage.get(&key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod lrutests {
    use super::LRUCache;

    #[test]
    fn create_empty_cache() {
        let cache = LRUCache::<usize, &str>::new(3);
        assert_eq!(cache.capacity, 3);
        assert_eq!(cache.order.len(), 0);
        assert_eq!(cache.storage.len(), 0);
    }

    #[test]
    fn insert_up_to_capacity() {
        let mut cache = LRUCache::<usize, &str>::new(3);
        let first = cache.insert(1, "Sadness");
        let second = cache.insert(2, "Depression");
        let third = cache.insert(3, "Melancholy");

        assert_eq!((first, second, third), (Some(1), Some(2), Some(3)));

        assert_eq!(cache.capacity, 3);
        assert_eq!(cache.storage.len(), cache.order.len());
    }

    #[test]
    fn insert_beyond_capacity() {
        let mut cache = LRUCache::<usize, &str>::new(3);
        cache.insert(1, "Sadness");
        cache.insert(2, "Depression");
        cache.insert(3, "Melancholy");
        cache.insert(4, "Myth");

        assert_eq!(cache.storage.get(&4), Some(&"Myth"));

        assert_eq!(cache.order.len(), cache.capacity as usize);
        assert_eq!(cache.storage.len(), cache.capacity as usize);

        // 1 should be removed
        assert_eq!(cache.order.contains(&1), false);
        assert_eq!(cache.order.contains(&2), true);
        assert_eq!(cache.order.contains(&3), true);
        assert_eq!(cache.order.contains(&4), true);

        // 4 should be at the front (most recently used)
        assert_eq!(cache.order.pop_front(), Some(4));
    }

    #[test]
    fn insert_an_already_existing_item() {
        let mut cache = LRUCache::<usize, &str>::new(5);
        cache.insert(1, "Sadness");
        cache.insert(2, "Depression");
        cache.insert(3, "Shitty life");
        assert_eq!(cache.order.len(), 3);

        cache.insert(2, "Melancholy");

        assert_eq!(cache.storage.get(&2), Some(&"Melancholy"));
        assert_eq!(cache.order.len(), 3);

        // Check the list
        // Expected is:
        //
        // 2        3         1
        // MRU<------------->LRU
        assert_eq!(cache.order.pop_front(), Some(2));
        assert_eq!(cache.order.pop_front(), Some(3));
        assert_eq!(cache.order.pop_front(), Some(1));
    }

    #[test]
    fn insert_integers() {
        let mut cache = LRUCache::<i32, i32>::new(2);

        assert_eq!(cache.get(2), None);

        cache.insert(2, 6);

        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(&6));

        cache.insert(1, 5);
        cache.insert(1, 2);

        assert_eq!(cache.get(1), Some(&2));
        assert_eq!(cache.get(2), Some(&6));
        assert_eq!(cache.storage.len(), cache.capacity as usize);
    }

    #[test]
    fn get_an_existing_item() {
        let mut cache = LRUCache::<usize, &str>::new(5);
        cache.insert(1, "Sadness");
        cache.insert(2, "Depression");
        cache.insert(3, "Shitty life");

        let retrieved = cache.get(2);
        assert_eq!(retrieved, Some(&"Depression"));

        // Check the list
        // Expected is:
        //
        // 2        3         1
        // MRU<------------->LRU
        assert_eq!(cache.order.pop_front(), Some(2));
        assert_eq!(cache.order.pop_front(), Some(3));
        assert_eq!(cache.order.pop_front(), Some(1));
    }

    #[test]
    fn get_a_nonexisting_item() {
        let mut cache = LRUCache::<usize, &str>::new(5);
        cache.insert(1, "Sadness");
        cache.insert(2, "Depression");
        cache.insert(3, "Shitty life");

        let retrieved = cache.get(5);
        assert_eq!(retrieved, None);

        // check the list

        // Check the list
        // Expected is:
        //
        // 3        2         1
        // MRU<------------->LRU
        assert_eq!(cache.order.pop_front(), Some(3));
        assert_eq!(cache.order.pop_front(), Some(2));
        assert_eq!(cache.order.pop_front(), Some(1));
    }
}
