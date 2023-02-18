use libsquashfs1_sys::ffi::SQFS_FILE_OPEN_FLAGS;
use squashed::file::File;

#[test]
fn open_file() {
    assert!(
        File::new(
            "../deltagen/imgs/new.img",
            SQFS_FILE_OPEN_FLAGS::SQFS_FILE_OPEN_READ_ONLY,
        )
        .is_ok(),
        "file should open without error"
    );
}
