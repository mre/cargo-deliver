#[macro_use]
extern crate duct;

#[macro_use]
extern crate lazy_static;
extern crate rprompt;

extern crate failure;

#[macro_use]
extern crate tera;

use failure::SyncFailure;

use tera::{Context, Tera};
use std::fs::File;
use std::io::Write;
use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::result;
use std::process;

pub type Result<T> = result::Result<T, failure::Error>;

static GORLEASER_CONFIG: &'static str = ".goreleaser.yml";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        compile_templates!("templates/**/*")
    };
}

pub fn has_cmd(cmd: &str) -> bool {
    let path = env::var_os("PATH").unwrap_or(OsString::new());
    env::split_paths(&path)
        .map(|p| p.join(&cmd))
        .any(|p| p.exists())
}

fn create_goreleaser_config() -> Result<String> {
    let mut context = Context::new();
    context.add("binary_name", &"cargo-deliver");
    context.add("targets", &vec!["x86_64-apple-darwin"]);
    Ok(TEMPLATES
        .render(GORLEASER_CONFIG, &context)
        .map_err(SyncFailure::new)?)
}

fn write_config(content: String) -> Result<()> {
    let mut output = File::create(GORLEASER_CONFIG)?;
    Ok(output.write_all(content.as_bytes())?)
}

fn main() {
    if !has_cmd("goreleaser") {
        println!("Cannot find goreleaser. Get it from https://goreleaser.com/")
    }

    if !Path::new(GORLEASER_CONFIG).exists() {
        let reply = rprompt::prompt_reply_stdout(&format!(
            "{} was not found. Shall I create one? [y/n]",
            GORLEASER_CONFIG
        )).unwrap();
        match reply.as_ref() {
            "y" => create_goreleaser_config()
                .and_then(|rendered| Ok(write_config(rendered)?))
                .and_then(|_| {
                    Ok(println!(
                        "Done. Please review the config file at {} and re-run the command.",
                        GORLEASER_CONFIG
                    ))
                })
                .and_then(|_| Ok(process::exit(0)))
                .expect(&format!("Cannot create `{}`", GORLEASER_CONFIG)),
            _ => return,
        };
    }

    cmd!("goreleaser", "--rm-dist")
        .read()
        .expect("Failure! Is goreleaser installed?");
}
