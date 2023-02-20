use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::{slice_from_raw_parts, NonNull};

use crate::BlockAttributes;
use derive_more::Deref;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::blocks::Block;
pub use crate::ffi::SQFS_INODE_MODE;
use crate::ffi::{
    __IncompleteArrayField, sqfs_inode_dev_ext_t, sqfs_inode_dev_t,
    sqfs_inode_generic_t__bindgen_ty_1, sqfs_inode_ipc_ext_t, sqfs_inode_ipc_t,
    sqfs_inode_slink_ext_t, sqfs_inode_slink_t, sqfs_u32,
};
use crate::ffi::{sqfs_inode_dir_ext_t, sqfs_inode_dir_t, sqfs_inode_generic_t};
use crate::ffi::{sqfs_inode_file_ext_t, sqfs_inode_get_file_block_start};
use crate::ffi::{sqfs_inode_file_t, SQFS_INODE_TYPE};

/// Used by [INode] to identify inode type.
#[derive(Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum INodeType {
    Directory = SQFS_INODE_TYPE::SQFS_INODE_DIR,
    File = SQFS_INODE_TYPE::SQFS_INODE_FILE,
    SymbolicLink = SQFS_INODE_TYPE::SQFS_INODE_SLINK,
    BlockDevice = SQFS_INODE_TYPE::SQFS_INODE_BDEV,
    CharacterDev = SQFS_INODE_TYPE::SQFS_INODE_CDEV,
    Fifo = SQFS_INODE_TYPE::SQFS_INODE_FIFO,
    Socket = SQFS_INODE_TYPE::SQFS_INODE_SOCKET,
    ExtendedDirectory = SQFS_INODE_TYPE::SQFS_INODE_EXT_DIR,
    ExtendedFile = SQFS_INODE_TYPE::SQFS_INODE_EXT_FILE,
    ExtendedSymbolicLink = SQFS_INODE_TYPE::SQFS_INODE_EXT_SLINK,
    ExtendedBlockDevice = SQFS_INODE_TYPE::SQFS_INODE_EXT_BDEV,
    ExtendedCharacterDevice = SQFS_INODE_TYPE::SQFS_INODE_EXT_CDEV,
    ExtendedFifo = SQFS_INODE_TYPE::SQFS_INODE_EXT_FIFO,
    ExtendedSocket = SQFS_INODE_TYPE::SQFS_INODE_EXT_SOCKET,
}

/// Safe wrapper for [sqfs_inode_generic_t].
pub enum INode<'a> {
    Directory(DirectoryINode<'a>),
    File(FileINode<'a>),
    SymbolicLink(SymbolicLinkINode<'a>),
    Device(DeviceINode<'a>),
    Ipc(IpcINode<'a>),
    ExtendedDirectory(ExtendedDirectoryINode<'a>),
    ExtendedFile(ExtendedFileINode<'a>),
    ExtendedSymbolicLink(ExtendedSymbolicLinkINode<'a>),
    ExtendedDevice(ExtendedDeviceINode<'a>),
    ExtendedIpc(ExtendedIpcINode<'a>),
}

impl<'a> INode<'a> {
    pub(crate) fn new(ptr: NonNull<sqfs_inode_generic_t>) -> Self {
        let node = INodeInternal::new(ptr);

        match node.tipe() {
            INodeType::Directory => INode::Directory(DirectoryINode::new(node)),
            INodeType::File => INode::File(FileINode::new(node)),
            INodeType::SymbolicLink => INode::SymbolicLink(SymbolicLinkINode::new(node)),
            INodeType::BlockDevice | INodeType::CharacterDev => {
                INode::Device(DeviceINode::new(node))
            }
            INodeType::Fifo | INodeType::Socket => INode::Ipc(IpcINode::new(node)),
            INodeType::ExtendedDirectory => {
                INode::ExtendedDirectory(ExtendedDirectoryINode::new(node))
            }
            INodeType::ExtendedFile => INode::ExtendedFile(ExtendedFileINode::new(node)),
            INodeType::ExtendedSymbolicLink => {
                INode::ExtendedSymbolicLink(ExtendedSymbolicLinkINode::new(node))
            }
            INodeType::ExtendedBlockDevice | INodeType::ExtendedCharacterDevice => {
                INode::ExtendedDevice(ExtendedDeviceINode::new(node))
            }
            INodeType::ExtendedFifo | INodeType::ExtendedSocket => {
                INode::ExtendedIpc(ExtendedIpcINode::new(node))
            }
        }
    }
}

/// Holder of the [sqfs_inode_generic_t]
#[derive(Debug)]
pub struct INodeInternal<'a> {
    ptr: NonNull<sqfs_inode_generic_t>,
    _marker: PhantomData<&'a usize>,
}

impl<'a> INodeInternal<'a> {
    fn new(ptr: NonNull<sqfs_inode_generic_t>) -> Self {
        Self {
            ptr,
            _marker: Default::default(),
        }
    }

    fn as_ref(&self) -> &sqfs_inode_generic_t {
        unsafe { &(*self.ptr.as_ptr()) }
    }

    fn tipe(&self) -> INodeType {
        INodeType::from_u16(self.as_ref().base.type_).expect("invalid node type")
    }

    pub fn mode(&self) -> SQFS_INODE_MODE {
        SQFS_INODE_MODE(u32::from(self.as_ref().base.mode))
    }

    pub fn uid_index(&self) -> u16 {
        self.as_ref().base.uid_idx
    }

    pub fn gid_index(&self) -> u16 {
        self.as_ref().base.gid_idx
    }

    pub fn modification_time(&self) -> u32 {
        self.as_ref().base.mod_time
    }

    pub fn inode_number(&self) -> u32 {
        self.as_ref().base.inode_number
    }

    pub fn payload_bytes_available(&self) -> u32 {
        self.as_ref().payload_bytes_available
    }

    pub fn payload_bytes_used(&self) -> u32 {
        self.as_ref().payload_bytes_used
    }

    fn extra(&self) -> &__IncompleteArrayField<sqfs_u32> {
        &self.as_ref().extra
    }

    fn union_data(&self) -> &sqfs_inode_generic_t__bindgen_ty_1 {
        &self.as_ref().data
    }
}

macro_rules! inode {
    ($name:ident, $internal_name:ident, $t:ty) => {
        #[doc = concat!("Safe wrapper for [", stringify!($t), "]")]
        #[derive(Debug, Deref)]
        pub struct $name<'a> {
            node: INodeInternal<'a>,
        }

        impl<'a> $name<'a> {
            fn new(node: INodeInternal<'a>) -> Self {
                Self { node }
            }
        }

        impl<'a> $name<'a> {
            fn data(&self) -> &$t {
                unsafe { &self.union_data().$internal_name }
            }
        }
    };
}

macro_rules! inode_data_field {
    ($name:ident, $t:ty) => {
        inode_data_field!($name, $t, $name);
    };
    ($name:ident, $t:ty, $field:ident) => {
        pub fn $name(&self) -> $t {
            self.data().$field
        }
    };
}

inode!(DirectoryINode, dir, sqfs_inode_dir_t);
inode!(FileINode, file, sqfs_inode_file_t);
inode!(SymbolicLinkINode, slink, sqfs_inode_slink_t);
inode!(DeviceINode, dev, sqfs_inode_dev_t);
inode!(IpcINode, ipc, sqfs_inode_ipc_t);
inode!(ExtendedDirectoryINode, dir_ext, sqfs_inode_dir_ext_t);
inode!(ExtendedFileINode, file_ext, sqfs_inode_file_ext_t);
inode!(ExtendedSymbolicLinkINode, slink_ext, sqfs_inode_slink_ext_t);
inode!(ExtendedDeviceINode, dev_ext, sqfs_inode_dev_ext_t);
inode!(ExtendedIpcINode, ipc_ext, sqfs_inode_ipc_ext_t);

impl<'a> DirectoryINode<'a> {
    inode_data_field!(start_block, u32);
    inode_data_field!(number_of_hard_links, u32, nlink);
    inode_data_field!(size, u16);
    inode_data_field!(offset, u16);
    inode_data_field!(parent_inode_number, u32, parent_inode);
}

impl<'a> ExtendedDirectoryINode<'a> {
    inode_data_field!(start_block, u32);
    inode_data_field!(number_of_hard_links, u32, nlink);
    inode_data_field!(size, u32);
    inode_data_field!(offset, u16);
    inode_data_field!(parent_inode_number, u32, parent_inode);
    inode_data_field!(extended_attribute_index, u32, xattr_idx);

    pub fn directory_indices(&self) -> &[u32] {
        unsafe {
            slice_from_raw_parts(self.extra().as_ptr(), self.data().inodex_count as usize).as_ref()
        }
        .expect("could not create slice of directory indices")
    }
}

impl<'a> FileINode<'a> {
    inode_data_field!(fragment_index, u32);
    inode_data_field!(fragment_offset, u32);
    inode_data_field!(file_size, u32);

    pub fn blocks_start(&self) -> u64 {
        let mut start_offset = 0u64;
        unsafe { sqfs_inode_get_file_block_start(self.ptr.as_ptr(), &mut start_offset) };

        start_offset
    }

    pub fn block_count(&self) -> u32 {
        // The internal function is always inlined. So we must reimplement it.
        // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/inode_8h_source.html#l00552
        self.payload_bytes_used()
            / u32::try_from(size_of::<u32>()).expect("u32 not 4 bytes long these days?")
    }

    pub fn blocks(&self) -> Blocks<'a> {
        let len = usize::try_from(self.block_count()).expect("will always fit in in a u32");
        let arr = unsafe { slice_from_raw_parts(self.extra().as_ptr(), len).as_ref() }
            .expect("always an extra array on files");

        Blocks::new(arr, self.blocks_start())
    }
}

impl<'a> ExtendedFileINode<'a> {
    inode_data_field!(fragment_index, u32, fragment_idx);
    inode_data_field!(fragment_offset, u32);
    inode_data_field!(file_size, u64);
    inode_data_field!(number_of_hard_links, u32, nlink);
    inode_data_field!(extended_attribute_index, u32, xattr_idx);
    inode_data_field!(number_of_bytes_not_written_if_sparse, u64, sparse);

    pub fn blocks_start(&self) -> u64 {
        let mut start_offset = 0u64;
        unsafe { sqfs_inode_get_file_block_start(self.ptr.as_ptr(), &mut start_offset) };

        start_offset
    }

    pub fn block_count(&self) -> u32 {
        // The internal function is always inlined. So we must reimplement it.
        // https://infraroot.at/projects/squashfs-tools-ng/doxydoc/inode_8h_source.html#l00552
        self.payload_bytes_used()
            / u32::try_from(size_of::<u32>()).expect("u32 not 4 bytes long these days?")
    }

    pub fn blocks(&self) -> Blocks<'a> {
        let len = usize::try_from(self.block_count()).expect("will always fit in in a u32");
        let arr = unsafe { slice_from_raw_parts(self.as_ref().extra.as_ptr(), len).as_ref() }
            .expect("always an extra array on files");

        Blocks::new(arr, self.blocks_start())
    }
}

/// Iterator over [Block] data in [FileINode] and [ExtendedFileINode]
pub struct Blocks<'a> {
    arr: &'a [u32],
    index: usize,
    start_offset: u64,
}

impl<'a> Blocks<'a> {
    fn new(arr: &'a [u32], start_offset: u64) -> Self {
        Self {
            arr,
            index: 0,
            start_offset,
        }
    }
}

impl<'a> Iterator for Blocks<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let block = self
            .arr
            .get(self.index)
            .map(|&size| Block::new(self.start_offset, size));
        if let Some(block) = &block {
            self.start_offset += u64::from(block.size());
        }

        self.index += 1;

        block
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.arr.len() - self.index;
        (left, Some(left))
    }
}

impl<'a> ExactSizeIterator for Blocks<'a> {
    fn len(&self) -> usize {
        self.arr.len()
    }
}
