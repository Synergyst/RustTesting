#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(overflowing_literals)]
use winapi;
use winapi::shared::windef::HHOOK;
use winapi::um::winuser;
//use winapi::um::winuser::{HC_ACTION, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP};
use winapi::um::winuser::GetAsyncKeyState;
use std::net::UdpSocket;
use std::fs;
use std::path::Path;
use std::mem::MaybeUninit;
use std::{thread, time};
enum KeyInput {
  NextSound,
  PrevSound,
  NextLibrary,
  PrevLibrary,
  PlaySound,
  ToggleBoard,
  ToggleLoop,
  ReloadSounds,
  ShowHelp,
  Quit,
  PitchUp,
  PitchDown,
  VolumeUp,
  VolumeDown,
}
fn cycle_sound_file(snd_iter: &mut i32, actual_audio_file_list_size: i32, is_cycle_forward_down: bool, is_cycle_backward_down: bool, prev_cycle_forward_down: bool, prev_cycle_backward_down: bool) {
  if is_cycle_forward_down && is_cycle_forward_down != prev_cycle_forward_down || is_cycle_backward_down && is_cycle_backward_down != prev_cycle_backward_down {
    if is_cycle_forward_down && is_cycle_forward_down != prev_cycle_forward_down {
      *snd_iter = (*snd_iter + 1) % (actual_audio_file_list_size);
    } else {
      *snd_iter = (*snd_iter + actual_audio_file_list_size) % (actual_audio_file_list_size + 1);
    }
    //configureNextFile(audio_file_list[*snd_iter]);
  }
}
fn cycle_sound_library(snd_dir_iter: &mut i32, actual_audio_dir_list_size: i32, is_cycle_forward_dir_down: bool, is_cycle_backward_dir_down: bool, prev_cycle_forward_dir_down: bool, prev_cycle_backward_dir_down: bool) {
  if is_cycle_forward_dir_down && is_cycle_forward_dir_down != prev_cycle_forward_dir_down || is_cycle_backward_dir_down && prev_cycle_backward_dir_down != prev_cycle_backward_dir_down {
    *snd_dir_iter = *snd_dir_iter + if is_cycle_forward_dir_down { 1 } else { -1 };
    if *snd_dir_iter < 0 {
      *snd_dir_iter = actual_audio_dir_list_size;
    } else if *snd_dir_iter > actual_audio_dir_list_size {
      *snd_dir_iter = 0;
    }
    //configure_next_sound_dir(sound_dir_list[snd_dir_iter]);
  }
}
fn get_key_state(r: KeyInput) -> bool {
  match r {
    KeyInput::NextSound | KeyInput::PrevSound => true,
    _ => false,
  }
}
fn list_files(dir: &str) -> Vec<String> {
  let paths = fs::read_dir(dir).unwrap();
  let mut files = vec![];
  for path in paths {
    let path = path.unwrap().path();
    if path.is_file() {
      files.push(path.to_str().unwrap().to_owned());
    }
  }
  files
}
fn list_folders(dir: &str) -> Vec<String> {
  let paths = fs::read_dir(dir).unwrap();
  let mut dirs = vec![];
  for path in paths {
    let path = path.unwrap().path();
    if path.is_dir() {
      dirs.push(path.to_str().unwrap().to_owned());
    }
  }
  dirs
}
fn listen_for_key_press(key: u32) -> bool {
  unsafe {
      let key_state = GetAsyncKeyState(key as i32);
      key_state & 0x8000 != 0
  }
}

fn main() {
  //println!("The area of the rectangle is {} square pixels.", area(640, 360));
  //print!("{:?}", get_key_state(KeyInput::NextLibrary));
  //let result = udp_runner();
  //println!("{:?}", result);
  let files = list_files("./");
  let folders = list_folders("./");
  for file in files {
    println!("File: {}", file);
  }
  for folder in folders {
    println!("Folder: {}", folder);
  }
  let duration = time::Duration::from_millis(5000);
  let now = time::Instant::now();
  thread::sleep(duration);
  let is_numpad0_pressed = listen_for_key_press(0x60);
  assert!(now.elapsed() >= duration);
  println!("{:?}", is_numpad0_pressed);

}
fn area(width: u32, height: u32) -> u32 {
  width * height
}
fn hex_to_i32(hex: &str) -> i32 {
  let without_prefix = hex.trim_start_matches("0x");
  i32::from_str_radix(without_prefix, 16).unwrap()
}
fn hex_to_i64(hex: &str) -> i64 {
  let without_prefix = hex.trim_start_matches("0x");
  i64::from_str_radix(without_prefix, 16).unwrap()
}
fn udp_runner() -> std::io::Result<()> {
  {
    let socket = UdpSocket::bind("0.0.0.0:9999")?;
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 64];
    let (amt, src) = socket.recv_from(&mut buf)?;
    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
  } // the socket is closed here
  Ok(())
}
