use fl2rust;
use std::env;
use std::path::PathBuf;
use winres::WindowsResource;
fn main() {
    let res = WindowsResource::new();
    res.compile().unwrap();

    println!("cargo:rustc-link-lib=Crypt32");
    println!("cargo:rustc-link-lib=User32");

    println!("cargo:rerun-if-changed=src/ui/ui.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/ui/ui.fl", "src/ui/mod.rs")
        .expect("Failed to generate rust from fl file!");
}
