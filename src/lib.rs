mod error;
mod wal;

pub use error::{KvsError, Result};
use wal::WalLog;

use std::{collections::HashMap, fs};

pub struct KvStore {
    kv: HashMap<String, u64>,
    wal: WalLog,
    count: u32,
}

impl KvStore {
    pub fn new() -> KvStore {
        let mut s = KvStore {
            kv: HashMap::new(),
            wal: WalLog::new(String::new()),
            count: 0,
        };
        s.init();
        s
    }

    pub fn init(&mut self) {
        for e in self.wal.read_all().unwrap() {
            match e.entry {
                wal::LogEntry::Set { key, value: _ } => {
                    self.kv.insert(key, e.idx);
                }
                wal::LogEntry::Rm { key } => {
                    self.kv.remove(&key);
                }
            }
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        let pos = self
            .wal
            .write(&wal::LogEntry::Set {
                key: key.clone(),
                value: val,
            })
            .unwrap();
        self.kv.insert(key, pos);

        self.count += 1;
        if self.count > 1000 {
            self.wal.compacting();
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        if self.kv.contains_key(&key) {
            let pos = self.kv.get(&key).unwrap();
            let val = self.wal.read_line_at(*pos);

            return match val {
                Ok(val) => Some(val),
                Err(KvsError::KeyNotFound) => None,
                Err(err) => panic!("{}", err.to_string()),
            };
        }
        None
    }

    pub fn remove(&mut self, key: String) {
        self.kv.remove(&key);
        self.wal.write(&wal::LogEntry::Rm { key }).unwrap();

        self.count += 1;
        if self.count > 1000 {
            self.wal.compacting();
        }
    }

    pub fn compacting_wal_log(&mut self) {
        self.count = 0;

        let mut new_wal = WalLog::new(String::from("test.txt"));
        let mut new_kv = HashMap::new();
        for kv in &mut self.kv {
            let value = self.wal.read_line_at(*kv.1).unwrap();
            let pos = new_wal
                .write(&wal::LogEntry::Set {
                    key: kv.0.clone(),
                    value,
                })
                .unwrap();
            new_kv.insert(kv.0.clone(), pos);
        }
        self.kv = new_kv;
        self.wal = new_wal;

        // 更改文件名
        fs::remove_file("log.txt").unwrap();
        fs::rename("test.txt", "log.txt").unwrap();
        self.wal.set_file_name("log.txt".to_string());
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn compacting_wal_log() {
    let mut kvs = KvStore::new();
    for i in 0..100 {
        kvs.set(i.to_string(), i.to_string());
    }
    for i in 0..100 {
        kvs.remove(i.to_string());
    }
    for i in 0..100 {
        kvs.set(i.to_string(), i.to_string());
    }
    kvs.compacting_wal_log();
    for i in 0..100 {
        _ = kvs.get(i.to_string()).unwrap();
    }
}

#[test]
fn remove_file() {
    fs::remove_file("log.txt").unwrap();
}
