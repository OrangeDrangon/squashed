use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

use libsquashfs1_sys::ffi::SQFS_COMPRESSOR::{
    SQFS_COMP_GZIP, SQFS_COMP_LZ4, SQFS_COMP_LZMA, SQFS_COMP_LZO, SQFS_COMP_XZ, SQFS_COMP_ZSTD,
};

use crate::ffi::{
    sqfs_compressor_config_init, sqfs_compressor_config_t, sqfs_compressor_create,
    sqfs_compressor_t,
};
use crate::super_block::SuperBlock;
use crate::{ManagedPointer, Result};

/// The type of compression used in the image.
#[derive(Debug, FromPrimitive, ToPrimitive)]
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
    pub fn new(super_block: &SuperBlock) -> Result<Self> {
        let mut compressor_config = Default::default();

        let code = unsafe {
            sqfs_compressor_config_init(
                &mut compressor_config,
                super_block
                    .compression_id()
                    .to_u32()
                    .expect("invalid compression type"),
                super_block.block_size() as usize,
                super_block.flags(),
            )
        };

        crate::sqfs_check(code, "Initializing CompressorConfig")?;

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
