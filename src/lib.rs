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
    op_count: u32,
}

impl KvStore {
    pub fn open(dir_path: &Path) -> Result<KvStore> {
        let mut kv = KvStore {
            path: dir_path.to_path_buf(),
            index: std::collections::HashMap::new(),
            op_count: 0,
        };

        std::fs::create_dir_all(&kv.path)?;
        match File::create_new(Path::join(&kv.path, "wal")) {
            Ok(_) => {}
            _ => {
                kv.build_index()?;
            }
        }

        Ok(kv)
    }

    fn wal_path(&self) -> PathBuf {
        Path::new(&self.path).join("wal")
    }

    fn append_to_wal(&mut self, operation: Operation) -> Result<()> {
        self.append_to_path(operation, self.wal_path())?;

        Ok(())
    }

    fn append_to_path(&mut self, operation: Operation, path: PathBuf) -> Result<()> {
        let mut file = File::options().append(true).open(&path)?;
        let serialized_op = serde_json::to_string(&operation)?;

        let offset = file.seek(SeekFrom::End(0))?;
        let length = serialized_op.len() as u32;

        file.write_all(&length.to_le_bytes())?;
        file.write_all(serialized_op.as_bytes())?;

        match operation {
            Operation::Rm { key } => {
                self.index.remove(&key);
            }
            Operation::Set { key, .. } => {
                self.index.insert(key, LogPointer { offset, length });
            }
        }

        Ok(())
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
                            offset: current_offset.clone(),
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
        self.op_count += 1;

        if self.op_count >= 20 {
            self.compact()?;
            self.op_count = 0;
        }

        return Ok(());
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            self.append_to_wal(Operation::Rm { key })?;
            Ok(())
        } else {
            Err(KvsError::KeyNotFound.into())
        }
    }

    pub fn compact(&mut self) -> Result<()> {
        let wal = File::open(self.wal_path())?;
        let mut reader = BufReader::new(&wal);

        let new_wal_path = Path::join(&self.path, "new_wal");
        let mut new_wal = File::options()
            .append(true)
            .create(true)
            .open(&new_wal_path)?;

        for (_, log_pointer) in self.index.iter() {
            reader.seek(SeekFrom::Start(log_pointer.offset + 4))?;

            let mut data = vec![0u8; log_pointer.length as usize];
            reader.read_exact(&mut data)?;

            let op: Operation = serde_json::from_slice(&data)?;

            new_wal.write_all(&log_pointer.length.to_le_bytes())?;
            new_wal.write_all(&serde_json::to_string(&op)?.as_bytes())?;
        }

        std::fs::rename(new_wal_path, self.wal_path())?;
        self.build_index()?;

        Ok(())
    }

    fn wal_operation(&self, wal: &mut File, pointer: &LogPointer) -> Result<Vec<u8>> {
        let mut data = vec![0u8; (pointer.length + 4) as usize];
        wal.seek(SeekFrom::Start(pointer.offset))?;
        wal.read_exact(&mut data)?;

        Ok(data)
    }
}
