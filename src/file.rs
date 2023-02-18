use std::ffi::{c_char, c_void, CStr};
use std::path::Path;

pub use crate::ffi::SQFS_FILE_OPEN_FLAGS;
use crate::ffi::{sqfs_file_t, sqfs_open_file};
use crate::{ManagedPointer, Result, SqfsError};

/// Safe wrapper for [sqfs_file_t].
pub struct File {
    ptr: ManagedPointer<sqfs_file_t>,
}

impl File {
    /// Safe wrapper for [sqfs_open_file]
    pub fn open<P: AsRef<Path>>(path: P, flags: SQFS_FILE_OPEN_FLAGS) -> Result<Self> {
        let bytes = crate::path_to_c_str(path);
        let path_ptr = bytes.as_ptr() as *const c_char;
        let init = || unsafe { sqfs_open_file(path_ptr, flags.0) };

        ManagedPointer::check_null(&init, "Opening file", crate::sqfs_destroy)
            .map(|ptr| Self { ptr })
    }

    pub fn read_at(&self, start_offset: u64, size: usize) -> Result<Box<[u8]>> {
        let mut buf = vec![0u8; size].into_boxed_slice();

        let read_at = self
            .as_ref()
            .read_at
            .expect("missing read_at function on the file");

        let code = unsafe {
            read_at(
                self.ptr.as_ptr(),
                start_offset,
                buf.as_mut_ptr() as *mut c_void,
                size,
            )
        };

        crate::sqfs_check(code, "Reading from file")?;

        Ok(buf)
    }

    pub fn write_at(&self, start_offset: u64, buffer: &[u8]) -> Result<()> {
        let write_at = self
            .as_ref()
            .write_at
            .expect("missing write_at function on the file");

        let code = unsafe {
            write_at(
                self.ptr.as_ptr(),
                start_offset,
                buffer.as_ptr() as *const c_void,
                buffer.len(),
            )
        };

        crate::sqfs_check(code, "Writing to file")?;

        Ok(())
    }

    pub fn get_size(&self) -> u64 {
        let get_size = self
            .as_ref()
            .get_size
            .expect("missing get_size function on the file");
        unsafe { get_size(self.ptr.as_ptr()) }
    }

    pub fn truncate(&self, size: u64) -> Result<()> {
        let truncate = self
            .as_ref()
            .truncate
            .expect("missing truncate function on the file");

        let code = unsafe { truncate(self.ptr.as_ptr(), size) };

        crate::sqfs_check(code, "Truncating file").map(|_| ())
    }

    pub(crate) fn ptr(&self) -> &ManagedPointer<sqfs_file_t> {
        &self.ptr
    }

    fn as_ref(&self) -> &sqfs_file_t {
        unsafe { &(*self.ptr.as_ptr()) }
    }
}
