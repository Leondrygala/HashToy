
fn main() {
    use my_hash_map::MyHashMap as HashMap;

    let mut my_map: HashMap = HashMap::new();
    let my_key = "my key";
    let my_val = "Hello, world!";
    my_map.insert(my_key.to_string(), my_val.to_string());
    println!("Hello, world! Map is {}", my_map.len());
}

mod my_hash_map {
use std::option::Option;

    pub struct MyHashMap {
        pub store: [Option<(String, String)>; 3]
    }

    impl MyHashMap {
        pub fn new() -> MyHashMap {
            return MyHashMap { store: [None, None, None] };
        }

        pub fn len(& self) -> usize {
            let count = 0;
            for x in &self.store {
                x.and_then({count = count + 1;});
            }
            return count;
        }

        pub fn insert(&mut self, _key: String, _val: String) {
            self.store[0] = (_key, _val);
        }
    }

}

#[cfg(test)]
mod tests {
// use my_hash_map::MyHashMap as HashMap;
use std::collections::HashMap as HashMap;

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
