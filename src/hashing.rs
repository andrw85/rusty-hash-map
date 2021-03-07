use std::collections::hash_map::DefaultHasher;
use std::default::Default;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::mem;
use std::result::Result;
pub enum Error {}

#[derive(Debug)]
pub struct RustyHashMap<K, V> {
    content: Vec<Vec<(K, V)>>,
    buckets: u64,
}

impl<K, V> RustyHashMap<K, V>
where
    K: PartialEq + Eq + Default + Hash + Clone + Debug,
    V: Clone,
{
    pub fn new() -> RustyHashMap<K, V>
    where
        V: Default + Clone,
    {
        RustyHashMap {
            content: Vec::new(),
            buckets: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), Error> {
        self.resize();
        let index = self.hash_id(&key);
        println!("hash key {:?} inserting {}", key, index);
        self.content[index].push((key, value));
        Ok(())
    }

    pub fn contains(&self, key: K) -> Option<&V> {
        let index = self.hash_id(&key);
        println!("hash key {:?} is in index {}", key, index);
        match self.content[index].iter().find(|&x| x.0 == key) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    fn hash_id(&self, k: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        (hasher.finish() % (self.content.len() as u64)) as usize
    }

    fn resize(&mut self) {
        match self.content.len() {
            0 => {
                self.buckets += 1;
                println!(
                    "current number buckets: {} new size {}",
                    self.content.len(),
                    self.buckets
                );
                let empty = Vec::<(K, V)>::new();
                self.content.push(empty);
            }
            n => {
                self.buckets = n as u64 * 2;
                println!(
                    "current number buckets: {} new size {}",
                    self.content.len(),
                    self.buckets
                );
                let mut old_data: Vec<Vec<(K, V)>> = Vec::new();
                (0..self.buckets).for_each(|_| {
                    old_data.push(Vec::<(K, V)>::new()); // init buckets
                });
                mem::swap(&mut old_data, &mut self.content);
                old_data.into_iter().for_each(|v: Vec<(K, V)>| {
                    v.iter().for_each(|el| {
                        let hkey = self.hash_id(&el.0);
                        println!("moving key {:?} to index {}", el.0, hkey);
                        self.content[hkey].push(el.clone());
                    });
                });
            }
        }
    }
}
