use std::ffi::OsString;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::ptr;
use std::ptr::NonNull;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use thiserror::Error;

use ffi::sqfs_object_t;
use ffi::SQFS_ERROR::*;
pub use libsquashfs1_sys::ffi;

pub mod blocks;
pub mod compressor;
pub mod directory_reader;
pub mod file;
pub mod fragment;
pub mod id;
pub mod inode;
pub mod super_block;

type BoxedError = Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

//                                  Apache License
//                            Version 2.0, January 2004
//                         http://www.apache.org/licenses/
//
//    TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION
//
//    1. Definitions.
//
//       "License" shall mean the terms and conditions for use, reproduction,
//       and distribution as defined by Sections 1 through 9 of this document.
//
//       "Licensor" shall mean the copyright owner or entity authorized by
//       the copyright owner that is granting the License.
//
//       "Legal Entity" shall mean the union of the acting entity and all
//       other entities that control, are controlled by, or are under common
//       control with that entity. For the purposes of this definition,
//       "control" means (i) the power, direct or indirect, to cause the
//       direction or management of such entity, whether by contract or
//       otherwise, or (ii) ownership of fifty percent (50%) or more of the
//       outstanding shares, or (iii) beneficial ownership of such entity.
//
//       "You" (or "Your") shall mean an individual or Legal Entity
//       exercising permissions granted by this License.
//
//       "Source" form shall mean the preferred form for making modifications,
//       including but not limited to software source code, documentation
//       source, and configuration files.
//
//       "Object" form shall mean any form resulting from mechanical
//       transformation or translation of a Source form, including but
//       not limited to compiled object code, generated documentation,
//       and conversions to other media types.
//
//       "Work" shall mean the work of authorship, whether in Source or
//       Object form, made available under the License, as indicated by a
//       copyright notice that is included in or attached to the work
//       (an example is provided in the Appendix below).
//
//       "Derivative Works" shall mean any work, whether in Source or Object
//       form, that is based on (or derived from) the Work and for which the
//       editorial revisions, annotations, elaborations, or other modifications
//       represent, as a whole, an original work of authorship. For the purposes
//       of this License, Derivative Works shall not include works that remain
//       separable from, or merely link (or bind by name) to the interfaces of,
//       the Work and Derivative Works thereof.
//
//       "Contribution" shall mean any work of authorship, including
//       the original version of the Work and any modifications or additions
//       to that Work or Derivative Works thereof, that is intentionally
//       submitted to Licensor for inclusion in the Work by the copyright owner
//       or by an individual or Legal Entity authorized to submit on behalf of
//       the copyright owner. For the purposes of this definition, "submitted"
//       means any form of electronic, verbal, or written communication sent
//       to the Licensor or its representatives, including but not limited to
//       communication on electronic mailing lists, source code control systems,
//       and issue tracking systems that are managed by, or on behalf of, the
//       Licensor for the purpose of discussing and improving the Work, but
//       excluding communication that is conspicuously marked or otherwise
//       designated in writing by the copyright owner as "Not a Contribution."
//
//       "Contributor" shall mean Licensor and any individual or Legal Entity
//       on behalf of whom a Contribution has been received by Licensor and
//       subsequently incorporated within the Work.
//
//    2. Grant of Copyright License. Subject to the terms and conditions of
//       this License, each Contributor hereby grants to You a perpetual,
//       worldwide, non-exclusive, no-charge, royalty-free, irrevocable
//       copyright license to reproduce, prepare Derivative Works of,
//       publicly display, publicly perform, sublicense, and distribute the
//       Work and such Derivative Works in Source or Object form.
//
//    3. Grant of Patent License. Subject to the terms and conditions of
//       this License, each Contributor hereby grants to You a perpetual,
//       worldwide, non-exclusive, no-charge, royalty-free, irrevocable
//       (except as stated in this section) patent license to make, have made,
//       use, offer to sell, sell, import, and otherwise transfer the Work,
//       where such license applies only to those patent claims licensable
//       by such Contributor that are necessarily infringed by their
//       Contribution(s) alone or by combination of their Contribution(s)
//       with the Work to which such Contribution(s) was submitted. If You
//       institute patent litigation against any entity (including a
//       cross-claim or counterclaim in a lawsuit) alleging that the Work
//       or a Contribution incorporated within the Work constitutes direct
//       or contributory patent infringement, then any patent licenses
//       granted to You under this License for that Work shall terminate
//       as of the date such litigation is filed.
//
//    4. Redistribution. You may reproduce and distribute copies of the
//       Work or Derivative Works thereof in any medium, with or without
//       modifications, and in Source or Object form, provided that You
//       meet the following conditions:
//
//       (a) You must give any other recipients of the Work or
//           Derivative Works a copy of this License; and
//
//       (b) You must cause any modified files to carry prominent notices
//           stating that You changed the files; and
//
//       (c) You must retain, in the Source form of any Derivative Works
//           that You distribute, all copyright, patent, trademark, and
//           attribution notices from the Source form of the Work,
//           excluding those notices that do not pertain to any part of
//           the Derivative Works; and
//
//       (d) If the Work includes a "NOTICE" text file as part of its
//           distribution, then any Derivative Works that You distribute must
//           include a readable copy of the attribution notices contained
//           within such NOTICE file, excluding those notices that do not
//           pertain to any part of the Derivative Works, in at least one
//           of the following places: within a NOTICE text file distributed
//           as part of the Derivative Works; within the Source form or
//           documentation, if provided along with the Derivative Works; or,
//           within a display generated by the Derivative Works, if and
//           wherever such third-party notices normally appear. The contents
//           of the NOTICE file are for informational purposes only and
//           do not modify the License. You may add Your own attribution
//           notices within Derivative Works that You distribute, alongside
//           or as an addendum to the NOTICE text from the Work, provided
//           that such additional attribution notices cannot be construed
//           as modifying the License.
//
//       You may add Your own copyright statement to Your modifications and
//       may provide additional or different license terms and conditions
//       for use, reproduction, or distribution of Your modifications, or
//       for any such Derivative Works as a whole, provided Your use,
//       reproduction, and distribution of the Work otherwise complies with
//       the conditions stated in this License.
//
//    5. Submission of Contributions. Unless You explicitly state otherwise,
//       any Contribution intentionally submitted for inclusion in the Work
//       by You to the Licensor shall be under the terms and conditions of
//       this License, without any additional terms or conditions.
//       Notwithstanding the above, nothing herein shall supersede or modify
//       the terms of any separate license agreement you may have executed
//       with Licensor regarding such Contributions.
//
//    6. Trademarks. This License does not grant permission to use the trade
//       names, trademarks, service marks, or product names of the Licensor,
//       except as required for reasonable and customary use in describing the
//       origin of the Work and reproducing the content of the NOTICE file.
//
//    7. Disclaimer of Warranty. Unless required by applicable law or
//       agreed to in writing, Licensor provides the Work (and each
//       Contributor provides its Contributions) on an "AS IS" BASIS,
//       WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
//       implied, including, without limitation, any warranties or conditions
//       of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
//       PARTICULAR PURPOSE. You are solely responsible for determining the
//       appropriateness of using or redistributing the Work and assume any
//       risks associated with Your exercise of permissions under this License.
//
//    8. Limitation of Liability. In no event and under no legal theory,
//       whether in tort (including negligence), contract, or otherwise,
//       unless required by applicable law (such as deliberate and grossly
//       negligent acts) or agreed to in writing, shall any Contributor be
//       liable to You for damages, including any direct, indirect, special,
//       incidental, or consequential damages of any character arising as a
//       result of this License or out of the use or inability to use the
//       Work (including but not limited to damages for loss of goodwill,
//       work stoppage, computer failure or malfunction, or any and all
//       other commercial damages or losses), even if such Contributor
//       has been advised of the possibility of such damages.
//
//    9. Accepting Warranty or Additional Liability. While redistributing
//       the Work or Derivative Works thereof, You may choose to offer,
//       and charge a fee for, acceptance of support, warranty, indemnity,
//       or other liability obligations and/or rights consistent with this
//       License. However, in accepting such obligations, You may act only
//       on Your own behalf and on Your sole responsibility, not on behalf
//       of any other Contributor, and only if You agree to indemnify,
//       defend, and hold each Contributor harmless for any liability
//       incurred by, or claims asserted against, such Contributor by reason
//       of your accepting any such warranty or additional liability.
//
//    END OF TERMS AND CONDITIONS

// Changed type names
// Changed constant names used
// MModified ManagedPtr to be NonNull friendly
// Added error types
// Removed error types

/// Errors raised by the underlying library.
///
/// This error type reflects all errors raised by the squashfs-tools-ng library.  This should
/// always be wrapped in a [`SquashfsError`] before being returned from any of the functions in
/// this library.
#[derive(Error, Debug, FromPrimitive)]
#[repr(i32)]
pub enum LibError {
    #[error("Failed to allocate memory")]
    Alloc = SQFS_ERROR_ALLOC,
    #[error("Generic I/O failure")]
    Io = SQFS_ERROR_IO,
    #[error("Compressor failed to extract data")]
    Compressor = SQFS_ERROR_COMPRESSOR,
    #[error("Internal error")]
    Internal = SQFS_ERROR_INTERNAL,
    #[error("Archive file appears to be corrupted")]
    Corrupted = SQFS_ERROR_CORRUPTED,
    #[error("Unsupported feature used")]
    Unsupported = SQFS_ERROR_UNSUPPORTED,
    #[error("Archive would overflow memory")]
    Overflow = SQFS_ERROR_OVERFLOW,
    #[error("Out-of-bounds access attempted")]
    OutOfBounds = SQFS_ERROR_OUT_OF_BOUNDS,
    #[error("Superblock magic number incorrect")]
    SuperMagic = SFQS_ERROR_SUPER_MAGIC,
    #[error("Unsupported archive version")]
    SuperVersion = SFQS_ERROR_SUPER_VERSION,
    #[error("Archive block size is invalid")]
    SuperBlockSize = SQFS_ERROR_SUPER_BLOCK_SIZE,
    #[error("Not a directory")]
    NotDir = SQFS_ERROR_NOT_DIR,
    #[error("Path does not exist")]
    NoEntry = SQFS_ERROR_NO_ENTRY,
    #[error("Hard link loop detected")]
    LinkLoop = SQFS_ERROR_LINK_LOOP,
    #[error("Not a regular file")]
    NotFile = SQFS_ERROR_NOT_FILE,
    #[error("Invalid argument passed")]
    ArgInvalid = SQFS_ERROR_ARG_INVALID,
    #[error("Library operations performed in incorrect order")]
    Sequence = SQFS_ERROR_SEQUENCE,
}

/// Errors encountered while reading or writing an archive.
///
/// This wraps all errors that might be encountered by the library during its normal course of
/// operation.
#[derive(Error, Debug)]
pub enum SqfsError {
    #[error("Input contains an invalid null character")]
    NullInput(#[from] std::ffi::NulError),
    #[error("Encoded string is not valid UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Path is not valid UTF-8: {0}")]
    PathToStr(PathBuf),
    #[error("OS string is not valid UTF-8")]
    OsUtf8(OsString),
    #[error("{0}: {1}")]
    LibraryError(String, LibError),
    #[error("{0}: Unknown error {1} in SquashFS library")]
    UnknownLibraryError(String, i32),
    #[error("{0}: Squashfs library did not return expected value")]
    LibraryReturnError(String),
    #[error("{0}")]
    LibraryNullError(String),
    #[error("Symbolic link chain exceeds {0} elements")]
    LinkChain(i32),
    #[error("Symbolic link loop detected containing {0}")]
    LinkLoop(PathBuf),
    #[error("Dangling symbolic link from {0} to {1}")]
    DanglingLink(PathBuf, PathBuf),
    #[error("{0} is type {1}, not {2}")]
    WrongType(String, String, String),
    #[error("Tried to copy an object that can't be copied")]
    Copy,
    #[error("Tried to get parent of a node with an unknown path")]
    NoPath,
    #[error("Inode index {0} is not within limits 1..{1}")]
    Range(u64, u64),
    #[error("Couldn't read file: {0}")]
    Read(#[from] std::io::Error),
    #[error("The filesystem does not support the feature: {0}")]
    Unsupported(String),
    #[error("Memory mapping failed: {0}")]
    Mmap(std::io::Error),
    #[error("Couldn't get the current system time: {0}")]
    Time(#[from] std::time::SystemTimeError),
    #[error("Refusing to create empty archive")]
    Empty,
    #[error("Tried to write parent directory before child node {0}")]
    WriteOrder(u32),
    #[error("Tried to write unknown or unsupported file type")]
    WriteType(std::fs::FileType),
    #[error("Callback returned an error")]
    WrappedError(BoxedError),
    #[error("Failed to retrieve xattrs for {0}: {1}")]
    Xattr(PathBuf, std::io::Error),
    #[error("Tried to add files to a writer that was already finished")]
    Finished,
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type returned by SquashFS library operations.
pub type Result<T> = std::result::Result<T, SqfsError>;

fn sqfs_check(code: i32, desc: &str) -> Result<i32> {
    match code {
        i if i >= 0 => Ok(i),
        i => match FromPrimitive::from_i32(i) {
            Some(e) => Err(SqfsError::LibraryError(desc.to_string(), e)),
            None => Err(SqfsError::UnknownLibraryError(desc.to_string(), i)),
        },
    }
}

fn sqfs_destroy<T>(x: *mut T) {
    unsafe {
        let obj = x as *mut sqfs_object_t;
        ((*obj)
            .destroy
            .expect("SquashFS object did not provide a destroy callback"))(obj);
    }
}

const NO_XATTRS: u32 = 0xffffffff;
const LOCK_ERR: &str = "A thread panicked while holding a lock";
// Because poisoned locks only happen when a thread panics, we probably want to panic too.
const LINK_MAX: i32 = 1000;
const BLOCK_BUF_SIZE: usize = 4096;
const PAD_TO: usize = 4096;

#[derive(Debug)]
struct ManagedPointer<T> {
    ptr: NonNull<T>,
    destroy: fn(*mut T),
}

impl<T> ManagedPointer<T> {
    fn new(ptr: NonNull<T>, destroy: fn(*mut T)) -> Self {
        Self { ptr, destroy }
    }
}

impl<T> std::ops::Deref for ManagedPointer<T> {
    type Target = NonNull<T>;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T> std::ops::DerefMut for ManagedPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}

impl<T> Drop for ManagedPointer<T> {
    fn drop(&mut self) {
        (self.destroy)(self.as_ptr())
    }
}

fn sqfs_init<T>(init: &dyn Fn(*mut T) -> i32, err: &str) -> Result<T> {
    let mut ret: MaybeUninit<T> = MaybeUninit::uninit();
    sqfs_check(init(ret.as_mut_ptr()), err)?;
    Ok(unsafe { ret.assume_init() })
}

impl<T> ManagedPointer<T> {
    fn init_ptr(
        init: &dyn Fn(*mut *mut T) -> i32,
        err: &str,
        destroy: fn(*mut T),
    ) -> Result<ManagedPointer<T>> {
        let mut ret: *mut T = ptr::null_mut();

        sqfs_check(init(&mut ret), err)?;

        NonNull::new(ret)
            .ok_or(SqfsError::LibraryReturnError(err.to_string()))
            .map(|ptr| Self { ptr, destroy })
    }

    fn check_null(
        init: &dyn Fn() -> *mut T,
        err: &str,
        destroy: fn(*mut T),
    ) -> Result<ManagedPointer<T>> {
        NonNull::new(init())
            .ok_or(SqfsError::LibraryNullError(err.to_string()))
            .map(|ptr| Self { ptr, destroy })
    }
}

fn path_to_c_str<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let path = path.as_ref();
    let mut buf = Vec::new();

    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        buf.extend(path.as_os_str().as_bytes());
        buf.push(0);
    }

    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        buf.extend(
            path.as_os_str()
                .encode_wide()
                .chain(Some(0))
                .map(|b| {
                    let b = b.to_ne_bytes();
                    b.get(0).map(|s| *s).into_iter().chain(b.get(1).map(|s| *s))
                })
                .flatten(),
        );
    }

    buf.into_boxed_slice()
}
