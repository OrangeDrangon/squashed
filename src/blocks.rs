use crate::InternalBlockSize;

#[derive(Debug, Clone)]
pub struct Block {
    start_offset: u64,
    size: u32,
}

impl Block {
    pub(crate) fn new(start_offset: u64, size: u32) -> Self {
        Self { start_offset, size }
    }

    pub fn start_offset(&self) -> u64 {
        self.start_offset
    }
}

impl InternalBlockSize for Block {
    fn internal_size(&self) -> u32 {
        self.size
    }
}
