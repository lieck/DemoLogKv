use std::collections::HashMap;

pub struct KvStore {
    kv: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { kv: HashMap::new() }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.kv.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
        if self.kv.contains_key(&key) {
            return Some(self.kv.get(&key).unwrap().clone());
        }
        None
    }

    pub fn remove(&mut self, key: String) {
        self.kv.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
