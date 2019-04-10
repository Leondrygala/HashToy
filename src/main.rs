
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

    pub struct MyHashMap<K: Clone + Hash, V: Clone> {
        store: [Option<(K, V)>; 31],
    }

    impl<K: Clone + Hash, V: Clone> MyHashMap<K, V> {
        pub fn new() -> MyHashMap<K, V> {
            return MyHashMap { store: Default::default() };
        }

        pub fn len(&self) -> usize {
            return self.store.iter().fold(
                0,
                //|count, pair| pair.map_or(count, |_| count) 
                |count, pair| match pair {
                    Some(_) => count + 1,
                    None => count
                }
            );
        }

        pub fn insert(&self, key: K, val: V) -> MyHashMap<K, V> {
            let i = self.hash_key(&key);
            return self.insert_at(key, val, i);
        }

        pub fn get(&self, key: &K) -> Option<V> {
            return self.store[self.hash_key(key)].clone().map(|p| p.1);
        }

        fn hash_key(&self, key: &K) -> usize {
            let mut s = DefaultHasher::new();
            key.hash(&mut s);
            let hash = s.finish();
            return hash.checked_rem(self.store.len() as u64).unwrap() as usize;
        }

        fn insert_at(& self, _key: K, _val: V, i: usize) -> MyHashMap<K, V> {
            let mut new_store = self.store.clone();
            new_store[i] = Some((_key, _val));
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
    fn insert_and_get3() {
        let mut my_map: HashMap<String, String> = HashMap::new();
        let my_key = "my key".to_string();
        let my_val = "Hello, world!".to_string();
        my_map = my_map.insert(my_key.clone(), my_val.clone());
        my_map = my_map.insert("key2".to_string(), "val2".to_string());
        my_map = my_map.insert("key3".to_string(), "val3".to_string());
        let act_result = my_map.get(&my_key);
        assert_eq!(Some(my_val), act_result);
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
}
