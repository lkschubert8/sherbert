use std::fs;
use std::collections::btree_map::BTreeMap;
use std::io::{Write, Seek, SeekFrom};

pub struct FileMap {
    pub file : fs::File,
    pub map : BTreeMap<String, i32>
}

impl FileMap {
    pub fn add(&mut self, key : String, value : String){

    }

    pub fn get(&mut self, key : String) -> Option<String> {
        //TODO: implement key checking and file random access
        Some(String::from("Hello!"))
    }
}

pub struct KeyValMap {
    pub files : Vec<FileMap>
}

impl KeyValMap {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
