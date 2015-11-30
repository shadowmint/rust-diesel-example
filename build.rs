extern crate syntex;
extern crate diesel_codegen;

use std::env;
use std::path::Path;

pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut registry = syntex::Registry::new();
    diesel_codegen::register(&mut registry);

    let src = Path::new("src/schema.in.rs");
    let dst = Path::new(&out_dir).join("schema.rs");

    registry.expand("", &src, &dst).unwrap();
}
