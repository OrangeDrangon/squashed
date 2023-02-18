use std::path::PathBuf;

use squashed::compressor::Compressor;
use squashed::compressor::CompressorConfig;
use squashed::compressor::SQFS_COMP_FLAG;
use squashed::directory_reader::DirectoryReader;
use squashed::directory_reader::{SQFS_DIR_READER_FLAGS, SQFS_TREE_FILTER_FLAGS};
use squashed::file::File;
use squashed::file::SQFS_FILE_OPEN_FLAGS;
use squashed::id::IdTable;
use squashed::super_block::SuperBlock;

#[test]
fn freeing_directory_tree() {
    let file = File::open(
        "../deltagen/imgs/new.img",
        SQFS_FILE_OPEN_FLAGS::SQFS_FILE_OPEN_READ_ONLY,
    )
    .expect("file");
    let super_block = SuperBlock::read(&file).expect("super block");
    let compressor_config =
        CompressorConfig::new(&super_block, SQFS_COMP_FLAG::SQFS_COMP_FLAG_UNCOMPRESS)
            .expect("compressor config");
    let compressor = Compressor::new(&compressor_config).expect("compressor");
    let id_table = IdTable::read(&file, &super_block, &compressor).expect("id table");
    let directory_reader =
        DirectoryReader::new(&file, &super_block, &compressor, SQFS_DIR_READER_FLAGS(0))
            .expect("directory reader");
    let _ = directory_reader
        .get_full_hierarchy::<PathBuf>(&id_table, None, SQFS_TREE_FILTER_FLAGS(0))
        .expect("directory tree");
}
