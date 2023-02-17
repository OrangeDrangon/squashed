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

    pub fn compressed(&self) -> bool {
        ((self.size) & (1 << 24)) == 0
    }

    pub fn on_disk_size(&self) -> u32 {
        (self.size) & ((1 << 24) - 1)
    }

    pub fn is_spare(&self) -> bool {
        self.on_disk_size() == 0
    }
}
