use libsquashfs1_sys::ffi::{sqfs_frag_table_lookup, sqfs_frag_table_write};

use crate::compressor::Compressor;
use crate::ffi::{
    sqfs_frag_table_create, sqfs_frag_table_get_size, sqfs_frag_table_read, sqfs_frag_table_t,
    sqfs_fragment_t,
};
use crate::file::File;
use crate::super_block::SuperBlock;
use crate::{InternalBlockSize, ManagedPointer, Result};

/// Safe wrapper for [sqfs_frag_table_t]
pub struct FragmentTable {
    ptr: ManagedPointer<sqfs_frag_table_t>,
}

impl FragmentTable {
    /// Safe wrapper for [sqfs_frag_table_create]
    pub fn new() -> Result<Self> {
        let init = || unsafe { sqfs_frag_table_create(0) };

        ManagedPointer::check_null(&init, "Creating FragmentTable", crate::sqfs_destroy)
            .map(|ptr| Self { ptr })
    }

    /// Safe wrapper for [sqfs_frag_table_read]
    pub fn read(file: &File, super_block: &SuperBlock, compressor: &Compressor) -> Result<Self> {
        let fragment_table = Self::new()?;

        let code = unsafe {
            sqfs_frag_table_read(
                fragment_table.ptr.as_ptr(),
                file.ptr().as_ptr(),
                super_block.ptr(),
                compressor.ptr().as_ptr(),
            )
        };

        crate::sqfs_check(code, "Reading FragmentTable")?;

        Ok(fragment_table)
    }

    /// Safe wrapper for [sqfs_frag_table_write]
    pub fn write(
        &self,
        file: &File,
        super_block: &mut SuperBlock,
        compressor: &Compressor,
    ) -> Result<()> {
        let code = unsafe {
            sqfs_frag_table_write(
                self.ptr.as_ptr(),
                file.ptr().as_ptr(),
                super_block.ptr_mut(),
                compressor.ptr().as_ptr(),
            )
        };

        crate::sqfs_check(code, "Writing FragmentTable to file").map(|_| ())
    }

    /// Safe wrapper for [sqfs_frag_table_lookup]
    pub fn lookup(&self, index: u32) -> Result<Fragment> {
        let init = |ptr| unsafe { sqfs_frag_table_lookup(self.ptr.as_ptr(), index, ptr) };

        crate::sqfs_init(&init, &format!("Looking up fragment with index({})", index))
            .map(|fragment| Fragment { fragment })
    }

    // TODO: implement append
    // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/structsqfs__frag__table__t.html#ae8c0eac1e9026c04e26f7ecd5d302a9f

    // TODO: implement set
    // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/structsqfs__frag__table__t.html#a84064112b146872e9f6460d04c12c8a7

    /// Safe wrapper for [sqfs_frag_table_get_size]
    pub fn get_size(&self) -> usize {
        unsafe { sqfs_frag_table_get_size(self.ptr.as_ptr()) }
    }

    pub fn fragments(&self) -> Fragments {
        Fragments::new(self)
    }
}

/// Iterator over the fragment table.
pub struct Fragments<'a> {
    index: u32,
    fragment_table: &'a FragmentTable,
}

impl<'a> Fragments<'a> {
    fn new(fragment_table: &'a FragmentTable) -> Self {
        Self {
            fragment_table,
            index: 0,
        }
    }
}

impl<'a> Iterator for Fragments<'a> {
    type Item = Fragment;

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index;
        self.index += 1;

        self.fragment_table.lookup(current_index).ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.fragment_table.get_size()
            - usize::try_from(self.index).expect("u32 should fit in usize");

        (left, Some(left))
    }
}

/// Safe wrapper for [sqfs_fragment_t]
#[derive(Debug, Default, Clone)]
pub struct Fragment {
    fragment: sqfs_fragment_t,
}

impl Fragment {
    pub fn start_offset(&self) -> u64 {
        self.fragment.start_offset
    }
    pub fn pad0(&self) -> u32 {
        self.fragment.pad0
    }
}

impl InternalBlockSize for Fragment {
    fn internal_size(&self) -> u32 {
        self.fragment.size
    }
}
