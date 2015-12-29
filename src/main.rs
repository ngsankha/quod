#![feature(convert)]

extern crate toml;
extern crate nix;

mod config;
mod agents;

use std::path::Path;
use config::Config;

fn main() {
  let config = Config::new(Path::new("config.toml"));
  //agents::cpu_time(config.cpu_time);
  println!("{:?}", agents::memory(config.memory));
  agents::run(config.exec);
}
