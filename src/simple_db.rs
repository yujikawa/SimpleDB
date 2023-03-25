use std::fs::File;

use crate::file_manager::file_manager::FileManager;

pub struct SimpleDB {
    file: File,
    a: i8,
    b: i8,
}



impl SimpleDB {
    pub fn new(filename: String, a: usize, b: usize) -> Self {
        todo!();
    }

    pub fn file_mgr(self) -> FileManager {
        todo!()
    }
}


