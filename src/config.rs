use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use toml::{Value};

pub struct Config {
  pub exec: String,
  pub cpu_time: u64,
  pub memory: u64
}

impl Config {
  pub fn new(filename: &Path) -> Config {
    let mut f = File::open(filename).unwrap();
    let mut data = String::new();
    let _ = f.read_to_string(&mut data);
    let config: Value = data.as_str().parse().unwrap();
    let exec = config.lookup("command.exec").unwrap().as_str().unwrap().to_string();
    let cpu_time = config.lookup("sandbox.cpu_time").unwrap().as_integer().unwrap() as u64;
    let memory = config.lookup("sandbox.memory").unwrap().as_integer().unwrap() as u64;
    Config {
      exec: exec,
      cpu_time: cpu_time,
      memory: memory
    }
  }
}
