#[derive(Debug, Clone, Copy)]
pub struct Page {
    size: u32
}

impl Page {
    pub fn new(size:u32) -> Self{
        Page {size: size}
    }

    pub fn max_length(l: usize) -> usize {
        todo!();
    }

    pub fn set_string(self, pos: usize, s: String) {
        todo!();
    }

    pub fn set_int(self, s: usize, u: usize) {
        todo!();
    }

    pub fn get_int(self, pos: usize) -> usize {
        todo!();
    }

    pub fn get_string(self, pos: usize) -> usize {
        todo!();
    }
}