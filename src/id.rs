use crate::compressor::Compressor;
use crate::ffi::{sqfs_id_table_create, sqfs_id_table_read, sqfs_id_table_t};
use crate::file::File;
use crate::super_block::SuperBlock;
use crate::ManagedPointer;
use crate::Result;

/// Safe wrapper for [sqfs_id_table_t]
pub struct IdTable {
    ptr: ManagedPointer<sqfs_id_table_t>,
}

impl IdTable {
    /// Safe wrapper for [sqfs_id_table_create]
    pub fn new() -> Result<Self> {
        let init = || unsafe { sqfs_id_table_create(0) };

        ManagedPointer::check_null(&init, "Creating IdTable", crate::sqfs_destroy)
            .map(|ptr| Self { ptr })
    }

    // TODO: implement id_to_index
    // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/structsqfs__id__table__t.html#a234b2ddaef38b57615c15829308bc74f

    // TODO: implement write
    // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/structsqfs__id__table__t.html#a99de5e09c14ffa6e9ab72c554cd99b72

    /// Safe wrapper for [sqfs_id_table_read]
    pub fn read(file: &File, super_block: &SuperBlock, compressor: &Compressor) -> Result<Self> {
        let id_table = Self::new()?;

        let code = unsafe {
            sqfs_id_table_read(
                id_table.ptr.as_ptr(),
                file.ptr().as_ptr(),
                super_block.ptr(),
                compressor.ptr().as_ptr(),
            )
        };

        crate::sqfs_check(code, "Reading IdTable")?;

        Ok(id_table)
    }

    // TODO: implement index_to_id
    // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/structsqfs__id__table__t.html#aa133138a753b1405778ec515336ea28b

    pub(crate) fn ptr(&self) -> &ManagedPointer<sqfs_id_table_t> {
        &self.ptr
    }
}
