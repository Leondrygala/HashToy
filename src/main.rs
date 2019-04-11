
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
        store: [Vec<(K, V)>; 3],
    }

    impl<K: Clone + Hash + PartialEq, V: Clone> MyHashMap<K, V> {
        pub fn new() -> MyHashMap<K, V> {
            return MyHashMap { store: Default::default() };
        }

        pub fn len(&self) -> usize {
            return self.store.iter().fold(
                0,
                |count, items| count + items.len()
            );
        }

        pub fn insert(&self, key: K, val: V) -> MyHashMap<K, V> {
            let i = self.hash_to_bucket(&key);
            return self.insert_at(key, val, i);
        }

        pub fn get(&self, key: &K) -> Option<V> {
            return self.store[self.hash_to_bucket(key)].iter().find_map(
                |(k,v)| if k == key { 
                    Some(v.clone()) 
                } else {
                    None
                }
            );
        }

        fn hash_to_bucket(&self, key: &K) -> usize {
            let hash = MyHashMap::<K, V>::hash_key(key);
            return hash.checked_rem(self.store.len() as u64).unwrap() as usize;
        }

        fn hash_key(key: &K) -> u64 {
            let mut s = DefaultHasher::new();
            key.hash(&mut s);
            return s.finish();
        }

        fn insert_at(& self, _key: K, _val: V, i: usize) -> MyHashMap<K, V> {
            let mut new_store = self.store.clone();
            new_store[i].push((_key, _val));
            return MyHashMap { store: new_store, };
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
