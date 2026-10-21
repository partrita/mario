use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let font_path = "resources/JBD2/JBD2NF-Regular.ttf";
    println!("cargo:rerun-if-changed={font_path}");
    println!("cargo:rerun-if-changed=build.rs");

    let raw = fs::read(font_path).expect("failed to read JBD2 font");
    let compressed = miniz_oxide::deflate::compress_to_vec(&raw, 10);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("JBD2NF-Regular.ttf.deflate");
    fs::write(dest_path, compressed).expect("failed to write compressed font");
}
