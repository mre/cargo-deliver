#[macro_use]
extern crate duct;

use std::env;
use std::ffi::OsString;
use std::path::Path;

static GORLEASER_CONFIG: &'static str = ".goreleaser.yml";

pub fn has_cmd(cmd: &str) -> bool {
    let path = env::var_os("PATH").unwrap_or(OsString::new());
    env::split_paths(&path)
        .map(|p| p.join(&cmd))
        .any(|p| p.exists())
}

fn main() {
    if !has_cmd("goreleaser") {
        println!("Cannot find goreleaser. Get it from https://goreleaser.com/")
    }

    if !Path::new(GORLEASER_CONFIG).exists() {
        println!("yolo")
    }

    cmd!("goreleaser", "--rm-dist")
        .read()
        .expect("Failure! Is goreleaser installed?");
}
