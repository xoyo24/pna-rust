use std::collections::HashMap;

/// A data type support woking as a key-value store
///
/// # Examples
///
/// ```no_run
/// # use kvs::KvStore;
///
/// let mut store = KvStore::new();
/// store.set("key1".to_owned(), "value1".to_owned());
/// store.get("key1".to_owned());
/// store.remove("key1".to_owned());
///
/// ```
pub struct KvStore {
    values: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        KvStore::new()
    }
}

impl KvStore {
    /// Creates a new instance of KvStore
    pub fn new() -> KvStore {
        KvStore {
            values: HashMap::new(),
        }
    }

    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.values.insert(key, value);
        unimplemented!("unimplemented")
    }

    /// Get the string value of a given string key
    pub fn get(&self, key: String) -> Option<String> {
        self.values.get(&key).map(|s| s.to_owned());
        unimplemented!("unimplemented")
    }

    /// Remove a given key
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.values.remove(&key);
        unimplemented!("unimplemented")
    }
}
