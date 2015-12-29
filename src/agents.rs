use nix::sys::ioctl::libc::{rlimit, setrlimit, execv};
use nix::sys::ioctl::libc::{RLIMIT_AS, RLIMIT_CPU};
use std::ptr;

pub fn cpu_time(time: u64) -> i32 {
  let limit = rlimit { rlim_cur: time, rlim_max: time };
  unsafe { setrlimit(RLIMIT_CPU, &limit as *const rlimit) }
}

pub fn memory(size: u64) -> i32 {
  let limit = rlimit { rlim_cur: size, rlim_max: size };
  unsafe { setrlimit(RLIMIT_AS, &limit as *const rlimit) }
}

pub fn run(path: String) -> i32 {
  println!("{:?}", path);
  unsafe { execv(path.as_bytes().as_ptr() as *const i8, ptr::null()) }
}