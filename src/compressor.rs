use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

use crate::ffi::SQFS_COMPRESSOR::*;
pub use crate::ffi::SQFS_COMP_FLAG;
use crate::ffi::{
    sqfs_compressor_config_init, sqfs_compressor_config_t, sqfs_compressor_create,
    sqfs_compressor_t,
};
use crate::super_block::SuperBlock;
use crate::{ManagedPointer, Result};

/// The type of compression used in the image.
#[derive(Debug, FromPrimitive, ToPrimitive, Eq, PartialEq)]
#[repr(u32)]
pub enum CompressorType {
    GZip = SQFS_COMP_GZIP,
    Lzma = SQFS_COMP_LZMA,
    Lzo = SQFS_COMP_LZO,
    Xz = SQFS_COMP_XZ,
    Lz4 = SQFS_COMP_LZ4,
    Zstd = SQFS_COMP_ZSTD,
}

/// Safe warapper for [sqfs_compressor_config_t]
pub struct CompressorConfig {
    compressor_config: sqfs_compressor_config_t,
}

impl CompressorConfig {
    /// Safe wrapper for [sqfs_compressor_config_init]
    pub fn new(super_block: &SuperBlock, flags: SQFS_COMP_FLAG) -> Result<Self> {
        let init = |ptr| unsafe {
            sqfs_compressor_config_init(
                ptr,
                super_block
                    .compression_id()
                    .to_u32()
                    .expect("invalid compression type"),
                usize::try_from(super_block.block_size()).expect("blocksize should fit in a usize"),
                u16::try_from(flags.0).expect("flags should fit in u16"),
            )
        };

        let compressor_config = crate::sqfs_init(&init, "Initializing CompressorConfig")?;

        Ok(Self { compressor_config })
    }
}

/// Safe wrapper for [sqfs_compressor_t]
pub struct Compressor {
    ptr: ManagedPointer<sqfs_compressor_t>,
}

impl Compressor {
    /// Safe wrapper for [sqfs_compressor_create]
    pub fn new(compressor_config: &CompressorConfig) -> Result<Self> {
        let init =
            |ptr| unsafe { sqfs_compressor_create(&compressor_config.compressor_config, ptr) };

        ManagedPointer::init_ptr(&init, "Creating Compressor", crate::sqfs_destroy)
            .map(|ptr| Self { ptr })
    }

    // TODO: implement get_configuration
    // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/structsqfs__compressor__t.html#a7cbb0194b79880c229bcec8241a5f19b

    // TODO: implement read_options
    // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/structsqfs__compressor__t.html#a15e937a2611ef6d9abc7f77fae2c11c8

    // TODO: implement write_options
    // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/structsqfs__compressor__t.html#a956781a777f8e304331d8b02295ded9e

    pub(crate) fn ptr(&self) -> &ManagedPointer<sqfs_compressor_t> {
        &self.ptr
    }
}
