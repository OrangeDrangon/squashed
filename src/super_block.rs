use num_traits::FromPrimitive;

use crate::compressor::CompressorType;
use crate::ffi::{sqfs_super_read, sqfs_super_t, sqfs_super_write};
use crate::file::File;
use crate::Result;

/// Safe wrapper for [sqfs_super_t]
pub struct SuperBlock {
    super_block: sqfs_super_t,
}

impl SuperBlock {
    /// Safe wrapper for [sqfs_super_read]
    pub fn read(file: &File) -> Result<Self> {
        let mut super_block = Default::default();

        let code = unsafe { sqfs_super_read(&mut super_block, file.ptr().as_ptr()) };

        crate::sqfs_check(code, "Reading SuperBlock")?;

        Ok(Self { super_block })
    }

    /// Safe wrapper for [sqfs_super_write]
    pub fn write(&self, file: &File) -> Result<()> {
        let code = unsafe { sqfs_super_write(&self.super_block, file.ptr().as_ptr()) };

        crate::sqfs_check(code, "Writing SuperBlock to file").map(|_| ())
    }

    // TODO: wrap init

    pub fn magic(&self) -> u32 {
        self.super_block.magic
    }
    pub fn inode_count(&self) -> u32 {
        self.super_block.inode_count
    }
    pub fn modification_time(&self) -> u32 {
        self.super_block.modification_time
    }
    pub fn block_size(&self) -> u32 {
        self.super_block.block_size
    }
    pub fn fragment_entry_count(&self) -> u32 {
        self.super_block.fragment_entry_count
    }
    pub fn compression_id(&self) -> CompressorType {
        CompressorType::from_u16(self.super_block.compression_id)
            .expect("invalid compression algorithm")
    }
    pub fn block_log(&self) -> u16 {
        self.super_block.block_log
    }
    pub fn flags(&self) -> u16 {
        self.super_block.flags
    }
    pub fn id_count(&self) -> u16 {
        self.super_block.id_count
    }
    pub fn version_major(&self) -> u16 {
        self.super_block.version_major
    }
    pub fn version_minor(&self) -> u16 {
        self.super_block.version_minor
    }
    pub fn root_inode_ref(&self) -> u64 {
        self.super_block.root_inode_ref
    }
    pub fn bytes_used(&self) -> u64 {
        self.super_block.bytes_used
    }
    pub fn id_table_start(&self) -> u64 {
        self.super_block.id_table_start
    }
    pub fn xattr_id_table_start(&self) -> u64 {
        self.super_block.xattr_id_table_start
    }
    pub fn inode_table_start(&self) -> u64 {
        self.super_block.inode_table_start
    }
    pub fn directory_table_start(&self) -> u64 {
        self.super_block.directory_table_start
    }
    pub fn fragment_table_start(&self) -> u64 {
        self.super_block.fragment_table_start
    }
    pub fn export_table_start(&self) -> u64 {
        self.super_block.export_table_start
    }

    pub(crate) fn ptr(&self) -> &sqfs_super_t {
        &self.super_block
    }

    pub(crate) fn ptr_mut(&mut self) -> &mut sqfs_super_t {
        &mut self.super_block
    }
}
