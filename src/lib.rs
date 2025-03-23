#![deny(missing_docs)]
#![doc = "module"]

/// KvStore is a simple in memory key-value store.
pub struct KvStore {
    store: std::collections::HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> Self {
        KvStore {
            store: std::collections::HashMap::new(),
        }
    }

    /// Get the value of a key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.get("key1".to_owned());
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        if self.store.contains_key(key.as_str()) {
            return Some(self.store.get(key.as_str()).unwrap().to_string());
        }

        None
    }

    /// Get the value of a key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Get the value of a key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.remove("key1".to_owned());
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(key.as_str());
    }
}
