pub mod error;
pub use error::KvsError;

use anyhow::Result as AnyResult;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

pub type Result<T> = AnyResult<T, KvsError>;

#[derive(Serialize, Deserialize, Debug)]
enum Operation {
    Set { key: String, value: String },
    Rm { key: String },
}

pub struct KvStore {
    path: PathBuf,
}

impl KvStore {
    pub fn open(dir_path: &Path) -> Result<KvStore> {
        let kv = KvStore {
            path: dir_path.to_path_buf(),
        };

        std::fs::create_dir_all(&kv.path)?;
        match File::create_new(Path::join(&kv.path, "wal")) {
            _ => {}
        }

        Ok(kv)
    }

    fn wal_path(&self) -> PathBuf {
        Path::new(&self.path).join("wal")
    }

    fn append_to_wal(&self, op: Operation) -> Result<()> {
        let mut file = File::options().append(true).open(&self.wal_path())?;
        let serialized_op = serde_json::to_string(&op)?;
        let line = format!("{}\n", serialized_op);

        file.write(line.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        panic!("unimplemented");
    }

    pub fn set(&self, key: String, value: String) -> Result<()> {
        self.append_to_wal(Operation::Set { key, value })?;

        return Ok(());
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let file = File::open(self.wal_path())?;
        let reader = BufReader::new(&file);
        let mut found = false;

        for line in reader.lines() {
            let operation = serde_json::from_str(&line?)?;

            match operation {
                Operation::Set { key: k, .. } if key == k => {
                    found = true;
                    break;
                }
                _ => {}
            }
        }

        if found {
            self.append_to_wal(Operation::Rm { key })?;

            Ok(())
        } else {
            Err(KvsError::KeyNotFound.into())
        }
    }
}
