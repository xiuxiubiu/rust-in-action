#![allow(dead_code, unused_variables)]

use std::{borrow::Borrow, hash::Hash};

struct HashMap<K, V> {
    k: K,
    v: V,
}

impl<K, V> HashMap<K, V> {
    fn get<Q>(&self, k: K) -> Option<&V>
    where
        Q: Eq + Hash,
        K: Borrow<Q>,
    {
        Some(&self.v)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Custom {}

fn main() {
    let k = Custom {};
    let v = "abc";
    let m = crate::HashMap { k, v };
    let c = Custom {};
    m.get(c);
}
