#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}

#[cfg(target_os = "linux")]
fn main() {
    pkg_config::probe_library("javascriptcoregtk-4.1").unwrap();
}

#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rustc-link-lib=dylib=JavaScriptCore");
}
