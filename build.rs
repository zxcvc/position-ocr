// use fl2rust;
// use std::env;
// use std::path::PathBuf;
// use winres;

fn main() {
    // println!("cargo:rustc-link-lib=Crypt32");
    // println!("cargo:rustc-link-lib=User32");

    // println!("cargo:rerun-if-changed=src/ui/ui.fl");
    // let g = fl2rust::Generator::default();
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // g.in_out("src/ui/ui.fl", "src/ui/mod.rs")
    //     .expect("Failed to generate rust from fl file!");


    if cfg!(target_os = "windows") {
        embed_resource::compile("./icon.rc",embed_resource::NONE);
    }
}
