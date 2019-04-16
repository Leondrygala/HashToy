#[macro_use]
extern crate criterion;
extern crate my_hash_map;

use criterion::Criterion;
use criterion::black_box;
use my_hash_map::my_hash_map::MyHashMap;
use std::collections::HashMap;

fn insert_n(n: usize) {
    let mut my_map: HashMap<String, String> = HashMap::new();
    for i in 1..n+1 {
        my_map.insert(format!("{}{}", "key", i).to_string(), format!("{}{}", "val", i).to_string());
        assert_eq!(
            my_map.get(&format!("{}{}", "key", i).to_string()),
            Some(&format!("{}{}", "val", i).to_string())
        );
        assert_eq!(my_map.len(), i);
    }
    assert_eq!(my_map.get(&"key14".to_string()), Some(&"val14".to_string()));
}

fn insert_n_myhash(n: usize) {
    let mut my_map: MyHashMap<String, String> = MyHashMap::new();
    for i in 1..n+1 {
        my_map = my_map.insert(format!("{}{}", "key", i).to_string(), format!("{}{}", "val", i).to_string());
        assert_eq!(my_map.get(&format!("{}{}", "key", i).to_string()), Some(format!("{}{}", "val", i).to_string()));
        assert_eq!(my_map.len(), i);
    }
    assert_eq!(my_map.get(&"key14".to_string()), Some("val14".to_string()));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert 20", |b| b.iter(|| insert_n(black_box(20))));
    c.bench_function("insert 20 my hash", |b| b.iter(|| insert_n_myhash(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
