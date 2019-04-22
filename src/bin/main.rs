extern crate my_hash_map;

use my_hash_map::my_hash_map::MyHashMap as HashMap;

fn main() {
    let mut my_map: HashMap<String, String> = HashMap::new();
    assert_eq!(my_map.len(), 0);
    let my_key = "my key".to_string();
    let my_val = "Hello, world!".to_string();
    my_map = my_map.insert(my_key.clone(), my_val.clone());
    assert_eq!(my_map.len(), 1);
    assert_eq!(my_map.get(&my_key), Some(my_val));
    println!("my map: {}", my_map.to_string());
    println!("Assertions passed!");
}
