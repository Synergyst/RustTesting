#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]

use std::os;
use std::path::Path;

#[link(name = "target/debug/rnnoisedll.dll")]
extern "C" {
  fn process_files(f1: String, f2: String) -> i32;
}

/*#[link(name = "target/debug/rnnoiselib")]
extern "C" {
  fn rnnInit() -> i32;
  fn rnnDeinit() -> i32;
  fn rnnProcessFrame_s16_to_s16(input: &mut Vec<i16>, output: &mut Vec<i16>, frameCount: i32, channels: i32) -> f32;
}*/

fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  if args.len() != 3 {
    eprintln!("Usage: testingalt FILE1 FILE2");
    std::process::exit(1);
  }
  let f1 = &args[1];
  let f2 = &args[2];
  unsafe { process_files(f1.to_string(), f2.to_string()); }
  let lib = libloading::Library::new("target/debug/rnnoiselib.dll");
  //let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"my_func");
  std::process::exit(0);
}