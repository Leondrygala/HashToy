pub mod my_hash_map {
    use std::option::Option;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    const SIZE: usize = 4;

    #[derive(Clone)]
    pub struct MyHashMap<K: Clone + Hash + PartialEq, V: Clone> {
        store: [HashEntry<K,V>; SIZE]
    }

    #[derive(Clone)]
    pub enum HashEntry<K: Clone + Hash + PartialEq, V: Clone> {
        Entry(Option<(K, V)>),
        Clash(Box<MyHashMap<K, V>>),
    }

    impl<
        K: Clone + Hash + PartialEq + ToString,
        V: Clone + ToString
    > Default for HashEntry<K, V> {
        fn default() -> Self { HashEntry::Entry(None) }
    }

    impl<
        K: Clone + Hash + PartialEq + ToString,
        V: Clone + ToString
    > MyHashMap<K, V> {

        pub fn new() -> MyHashMap<K, V> {
            return MyHashMap { store: Default::default() };
        }

        pub fn to_string(&self) -> String {
            return self.store.iter().fold(
                "[".to_string(),
                |string, entries| 
                    string + "\n\t" + 
                    &match entries {
                        HashEntry::Entry(Some((k, v))) =>
                            format!(
                                "Some({}, {})",
                                k.to_string(),
                                v.to_string()
                            ),
                        HashEntry::Entry(None) => "None".to_string(),
                        HashEntry::Clash(map) =>
                            map.to_string().chars().flat_map(|ch| match ch {
                                '\n' => vec!['\n', '\t'].into_iter(),
                                c    => vec![c].into_iter()}
                            ).collect::<String>()
                    }
            ) + "\n]";
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

        pub fn get(&self, key: &K) -> Option<V> {
            return self.attempt_get(key, 0);
        }

        pub fn attempt_get(&self, key: &K, atmpt: u64) -> Option<V> {
            return match &self.store[self.hash_to_bucket(key, atmpt)] {
                HashEntry::Entry(entry) => entry.as_ref().map(|p| p.1.clone()),
                HashEntry::Clash(map) => map.attempt_get(key, atmpt + 1),
            }
        }

        fn hash_to_bucket(&self, key: &K, atmpt: u64) -> usize {
            let hash = MyHashMap::<K, V>::hash_key(key, atmpt + 1);
            return hash.checked_rem(self.store.len() as u64).unwrap() as usize;
        }

        fn hash_key(key: &K, repetitions: u64) -> u64 {
            let mut s = DefaultHasher::new();
            for _ in 0..repetitions {
                key.hash(&mut s);
            }
            return s.finish();
        }

        pub fn insert(&self, key: K, val: V) -> MyHashMap<K, V> {
            self.attempt_insert(key, val, 0)
        }

        fn attempt_insert(& self, key: K, val: V, atmpt: u64)
            -> MyHashMap<K, V> {
            let i = self.hash_to_bucket(&key, atmpt);
            let mut new_store: [HashEntry<K, V>; SIZE] = self.store.clone();
            new_store[i] = match self.store[i] {
                HashEntry::Clash(ref map) =>
                    HashEntry::Clash(Box::new(
                        map.attempt_insert(key, val, atmpt + 1)
                    ))
                ,
                HashEntry::Entry(None) => { 
                    HashEntry::Entry(Some((key, val)))
                },
                HashEntry::Entry(Some((ref old_key, ref old_val))) => {
                    if old_key == &key {
                        HashEntry::Entry(Some((key, val)))
                    } else {
                        let mut new_map_entry: MyHashMap<K, V> =
                            MyHashMap::new();
                        HashEntry::Clash(Box::new(new_map_entry
                            .attempt_insert(
                                old_key.clone(),
                                old_val.clone(),
                                atmpt + 1
                            ).attempt_insert(key, val, atmpt + 1)
                            // Above actually have a 1/store.len chance of
                            // clashing again
                        ))
                    }
                },
            };
            MyHashMap { store: new_store }
        }
    }
}

#[cfg(test)]
mod tests {
use my_hash_map::MyHashMap as HashMap;

    fn get_new_map() -> HashMap<String, String>{
        let map: HashMap<String, String> = HashMap::new();
        return map;
    }

    #[test]
    fn insert_1() {
        let mut my_map: HashMap<String, String> = HashMap::new();
        let my_key = "my key".to_string();
        let my_val = "Hello, world!".to_string();
        my_map = my_map.insert(my_key.clone(), my_val.clone());
        let act_result = my_map.get(&my_key);
        assert_eq!(act_result, Some(my_val));
        assert_eq!(my_map.len(), 1);
    }

    #[test]
    fn insert_3() {
        let mut my_map: HashMap<String, String> = HashMap::new();
        my_map = my_map.insert("my key".to_string(), "my val".to_string())
                       .insert("key2".to_string(), "val2".to_string())
                       .insert("key3".to_string(), "val3".to_string());
        assert_eq!(my_map.len(), 3);
        assert_eq!(my_map.get(&"my key".to_string()), Some("my val".to_string()));
    }

    #[test]
    fn insert_n() {
        let n = 50;
        let mut my_map: HashMap<String, String> = HashMap::new();
        for i in 1..n+1 {
            my_map = my_map.insert(format!("{}{}", "key", i).to_string(), format!("{}{}", "val", i).to_string());
            assert_eq!(my_map.get(&format!("{}{}", "key", i).to_string()), Some(format!("{}{}", "val", i).to_string()));
            assert_eq!(my_map.len(), i);
        }
        println!("Finished iteration {}, map is now: {}", n, my_map.to_string());
        //assert_eq!(1,2);
        assert_eq!(my_map.get(&"key14".to_string()), Some("val14".to_string()));
    }

    #[test]
    fn get_nothin() {
        let my_map = get_new_map();
        let act_result = my_map.get(&"Not a key".to_string());
        assert_eq!(act_result, None);
    }

    #[test]
    fn inital_len_0() {
        let my_map = get_new_map();
        assert_eq!(my_map.len(), 0);
    }

    #[test]
    fn update_value() {
        let mut my_map = HashMap::new();
        let my_key = "my key".to_string();
        let my_val1 = "Hello, world!".to_string();
        let my_val2 = "New Value!".to_string();
        my_map = my_map.insert(my_key.clone(), my_val1.clone());
        my_map = my_map.insert(my_key.clone(), my_val2.clone());
        assert_eq!(my_map.len(), 1);
        assert_eq!(my_map.get(&my_key), Some(my_val2));
    }
}
