
use std::path::PathBuf;
use std::env;
use fl2rust;

fn main() {
    println!("cargo:rerun-if-changed=src/ui/ui.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/ui/ui.fl", "src/ui/mod.rs").expect("Failed to generate rust from fl file!");
}
