use super::{block_id::BlockId, page::Page};



#[derive(Debug, Clone, Copy)]
pub struct FileManager {}

impl FileManager {
    pub fn block_size(self) -> u32 {
        todo!()
    }

    pub fn read(self, blk: &BlockId, p: &Page) {
        todo!()
    }

    pub fn write(self, blk: &BlockId, p: &Page) {
        todo!()
    }
}