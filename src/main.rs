
fn main() {
    use my_hash_map::MyHashMap as HashMap;

    let mut my_map: HashMap<String, String> = HashMap::new();
    assert_eq!(my_map.len(), 0);
    let my_key = "my key".to_string();
    let my_val = "Hello, world!".to_string();
    my_map = my_map.insert(my_key.clone(), my_val.clone());
    assert_eq!(my_map.len(), 1);
    assert_eq!(my_map.get(&my_key), Some(my_val));
    println!("Assertions passed!");
}

mod my_hash_map {
use std::option::Option;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

    pub struct MyHashMap<K: Clone + Hash + PartialEq, V: Clone> {
        store: [HashEntry<K,V>; 3]
    }

    pub enum HashEntry<K: Clone + Hash + PartialEq, V: Clone> {
        Entry(Option<(K, V)>),
        Clash(Box<MyHashMap<K, V>>),
    }

    impl<K: Clone + Hash + PartialEq, V: Clone> Default for HashEntry<K, V> {
        fn default() -> Self { HashEntry::Entry(None) }
    }

    impl<K: Clone + Hash + PartialEq, V: Clone> MyHashMap<K, V> {


        pub fn new() -> MyHashMap<K, V> {
            return MyHashMap { store: Default::default() };
        }

        pub fn len(&self) -> usize {
            return self.store.iter().fold(
                0,
                |count, entries| count + match entries {
                    HashEntry::Entry(Some(_)) => 1,
                    HashEntry::Entry(None) => 0,
                    HashEntry::Clash(map) => map.len(),
                }
            );
        }

        pub fn insert(&self, key: K, val: V) -> MyHashMap<K, V> {
            return self.attempt_insert(key, val, 0);
        }

        pub fn get(&self, key: &K) -> Option<V> {
            return self.attempt_get(key, 0);
        }

        pub fn attempt_get(&self, key: &K, atmpt: u64) -> Option<V> {
            return match self.store[self.hash_to_bucket(key, atmpt)] {
                HashEntry::Entry(entry) => entry.map(|p| p.1),
                HashEntry::Clash(map) => map.attempt_get(key, atmpt + 1),
            }
        }

        fn hash_to_bucket(&self, key: &K, salt: u64) -> usize {
            let hash = MyHashMap::<K, V>::hash_key(key) + salt;
            return hash.checked_rem(self.store.len() as u64).unwrap() as usize;
        }

        fn hash_key(key: &K) -> u64 {
            let mut s = DefaultHasher::new();
            key.hash(&mut s);
            return s.finish();
        }

        fn attempt_insert(& self, key: K, val: V, atmpt: u64) -> MyHashMap<K, V> {
            let i = self.hash_to_bucket(&key, atmpt);
            let mut new_store: [HashEntry<K, V>; 3] = self.store.clone();
            match self.store[i] {
                HashEntry::Clash(map) => map.attempt_insert(key, val, atmpt + 1),
                HashEntry::Entry(None) => { 
                    new_store[i] = HashEntry::Entry(Some((key, val)));
                    MyHashMap { store: new_store }
                },
                HashEntry::Entry(Some((old_key, old_val))) => {
                    // check if key is the same
                        new_store[i] = HashEntry::Entry(Some((key, val)));
                    // otherwise, make a new hashmap
                        let mut new_map_entry: MyHashMap<K, V> = MyHashMap::new();
                        new_map_entry.attempt_insert(old_key, old_val, 0); 
                        new_map_entry.attempt_insert(key, val, 1);// or should tthis be atmpt? There might be some weird behaviour here
                        new_store[i] = HashEntry::Clash(Box::new(new_map_entry));
                    MyHashMap { store: new_store }
                },
            }
        }

    }
}

#[cfg(test)]
mod tests {
use my_hash_map::MyHashMap as HashMap;
// use std::collections::HashMap as HashMap;

    fn get_new_map() -> HashMap<String, String>{
        let map: HashMap<String, String> = HashMap::new();
        return map;
    }

    #[test]
    fn insert_and_get() {
        let mut my_map: HashMap<String, String> = HashMap::new();
        let my_key = "my key".to_string();
        let my_val = "Hello, world!".to_string();
        my_map = my_map.insert(my_key.clone(), my_val.clone());
        let act_result = my_map.get(&my_key);
        assert_eq!(Some(my_val), act_result);
    }

    #[test]
    fn insert_3() {
        let mut my_map: HashMap<String, String> = HashMap::new();
        let my_key = "my key".to_string();
        let my_val = "Hello, world!".to_string();
        my_map = my_map.insert(my_key.clone(), my_val.clone());
        my_map = my_map.insert("key2".to_string(), "val2".to_string());
        my_map = my_map.insert("key3".to_string(), "val3".to_string());
        let act_result = my_map.get(&my_key);
        assert_eq!(Some(my_val), act_result);
        assert_eq!(3, my_map.len());
    }

    #[test]
    fn get_nothin() {
        let my_map = get_new_map();
        let act_result = my_map.get(&"Not a key".to_string());
        assert_eq!(act_result, None);
    }

    // Length 
    #[test]
    fn inital_len_0() {
        let my_map = get_new_map();
        assert_eq!(0, my_map.len());
    }
    
    #[test]
    fn len_1() {
        let mut my_map = HashMap::new();
        let my_key = "my key".to_string();
        let my_val = "Hello, world!".to_string();
        my_map = my_map.insert(my_key, my_val);
        assert_eq!(1, my_map.len());
    }

    #[test]
    fn update_value() {
        let mut my_map = HashMap::new();
        let my_key = "my key".to_string();
        let my_val1 = "Hello, world!".to_string();
        let my_val2 = "New Value!".to_string();
        my_map = my_map.insert(my_key.clone(), my_val1.clone());
        my_map = my_map.insert(my_key.clone(), my_val2.clone());
        assert_eq!(1, my_map.len());
        assert_eq!(my_map.get(&my_key), Some(my_val2));
    }
}
