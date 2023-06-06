use std::{collections::{HashMap, BTreeMap}, fs::{self, File}, path::{Path, PathBuf}, ops::Range, io::{Seek, BufReader, Read, SeekFrom, Write, BufWriter}, ffi::OsStr};

use crate::error::{KvsError, Result};


enum Command {
    Set { key : String, value : String },
    Remove { key : String } 
}

struct CommandPos {
    gen : u64,
    pos : u64,
    len : u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos { gen, pos: range.start, len: range.end - range.start }
    }
}

struct BufReaderWithPos<R: Read + Seek> {
    reader : BufReader<R>,
    pos : u64,
}

impl<R: Read + Seek> BufReaderWithPos<R>{
    fn new(mut inner : R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader : BufReader::new(inner),
            pos
        })
    }
}

impl<R:Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R:Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos : SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriteWithPos<W : Write + Seek> {
    writer : BufWriter<W>,
    pos : u64,
}

impl<W: Write + Seek> BufWriteWithPos<W> {
    fn new(mut inner : W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriteWithPos {
            writer : BufWriter::new(inner),
            pos
        })
    }
}

impl<W:Write + Seek> Write for BufWriteWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W:Write + Seek> Seek for BufWriteWithPos<W> {
    fn seek(&mut self, pos : SeekFrom) -> std::io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}

pub struct KvStore {
    path : PathBuf,
    readers : HashMap<u64, BufReaderWithPos<File>>,
    writer : BufWriteWithPos<File>,
    current_pos : u64,
    index : BTreeMap<String, CommandPos>,
    uncompacted : u64,
}

impl KvStore {
    pub fn open(path : impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        
        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();

        let gen_list = sorted_gen_list(&path)?;
        let mut uncomacted = 0;

        for &gen in &gen_list {
            let mut readr = BufWriteWithPos::new(File::open(log_path()));

        }

        unimplemented!()
    }
}


fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {
    let mut gen_list: Vec<u64> = 
        // 读fs::read_dir 取目录并返回目录项迭代器
        fs::read_dir(&path)?
        // flat_map 返回转换元素的迭代器
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        // filter 筛选元素返回迭代器
        // extension 提取文件扩展名
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    gen_list.sort_unstable();
    Ok(gen_list)
}


fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
}