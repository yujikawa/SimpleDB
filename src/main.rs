
mod file_manager;
use file_manager::block_id::BlockId;
use file_manager::page::Page;

mod simple_db;
use simple_db::SimpleDB;


fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_manager() {
        let db = SimpleDB::new(String::from("testfile"), 400, 8);
        let fm = db.file_mgr();
        
        let blk = BlockId::new(String::from("testfile"), 2);
        let p1 = Page::new(fm.block_size());
        let pos1 = 88;
        p1.set_string(pos1, String::from("abcdefghijklm"));
        let size = Page::max_length(String::from("abcdefghijklm").len());
        let pos2 = pos1 + size;
        p1.set_int(pos2, 345);
        fm.write(&blk, &p1);
    
        let p2 = Page::new(fm.block_size());
        fm.read(&blk, &p2);
        println!("{} {}", pos2, p2.get_int(pos2));
        println!("{} {}", pos1, p2.get_string(pos1));
    }
}