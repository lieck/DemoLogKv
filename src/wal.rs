use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
};

use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum LogEntry {
    Set { key: String, value: String },
    Rm { key: String },
}

#[derive(Debug)]
pub struct LogEntryIdx {
    pub idx: u64,
    pub entry: LogEntry,
}

pub struct WalLog {
    file_name: String,
    file: File,
}

impl WalLog {
    pub fn new(file_name: String) -> Self {
        let mut file_name = file_name;
        if file_name.is_empty() {
            file_name = String::from("log.txt");
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_name.clone())
            .unwrap();
        WalLog { file_name, file }
    }

    pub fn set_file_name(&mut self, new_name: String) {
        self.file_name = new_name;
    }

    pub fn write(&mut self, ent: &LogEntry) -> Result<u64> {
        let mut serialized = serde_json::to_string(&ent)
            .or_else(|err| Result::Err(KvsError::Serde(err)))
            .unwrap();
        serialized.push('\n');

        let pos = self.file.stream_position()?;

        _ = self
            .file
            .write_all(serialized.as_bytes())
            .or_else(|err| Result::Err(KvsError::Io(err)));

        Result::Ok(pos)
    }

    pub fn read_all(&self) -> Result<Vec<LogEntryIdx>> {
        let mut ret = Vec::new();

        let f = File::open(&self.file_name)
            .or_else(|err| Result::Err(KvsError::Io(err)))
            .unwrap();

        let mut reader = BufReader::new(f);
        let mut buffer = String::new();

        loop {
            buffer.clear();

            let pos = reader.stream_position()?;

            let bytes_read = reader.read_line(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            let e: LogEntry = serde_json::from_str(&buffer)
                .or_else(|err| Result::Err(KvsError::Serde(err)))
                .unwrap();

            ret.push(LogEntryIdx { idx: pos, entry: e });
        }
        Ok(ret)
    }

    pub fn read_line_at(&self, pos: u64) -> Result<String> {
        let file = File::open(&self.file_name)?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(pos))?;
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let ret: LogEntry = serde_json::from_str(&line)
            .or_else(|err| Result::Err(KvsError::Serde(err)))
            .unwrap();

        match ret {
            LogEntry::Set { key: _, value } => Result::Ok(value),
            _ => Result::Err(KvsError::KeyNotFound),
        }
    }

    pub fn compacting(&mut self) {}
}
