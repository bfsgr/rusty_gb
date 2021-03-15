use cbindgen;
use std::env;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(&dir)
        .unwrap()
        .write_to_file("bindings.h");
}
