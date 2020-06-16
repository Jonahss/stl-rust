#![feature(entry_insert)]
use std::collections::{HashMap, HashSet};

fn main() {
    let mut map: HashMap<&String, &String> = HashMap::new();

    let hi = String::from("hi");
    let hihi = &hi;

    let keys = vec!(String::from("a"), String::from("b"), String::from("c"));

    map.insert(&keys[0], &keys[0]);
    map.insert(&keys[1], &keys[1]);
    map.insert(&keys[2], &keys[2]);
    
    let a = map.get(&String::from("a")).unwrap();
    let b = map.get(&String::from("b")).unwrap();
    let c = map.get(&String::from("c")).unwrap();

    println!("\n\n\n");
    //println!("first key: {}", first_key); // can't use it, was moved into the hashmap
    println!("a: {}, b: {}, c: {}\n\n\n", a, b, c);
    println!("references equal? {}", &hi == hihi);

    let mut set: HashSet<&String> = HashSet::new();

    let hi = String::from("hi");
    let also_hi = String::from("hi");
    let chuck = String::from("chuckwudi");
    set.insert(&hi);
    set.insert(&also_hi);
    set.insert(&chuck);

    println!("set: {:?}", set);
}
