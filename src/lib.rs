pub struct KvStore {
    store: std::collections::HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            store: std::collections::HashMap::new(),
        }
    }

    pub fn get(self: &Self, key: String) -> Option<String> {
        if self.store.contains_key(key.as_str()) {
            return Some(self.store.get(key.as_str()).unwrap().to_string());
        }

        return None;
    }

    pub fn set(self: &mut Self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn remove(self: &mut Self, key: String) {
        self.store.remove(key.as_str());
    }
}
