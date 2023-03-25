#[derive(Debug)]
pub struct BlockId {
    filename: String,
    blknum: i32
}

impl BlockId {
    pub fn new(filename: String, blknum: i32) -> Self {
        BlockId {filename: filename, blknum: blknum}
    }

}