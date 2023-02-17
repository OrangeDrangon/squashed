use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;

fn main() {
    pkg_config::Config::new()
        .atleast_version("1.2.0")
        .statik(cfg!(feature = "static"))
        .probe("libsquashfs1")
        .unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_default(true)
        .allowlist_function("sqfs_.*")
        .allowlist_function("sqfs_destroy")
        .allowlist_type("SQFS_.*")
        .default_enum_style(EnumVariation::ModuleConsts {})
        .bitfield_enum("SQFS_.*_FLAG.*")
        .bitfield_enum("SQFS_INODE_MODE")
        .enable_function_attribute_detection()
        .disable_nested_struct_naming()
        .generate_comments(true)
        .clang_arg("-fretain-comments-from-system-headers")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
