pub mod error;
pub use error::KvsError;

use anyhow::Result as AnyResult;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

pub type Result<T> = AnyResult<T, KvsError>;

#[derive(Serialize, Deserialize, Debug)]
enum Operation {
    Set { key: String, value: String },
    Rm { key: String },
}

#[derive(Debug)]
struct LogPointer {
    offset: u64,
    length: u32,
}

pub struct KvStore {
    path: PathBuf,
    index: std::collections::HashMap<String, LogPointer>,
}

impl KvStore {
    pub fn open(dir_path: &Path) -> Result<KvStore> {
        let kv = KvStore {
            path: dir_path.to_path_buf(),
            index: std::collections::HashMap::new(),
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

    fn append_to_wal(&self, op: Operation) -> Result<LogPointer> {
        let mut file = File::options().append(true).open(&self.wal_path())?;
        let serialized_op = serde_json::to_string(&op)?;

        let offset = file.seek(SeekFrom::End(0))?;
        let size = serialized_op.len() as u32;

        file.write_all(&size.to_le_bytes())?;
        file.write_all(serialized_op.as_bytes())?;

        Ok(LogPointer {
            offset,
            length: size,
        })
    }

    fn build_index(&mut self) -> Result<()> {
        let file = File::open(self.wal_path())?;
        let mut reader = BufReader::new(&file);

        let mut current_offset = 0;
        let mut length_buf = [0u8; 4];

        while reader.read_exact(&mut length_buf).is_ok() {
            let length = u32::from_le_bytes(length_buf);

            let mut data = vec![0u8; length as usize];
            reader.read_exact(&mut data)?;

            let operation = serde_json::from_slice(&data)?;

            match operation {
                Operation::Set { key, .. } => {
                    self.index.insert(
                        key,
                        LogPointer {
                            offset: current_offset,
                            length,
                        },
                    );
                }

                Operation::Rm { key } => {
                    self.index.remove(&key);
                }
            }

            current_offset += 4 + length as u64;
        }

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.build_index()?;

        if let Some(log_pointer) = self.index.get(&key) {
            let mut file = File::open(self.wal_path())?;
            file.seek(SeekFrom::Start(log_pointer.offset + 4))?;

            let mut data = vec![0u8; log_pointer.length as usize];
            file.read_exact(&mut data)?;

            let operation = serde_json::from_slice(&data)?;
            if let Operation::Set { value, .. } = operation {
                return Ok(Some(value));
            }
        }

        return Ok(None);
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.append_to_wal(Operation::Set { key, value })?;

        return Ok(());
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.build_index()?;

        if self.index.contains_key(&key) {
            self.append_to_wal(Operation::Rm { key })?;
            Ok(())
        } else {
            Err(KvsError::KeyNotFound.into())
        }
    }
}
