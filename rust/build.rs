use std::{path::PathBuf, process::Command};

fn main() {
    let lean_prefix = Command::new("lean")
        .arg("--print-prefix")
        .output()
        .expect("Failed to execute lean --print-prefix")
        .stdout;
    let lean_prefix = String::from_utf8(lean_prefix).expect("Lean prefix is not valid UTF-8");
    let lean_prefix = PathBuf::from(lean_prefix.trim());

    let bindings = bindgen::Builder::default()
        .header(lean_prefix.join("include/lean/lean.h").to_str().unwrap())
        .clang_arg(format!("-I{}", lean_prefix.join("include").display()))
        .allowlist_item("(lean|Lean|LEAN).*")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
