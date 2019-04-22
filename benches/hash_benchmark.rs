#[macro_use]
extern crate criterion;
extern crate my_hash_map;

use criterion::*;
use my_hash_map::my_hash_map::MyHashMap;
use std::collections::HashMap;

fn insert_n(n: usize) {
    let mut my_map: HashMap<String, String> = HashMap::new();
    for i in 1..n+1 {
        my_map.insert(
            format!("{}{}", "key", i).to_string(),
            format!("{}{}", "val", i).to_string()
        );
    }
}

fn insert_n_myhash(n: usize) {
    let mut my_map: MyHashMap<String, String> = MyHashMap::new();
    for i in 1..n+1 {
        my_map = my_map.insert(
            format!("{}{}", "key", i).to_string(),
            format!("{}{}", "val", i).to_string()
        );
    }
}

fn setup_hash_n(n: usize) -> MyHashMap<String, String> {
    let mut my_map: MyHashMap<String, String> = MyHashMap::new();
    for i in 1..n+1 {
        my_map = my_map.insert(
            format!("{}{}", "key", i).to_string(),
            format!("{}{}", "val", i).to_string()
        );
    }
    my_map
}

fn setup_std_hash_n(n: usize) -> HashMap<String, String> {
    let mut my_map: HashMap<String, String> = HashMap::new();
    for i in 1..n+1 {
        my_map.insert(
            format!("{}{}", "key", i).to_string(),
            format!("{}{}", "val", i).to_string()
        );
    }
    my_map
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "setup MyHashMap and insert n",
        move |b, &&size| { b.iter_batched_ref(
            || setup_hash_n(size),
            |m| { m.insert("new key".to_string(), "new val".to_string()); },
            BatchSize::SmallInput
        );},
        &[1, 10, 50, 100]
    );
    c.bench_function_over_inputs(
        "setup std::HashMap and insert n",
        move |b, &&size| { b.iter_batched_ref(
            || setup_std_hash_n(size),
            |m| { m.insert("new key".to_string(), "new val".to_string()); },
            BatchSize::SmallInput
        );},
        &[1, 10, 50, 100]
    );
    c.bench_function_over_inputs(
        "setup std::HashMap and insert n",
        move |b, &&size| { b.iter_batched_ref(
            || setup_std_hash_n(size),
            |m| { m.insert("new key".to_string(), "new val".to_string()); },
            BatchSize::SmallInput
        );},
        &[1, 10, 50, 100]
    );
    c.bench_function_over_inputs(
            "MyHashMap insert n from 0",
            |b, &&size| { b.iter(|| insert_n_myhash(size));},
            &[1, 10, 50, 100]
    );
    c.bench_function_over_inputs(
            "std::HashMap insert n from 0",
            |b, &&size| { b.iter(|| insert_n(size));},
            &[1, 10, 50, 100]
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
