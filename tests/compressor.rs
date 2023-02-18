use libsquashfs1_sys::ffi::{SQFS_COMP_FLAG, SQFS_FILE_OPEN_FLAGS};
use squashed::compressor::CompressorConfig;
use squashed::file::File;
use squashed::super_block::SuperBlock;

#[test]
fn create_config() {
    let file = File::open(
        "../deltagen/imgs/new.img",
        SQFS_FILE_OPEN_FLAGS::SQFS_FILE_OPEN_READ_ONLY,
    )
    .expect("opening file");

    let super_block = SuperBlock::read(&file).expect("super block");

    assert!(
        CompressorConfig::new(&super_block, SQFS_COMP_FLAG::SQFS_COMP_FLAG_UNCOMPRESS).is_ok(),
        "creating compressor config"
    );
}
