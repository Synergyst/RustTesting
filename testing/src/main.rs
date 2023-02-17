#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::{thread, time, time::Duration};

#[link(name = "testing/lib/micpassthrough")]
extern "C" {
  fn retDevNameList(playbackCount: *mut libc::c_char, captureCount: *mut libc::c_char, playbackListGUI: *mut libc::c_char, captureListGUI: *mut libc::c_char, len: libc::c_int) -> libc::c_int;
  fn startMicPassthrough(captureDev: i32, playbackDev: i32) -> i32;
}

fn main() {
  let mut playback_count: [libc::c_char; 256] = [0; 256];
  let mut capture_count: [libc::c_char; 256] = [0; 256];
  let mut playback_list_gui: [libc::c_char; 8192] = [0; 8192];
  let mut capture_list_gui: [libc::c_char; 8192] = [0; 8192];
  let len = 8192;
  let ret = unsafe { retDevNameList(playback_count.as_mut_ptr(), capture_count.as_mut_ptr(), playback_list_gui.as_mut_ptr(), capture_list_gui.as_mut_ptr(), len) };
  if ret != 1 {
    println!("Failed to call retDevNameList: {}", ret);
    return;
  }
  let playback_count_usize = unsafe { std::ffi::CStr::from_ptr(playback_count.as_ptr()).to_str().unwrap().parse::<usize>().unwrap() };
  let capture_count_usize = unsafe { std::ffi::CStr::from_ptr(capture_count.as_ptr()).to_str().unwrap().parse::<usize>().unwrap() };
  let playback_list_str = unsafe { std::ffi::CStr::from_ptr(playback_list_gui.as_ptr()).to_string_lossy() };
  let capture_list_str = unsafe { std::ffi::CStr::from_ptr(capture_list_gui.as_ptr()).to_string_lossy() };
  let playback_list = playback_list_str.split("\n").collect::<Vec<&str>>();
  let capture_list = capture_list_str.split("\n").collect::<Vec<&str>>();
  //
  println!("\nPlayback count: {}", playback_count_usize);
  for i in 0..playback_count_usize {
    println!("\t[{}] -> [{}]", i, playback_list[i])
  }
  println!("\nCapture count: {}", capture_count_usize);
  for i in 0..capture_count_usize {
    println!("\t[{}] -> [{}]", i, capture_list[i])
  }
  //
  let duration = time::Duration::from_millis(1000);
  let now = time::Instant::now();
  thread::sleep(duration);
  assert!(now.elapsed() >= duration);
  //
  unsafe { startMicPassthrough(3, 5); }
}
