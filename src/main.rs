
fn main() {
    use my_hash_map::MyHashMap as HashMap;

    let mut my_map: HashMap<String, String> = HashMap::new();
    assert_eq!(my_map.len(), 0);
    let my_key = "my key";
    let my_val = "Hello, world!";
    my_map = my_map.insert(my_key.to_string(), my_val.to_string());
    assert_eq!(my_map.len(), 1);
    my_map = my_map.insert("my key2".to_string(), "Second value".to_string());
    assert_eq!(my_map.len(), 2);
    my_map = my_map.insert("ooooops".to_string(), "Third value".to_string());
    assert_eq!(my_map.get(&"my key2".to_string()), "Third value");
}



mod my_hash_map {
use std::option::Option;

    pub struct MyHashMap<K: Clone, V: Clone> {
        store: [Option<(K, V)>; 3],
    }

    impl<K: Clone, V: Clone> MyHashMap<K, V> {
        pub fn new() -> MyHashMap<K, V> {
            return MyHashMap { store: [None, None, None] };
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

        pub fn get(&self, key: &K) -> V {
            return self.store[self.hash_key(key)].clone().unwrap().1;
        }

        fn hash_key(&self, _key: &K) -> usize {
            let hash: usize;
            unsafe {
                hash = core::arch::x86_64::_rdrand16_step(&mut 0u16) as usize;
            }
            return hash.checked_rem(self.store.len()).unwrap();
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
        let my_key = "my key";
        let my_val = "Hello, world!";
        my_map.insert(my_key.to_string(), my_val.to_string());
        let act_result = my_map.get(my_key);
        assert_eq!(Some(&my_val.to_string()), act_result);
    }

    #[test]
    fn get_nothin() {
        let my_map = get_new_map();
        let act_result = my_map.get("Not a key");
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
        let my_key = "my key";
        let my_val = "Hello, world!";
        my_map.insert(my_key, my_val);
        assert_eq!(1, my_map.len());
    }
}
