extern crate pkg_config;

use std::process::Command;
use std::path::PathBuf;
use std::fs::create_dir;
use std::io::Result;
use std::env;

fn create_dir_if_not_exist(dir: &PathBuf) -> Result<()> {
    let result = create_dir(dir);
    if let Err(ref e) = result {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            return Ok(())
        }
    }
    result
}

#[cfg(not(feature = "vendor-xcb-proto"))]
fn get_paths() -> (String, String) {
    let pythondir = pkg_config::get_variable("xcb-proto", "pythondir").unwrap();
    let includedir = pkg_config::get_variable("xcb-proto", "xcbincludedir").unwrap();
    (pythondir, includedir)
}

#[cfg(feature = "vendor-xcb-proto")]
fn get_paths() -> (String, String) {
    let dir = "xcb-proto-1.13/".to_string();
    let pythondir = dir.clone();
    let includedir = dir + "src";
    (pythondir, includedir)
}

fn run_code_generator() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("generated");
    create_dir_if_not_exist(&out_path).unwrap();
    let out_path = out_path.to_str().unwrap();
    let (pythondir, includedir) = get_paths();
    let status = Command::new("python")
        .args(&["rs_code_generator.py", "-p", &pythondir, "-i", &includedir, "-o", out_path, "mod"])
        .status()
        .unwrap();
    assert!(status.success());
}

fn find_libxcb() {
    // xcb 1.12 adds xcb_send_request_with_fds64().
    pkg_config::Config::new()
        .atleast_version("1.12")
        .probe("xcb")
        .expect("Failed to find xcb >= 1.12 with pkg-config");
}

fn main() {
    run_code_generator();
    find_libxcb();
}
