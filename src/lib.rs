use std::collections::HashMap;
use std::collections::LinkedList;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct LRUCache<K,  V> 
where K: Eq + PartialEq + Copy + Hash
{
    storage: HashMap<K, V>,
    order: LinkedList<K>,
    capacity: u32,
}

impl<K, V> LRUCache<K, V> 
where K: Eq + PartialEq + Copy + Hash
{
    pub fn new(capacity: u32) -> Self {
        LRUCache {
            storage: HashMap::new(),
            order: LinkedList::new(),
            capacity,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<K> {
        if self.order.len() < self.capacity as usize {
            self.storage.insert(key, value);
        // If length has become equal to the capacity, we need to evict 
        // the "back" (LRU) member, both from the list and storage.
        } else {
            let evicted = self.order.pop_back();
            // Two possibilities: the key exists or doesn't.
            // In both cases, we can call the hashmap's `insert` method.
            // The value will get updated in case of a new value or 
            // it's a new key and so a new record will end up in there.
            self.storage.remove(&evicted.unwrap());
            self.storage.insert(key, value);
        }
        // If the list contains this key, then put it as the
        // front (most recent) element
        if self.order.contains(&key) {
            let mut updated_list = LinkedList::new();
            let mut found_index: Option<usize> = None;
            for (index, element) in self.order.iter().enumerate() {
                if *element == key {
                    updated_list.push_front(key.clone());
                    found_index = Some(index);
                    break;
                }
            }
            // Update the list
            let mut splitted_from_found = self.order.split_off(found_index.unwrap());
            splitted_from_found.pop_front();
            self.order.append(&mut splitted_from_found);
            updated_list.append(&mut self.order);
            self.order = updated_list;
        } else {
            self.order.push_front(key);
        }
        key.into()
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        // If the list contains this key, then put it as the
        // front (most recent) element
        if self.order.contains(&key) {
            let mut updated_list = LinkedList::new();
            let mut found_index: Option<usize> = None;
            for (index, element) in self.order.iter().enumerate() {
                if *element == key {
                    updated_list.push_front(key.clone());
                    found_index = Some(index);
                    break;
                }
            }
            // Update the list
            let mut splitted_from_found = self.order.split_off(found_index.unwrap());
            splitted_from_found.pop_front();
            self.order.append(&mut splitted_from_found);
            updated_list.append(&mut self.order);
            self.order = updated_list;
            return self.storage.get(&key);
        } else {
            None
        }
    }
}

#[cfg(test)]
mod lrutests {
    use super::*;

    #[test]
    fn create_empty_cache() {
        let cache = LRUCache::<usize, &str>::new(3);
        assert_eq!(cache.capacity, 3);
        assert_eq!(cache.order.len(), 0);
        assert_eq!(cache.storage.len(), 0);
    }

    #[test]
    fn insert_till_capacity() {
        let mut cache = LRUCache::<usize, &str>::new(3);
        let first = cache.insert(1, "Magnus");
        let second = cache.insert(2, "Alireza Firouza");
        let third = cache.insert(3, "Fabiano Caruana");

        assert_eq!((first, second, third), (Some(1), Some(2), Some(3)));

        assert_eq!(cache.capacity, 3);
        assert_eq!(cache.storage.len(), cache.order.len());
    }
    
    #[test]
    fn insert_beyond_capacity() {
        let mut cache = LRUCache::<usize, &str>::new(3);
        cache.insert(1, "Magnus");
        cache.insert(2, "Alireza Firouza");
        cache.insert(3, "Fabiano Caruana");
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
        cache.insert(1, "Magnus");
        cache.insert(2, "Alireza Firouza");
        cache.insert(3, "Shitty life");
        assert_eq!(cache.order.len(), 3);

        cache.insert(2, "Fabiano Caruana");

        assert_eq!(cache.storage.get(&2), Some(&"Fabiano Caruana"));
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
    fn stupid_test() {
        
        let mut cache = LRUCache::<i32, i32>::new(2);

        assert_eq!(cache.get(2), None);

        cache.insert(2, 6);

        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(&6));

        cache.insert(1, 5);
        cache.insert(1, 2);

        assert_eq!(cache.get(1), Some(&2));
        assert_eq!(cache.get(2), Some(&6));
        //assert_eq!(cache.get(2), Some(&"Alireza Firouza"));
        //assert_eq!(cache.get(3), Some(&"Shitty life"));
        //assert_eq!(cache.get(1), Some(&"Magnus"));
        
        //assert_eq!(cache.order.pop_front(), Some(1));
        //assert_eq!(cache.order.pop_front(), Some(3));
        //assert_eq!(cache.order.pop_front(), Some(2));
 
    }
    
    #[test]
    fn get_an_existing_item() {
        let mut cache = LRUCache::<usize, &str>::new(5);
        cache.insert(1, "Magnus");
        cache.insert(2, "Alireza Firouza");
        cache.insert(3, "Shitty life");

        let retrieved = cache.get(2);
        assert_eq!(retrieved, Some(&"Alireza Firouza"));
        
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
        cache.insert(1, "Magnus");
        cache.insert(2, "Alireza Firouza");
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
