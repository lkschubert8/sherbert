use circular_queue::CircularQueue;
use std::collections::btree_map::BTreeMap;
use std::fs;
use std::fs::{read, File, OpenOptions};
use std::io::{Seek, SeekFrom, Write, Read};

pub struct FileMap {
    pub file: fs::File,
    pub map: BTreeMap<String, u64>,
    pub writes: u32,
    pub end: u64
}

impl FileMap {
    pub fn new(path : String) -> FileMap {
        FileMap {
            file: OpenOptions::new().create(true).write(true).read(true).open(path).expect("File open f'ed up"),
            map: BTreeMap::new(),
            writes: 0,
            end: 0
        }
    }

    pub fn rebuild(file_path: String) -> FileMap {
        FileMap {
            file: fs::File::open(file_path).expect("Temporary Expect Message"),
            map: BTreeMap::new(),
            writes: 0,
            end: 0
        }
    }

    pub fn add(&mut self, key: String, value: String) {
        self.writes += 1;
        let output = format!("{},{:010},{}", key, value.len(), value);
        let position = self.end;
        self.file.seek(SeekFrom::End(0));
        let output_bytes = output.as_bytes();
        let length = output_bytes.len();
        self.file.write(output.as_bytes());
        self.end +=length as u64;
        self.map.insert(key, position);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        match self.map.get(key.as_str()) {
            Some(pos) => {
                self.file.seek(SeekFrom::Start(*pos));
                let size = key.len() + 11;
                let mut buffer: Vec<u8> = vec![0; size];
                self.file.read(&mut buffer).expect("File reading going wrong");
                let field_header = String::from_utf8(buffer).expect("Utf conversion f'ed up");
                let fields : Vec<&str> = field_header.split(",").collect();
                let length = fields[1].parse::<u64>().expect("Can't parse a thing");
                let mut field_buffer : Vec<u8> = vec![0; length as usize];
                self.file.seek(SeekFrom::Current(1));
                self.file.read(&mut field_buffer).expect("Second file read going wrong");
                let mut field = String::from_utf8(field_buffer).expect("Utf conversion f'ed up");
                return Some(field);
            }
            None => return None
        }
    }
}

pub struct KeyValMap {
    pub storage_dir: String,
    pub files: Vec<FileMap>,
    pub file_write_max: u32,
    pub read_depths: CircularQueue<usize>,
}

impl KeyValMap {
    /// Creates a completely new KeyValMap at target location
    pub fn new(new_storage_dir: String) -> KeyValMap {
        let first_map = FileMap::new(String::from(format!("{}_1.dat", new_storage_dir)));
        let mut files = Vec::new();
        files.push(first_map);
        KeyValMap {
            storage_dir: new_storage_dir,
            files,
            file_write_max: 55,
            read_depths: CircularQueue::with_capacity(128),
        }
    }

    /// Rebuilds the KeyValMap from files at a given location
    pub fn rebuild(old_storage_dir: String) -> KeyValMap {
        KeyValMap {
            storage_dir: old_storage_dir,
            files: Vec::new(),
            file_write_max: 55,
            read_depths: CircularQueue::with_capacity(128),
        }
    }

    /// Adds a key to the KeyValMap
    pub fn add(&mut self, key: String, value: String) {
        let mut last_index = &self.files.len() - 1;
        if (&self.files[last_index]).writes >= self.file_write_max {
            let new_file = FileMap::new(format!("{}_{}.dat", self.storage_dir, last_index + 2));
            &self.files.push(new_file);
            last_index += 1;
        }
        &self.files[last_index].add(key, value);
    }

    /// Gets a key if it exists
    pub fn get(&mut self, key: String) -> Option<String> {
        let length = &self.files.len();
        let mut read_depth = 1;
        while length - read_depth >= 0 {
            let file_search = &self.files[length - read_depth].get(key.clone());
            match file_search {
                Some(val) => {
                    &self.read_depths.push(read_depth);
                    return Some(val.to_string());
                }
                None => read_depth += 1,
            }
        }
        &self.read_depths.push(read_depth);
        None
    }
}

#[cfg(test)]
mod tests {
    use self::super::*;
    #[test]
    fn it_works() {}

    #[test]
    fn test_write() {
        let mut filemap = KeyValMap::new(String::from("test_files/sherbert"));
            filemap.add(String::from("test"), String::from("somee contents"));
            filemap.add(String::from("newkey"), String::from("new thing"));

        let new_key_response = filemap.get(String::from("newkey"));
        assert_eq!(new_key_response, Some(String::from("new thing")));
        assert_eq!(filemap.get(String::from("test")), Some(String::from("somee contents")));

    }
}
