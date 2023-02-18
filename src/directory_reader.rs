use std::ffi::{c_char, CStr};
use std::marker::PhantomData;
use std::path::Path;
use std::ptr::{self, NonNull};

use crate::compressor::Compressor;
use crate::ffi::sqfs_tree_node_t;
use crate::ffi::{sqfs_dir_reader_create, sqfs_dir_reader_get_full_hierarchy, sqfs_dir_reader_t};
pub use crate::ffi::{SQFS_DIR_READER_FLAGS, SQFS_TREE_FILTER_FLAGS};
use crate::file::File;
use crate::id::IdTable;
use crate::inode::INode;
use crate::super_block::SuperBlock;
use crate::Result;
use crate::{ManagedPointer, SqfsError};

/// Safe wrapper for [sqfs_dir_reader_t]
pub struct DirectoryReader {
    ptr: ManagedPointer<sqfs_dir_reader_t>,
}

impl DirectoryReader {
    /// Safe wrapper for [sqfs_dir_reader_create]
    pub fn new(
        file: &File,
        super_block: &SuperBlock,
        compressor: &Compressor,
        flags: SQFS_DIR_READER_FLAGS,
    ) -> Result<Self> {
        let init = || unsafe {
            sqfs_dir_reader_create(
                super_block.ptr(),
                compressor.ptr().as_ptr(),
                file.ptr().as_ptr(),
                flags.0,
            )
        };

        ManagedPointer::check_null(&init, "Creating DirectoryReader", crate::sqfs_destroy)
            .map(|ptr| Self { ptr })
    }

    // TODO: implement other methods

    /// Safe wrapper for [sqfs_dir_reader_get_full_hierarchy]
    pub fn get_full_hierarchy<P: AsRef<Path>>(
        &self,
        id_table: &IdTable,
        path: Option<P>,
        flags: SQFS_TREE_FILTER_FLAGS,
    ) -> Result<DirectoryTree> {
        let bytes = path.map(|p| crate::path_to_c_str(p));
        let path_ptr = if let Some(bytes_inside) = &bytes {
            bytes_inside.as_ptr() as *const c_char
        } else {
            ptr::null()
        };

        let init = |ptr| unsafe {
            sqfs_dir_reader_get_full_hierarchy(
                self.ptr.as_ptr(),
                id_table.ptr().as_ptr(),
                path_ptr,
                flags.0,
                ptr,
            )
        };

        ManagedPointer::init_ptr(&init, "Creating DirectoryTree", crate::sqfs_destroy)
            .map(|tree_node| DirectoryTree { tree_node })
    }
}

pub struct DirectoryTree {
    tree_node: ManagedPointer<sqfs_tree_node_t>,
}

impl DirectoryTree {
    pub fn root(&self) -> TreeNode {
        TreeNode::new(*self.tree_node)
    }
}

/// Safe wrapper for [sqfs_tree_node_t]
#[derive(Copy, Clone)]
pub struct TreeNode<'a> {
    ptr: NonNull<sqfs_tree_node_t>,
    _marker: PhantomData<&'a usize>,
}

impl<'a> TreeNode<'a> {
    fn new(ptr: NonNull<sqfs_tree_node_t>) -> Self {
        Self {
            ptr,
            _marker: Default::default(),
        }
    }

    pub fn children(self) -> Children<'a> {
        let first_child = self.as_ref().children;
        let current = NonNull::new(first_child).map(Self::new);

        Children { current }
    }

    pub fn gid(self) -> u32 {
        self.as_ref().gid
    }

    pub fn inode(self) -> INode<'a> {
        let inode = self.as_ref().inode;
        let ptr = NonNull::new(inode).expect("every directory tree has an inode");

        INode::new(ptr)
    }

    pub fn name(self) -> &'a str {
        let str_ptr = self.as_ref().name.as_ptr();
        let c_str = unsafe { CStr::from_ptr(str_ptr as *const i8) };

        c_str
            .to_str()
            .expect("failed to convert tree_node name to &str")
    }

    fn next(self) -> Option<Self> {
        let next = self.as_ref().next;
        NonNull::new(next).map(Self::new)
    }

    pub fn parent(self) -> Option<Self> {
        let parent = self.as_ref().parent;
        NonNull::new(parent).map(Self::new)
    }

    pub fn uid(self) -> u32 {
        self.as_ref().uid
    }

    /// Unconditional deref of backing pointer
    fn as_ref(self) -> &'a sqfs_tree_node_t {
        unsafe { &(*self.ptr.as_ptr()) }
    }
}

/// [Iterator] over the children of a [TreeNode]
pub struct Children<'a> {
    current: Option<TreeNode<'a>>,
}

impl<'a> Iterator for Children<'a> {
    type Item = TreeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current = self.current.and_then(|current| current.next());

        current
    }
}
