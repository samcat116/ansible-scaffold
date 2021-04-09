use std::fs;
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() -> std::io::Result<()> {
    let projectName: String;

    fs::create_dir("/hello/word")?;


}

struct Role {
    name: String,
}