use bindgen::callbacks;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CargoCallbacks;

impl callbacks::ParseCallbacks for CargoCallbacks {
    // https://github.com/rust-lang/rust-bindgen/issues/1594
    fn int_macro(&self, name: &str, _value: i64) -> Option<callbacks::IntKind> {
        if name.starts_with("CKA_")
            || name.starts_with("CKF_")
            || name.starts_with("CKM_")
            || name.starts_with("CKR_")
            || name.starts_with("CKO_")
            || name.starts_with("CKS_")
        {
            Some(bindgen::callbacks::IntKind::U64)
        } else if ["CK_TRUE", "CK_FALSE"].contains(&name) {
            Some(bindgen::callbacks::IntKind::U8)
        } else {
            None
        }
    }

    fn include_file(&self, filename: &str) {
        println!("cargo:rerun-if-changed={}", filename);
    }
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
