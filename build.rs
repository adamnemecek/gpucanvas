use std::env;
use std::fs::File;
use std::path::PathBuf;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

fn gl_main() {
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}

// fn mtl_main() {}

fn main() {
    gl_main();
}
