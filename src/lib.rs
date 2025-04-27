use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Result as AnyResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct OpSet {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
enum Operation {
    Set { key: String, value: String },
}

pub type Result<T> = AnyResult<T>;

pub struct KvStore {
    store: std::collections::HashMap<String, String>,
    file: File,
    wal: Vec<Operation>,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let file = File::options()
            .read(true)
            .append(true)
            .create(true)
            .open(path)?;

        let kv = KvStore {
            store: std::collections::HashMap::new(),
            wal: Vec::new(),
            file,
        };

        Ok(kv)
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        panic!("unimplemented");
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let s = serde_json::to_string(&Operation::Set { key, value })?;
        self.file.write(&s.as_bytes())?;

        return Ok(());
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        panic!("unimplemented");
    }
}
