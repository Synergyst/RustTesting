#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use winapi::um::winuser::GetAsyncKeyState;
use std::net::UdpSocket;
use std::fs;
use std::path::Path;
use std::mem::MaybeUninit;
use std::{thread, time};
use std::io::{BufReader, Sink};
use anyhow;
use clap::Parser;
use cpal;
use rodio;

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

enum OffSet {
  Neg(usize),
  Pos(usize),
}

struct KeyedInfoFile<'a> {
  snd_iter: &'a mut usize,
  actual_audio_file_list_size: usize,
  is_cycle_forward_file_down: bool,
  is_cycle_backward_file_down: bool,
  prev_cycle_forward_file_down: bool,
  prev_cycle_backward_file_down: bool,
  audio_file_list: &'a mut Vec<String>
}
struct KeyedInfoFolder<'a> {
  snd_dir_iter: &'a mut usize,
  actual_audio_dir_list_size: usize,
  is_cycle_forward_dir_down: bool,
  is_cycle_backward_dir_down: bool,
  prev_cycle_forward_dir_down: bool,
  prev_cycle_backward_dir_down: bool,
  sound_dir_list: &'a mut Vec<String>
}

fn bar(n: usize, offset: OffSet) -> Option<usize> {
  match offset {
      OffSet::Pos(offset) => n.checked_add(offset),
      OffSet::Neg(offset) => n.checked_sub(offset),
  }
}
fn hex_to_i32(hex: &str) -> i32 {
  let without_prefix = hex.trim_start_matches("0x");
  i32::from_str_radix(without_prefix, 16).unwrap()
}
fn hex_to_i64(hex: &str) -> i64 {
  let without_prefix = hex.trim_start_matches("0x");
  i64::from_str_radix(without_prefix, 16).unwrap()
}
/*fn cycle_sound_file(snd_iter: &mut i32, actual_audio_file_list_size: i32, is_cycle_forward_down: bool, is_cycle_backward_down: bool, prev_cycle_forward_down: bool, prev_cycle_backward_down: bool) {
  if is_cycle_forward_down && is_cycle_forward_down != prev_cycle_forward_down || is_cycle_backward_down && is_cycle_backward_down != prev_cycle_backward_down {
    if is_cycle_forward_down && is_cycle_forward_down != prev_cycle_forward_down {
      *snd_iter = (*snd_iter + 1) % (actual_audio_file_list_size);
    } else {
      *snd_iter = (*snd_iter + actual_audio_file_list_size) % (actual_audio_file_list_size + 1);
    }
    configure_next_sound_file(audio_file_list[*snd_iter]);
  }
}*/
fn cycle_sound_file(keyed_infos_file: &mut KeyedInfoFile) {
  if keyed_infos_file.is_cycle_forward_file_down && keyed_infos_file.is_cycle_forward_file_down != keyed_infos_file.prev_cycle_forward_file_down || keyed_infos_file.is_cycle_backward_file_down && keyed_infos_file.is_cycle_backward_file_down != keyed_infos_file.prev_cycle_backward_file_down {
    *keyed_infos_file.snd_iter = (*keyed_infos_file.snd_iter as isize + if keyed_infos_file.is_cycle_forward_file_down { 1 } else { -1 }) as usize % keyed_infos_file.actual_audio_file_list_size;
    // CHANGEME?
    /*if *keyed_infos_file.snd_iter < 0 {
      *keyed_infos_file.snd_iter += keyed_infos_file.actual_audio_file_list_size;
    }*/
    configure_next_sound_file(&keyed_infos_file.audio_file_list[*keyed_infos_file.snd_iter as usize]);
  }
}
fn cycle_sound_dir(keyed_infos_folder: &mut KeyedInfoFolder) {
  if keyed_infos_folder.is_cycle_forward_dir_down && keyed_infos_folder.is_cycle_forward_dir_down != keyed_infos_folder.prev_cycle_forward_dir_down || keyed_infos_folder.is_cycle_backward_dir_down && keyed_infos_folder.is_cycle_backward_dir_down != keyed_infos_folder.prev_cycle_backward_dir_down {
    *keyed_infos_folder.snd_dir_iter = (*keyed_infos_folder.snd_dir_iter as isize + if keyed_infos_folder.is_cycle_forward_dir_down { 1 } else { -1 }) as usize;
    if *keyed_infos_folder.snd_dir_iter >= keyed_infos_folder.actual_audio_dir_list_size {
      *keyed_infos_folder.snd_dir_iter = 0;
    } else {
      *keyed_infos_folder.snd_dir_iter = keyed_infos_folder.actual_audio_dir_list_size - 1;
    }
    configure_next_sound_dir(&keyed_infos_folder.sound_dir_list[*keyed_infos_folder.snd_dir_iter]);
  }
}
/*fn cycle_sound_library(snd_dir_iter: &mut isize, actual_audio_dir_list_size: isize, is_cycle_forward_dir_down: bool, is_cycle_backward_dir_down: bool, prev_cycle_forward_dir_down: bool, prev_cycle_backward_dir_down: bool, sound_dir_list: &mut Vec<String>) {
  if is_cycle_forward_dir_down && is_cycle_forward_dir_down != prev_cycle_forward_dir_down || is_cycle_backward_dir_down && prev_cycle_backward_dir_down != prev_cycle_backward_dir_down {
    *snd_dir_iter = *snd_dir_iter + if is_cycle_forward_dir_down { 1 } else { -1 };
    if *snd_dir_iter < 0 {
      *snd_dir_iter = actual_audio_dir_list_size;
    } else if *snd_dir_iter > actual_audio_dir_list_size {
      *snd_dir_iter = 0;
    }
    configure_next_sound_dir(sound_dir_list[snd_dir_iter]);
  }
}*/
fn get_key_state(r: KeyInput) -> bool {
  match r {
    KeyInput::NextSound | KeyInput::PrevSound => true,
    _ => false,
  }
}
fn list_files(dir: &str) -> Result<Vec<String>, std::io::Error> {
  let paths = fs::read_dir(dir)?;
  let mut files = vec![];
  for path in paths {
    let path = path?.path();
    if path.is_file() {
      files.push(path.to_str().unwrap().to_owned());
    }
  }
  Ok(files)
}
fn list_folders(dir: &str) -> Result<Vec<String>, std::io::Error> {
  let paths = fs::read_dir(dir)?;
  let mut dirs = vec![];
  for path in paths {
    let path = path?.path();
    if path.is_dir() {
      dirs.push(path.to_str().unwrap().to_owned());
    }
  }
  Ok(dirs)
}
/*fn modify_folders(folders: &mut Vec<String>) {
  // modify the `folders` value as needed
}
fn modify_files(files: &mut Vec<String>) {
  // modify the `files` value as needed
}*/
fn listen_for_key_press(key: u32) -> bool {
  unsafe {
      let key_state = GetAsyncKeyState(key as i32);
      key_state & 0x8000 != 0
  }
}
fn key_state_runner(keyed_infos_folder: &mut KeyedInfoFolder, keyed_infos_file: &mut KeyedInfoFile) {
  loop {
    let duration = time::Duration::from_millis(50);
    let now = time::Instant::now();
    thread::sleep(duration);
    keyed_infos_file.is_cycle_forward_file_down = listen_for_key_press(0x60);
    keyed_infos_file.is_cycle_backward_file_down = listen_for_key_press(0x61);
    /*let is_numpad2_pressed = listen_for_key_press(0x62);
    let is_numpad3_pressed = listen_for_key_press(0x63);*/
    keyed_infos_folder.is_cycle_backward_dir_down = listen_for_key_press(0x64);
    keyed_infos_folder.is_cycle_forward_dir_down = listen_for_key_press(0x65);
    /*let is_numpad6_pressed = listen_for_key_press(0x66);
    let is_numpad7_pressed = listen_for_key_press(0x67);
    let is_numpad8_pressed = listen_for_key_press(0x68);
    let is_numpad9_pressed = listen_for_key_press(0x69);*/
    assert!(now.elapsed() >= duration);
    /*print!("\r                                                                                                                                                                        \r");
    print!("NUM 0: {:?}, NUM 1: {:?}, NUM 2: {:?}, NUM 3: {:?}, NUM 4: {:?}, NUM 5: {:?}, NUM 6: {:?}, NUM 7: {:?}, NUM 8: {:?}, NUM 9: {:?}",
    is_numpad0_pressed, is_numpad1_pressed, is_numpad2_pressed, is_numpad3_pressed, is_numpad4_pressed, is_numpad5_pressed, is_numpad6_pressed, is_numpad7_pressed, is_numpad8_pressed, is_numpad9_pressed);*/
    if is_numpad0_pressed {
      println!("{}", keyed_infos_file.audio_file_list[*keyed_infos_file.snd_iter]);
      //cycle_sound_dir(keyed_infos_folder);
    }
    //
    keyed_infos_file.prev_cycle_forward_file_down = keyed_infos_file.is_cycle_forward_file_down;
    keyed_infos_file.prev_cycle_backward_file_down = keyed_infos_file.is_cycle_backward_file_down;
    keyed_infos_folder.prev_cycle_forward_dir_down = keyed_infos_folder.is_cycle_forward_dir_down;
    keyed_infos_folder.prev_cycle_backward_dir_down = keyed_infos_folder.is_cycle_backward_dir_down;
    
  }
}
fn configure_next_sound_dir(sound_dir: &str) {
  print!("\n{:?}\n", sound_dir);
}
fn configure_next_sound_file(sound_file: &str) {
  print!("\n{}\n", sound_file);
}
fn main() {
  let mut folder_position: usize = 0;
  let mut folder_position_max: usize = 0;
  let mut file_position: usize = 0;
  let mut file_position_max: usize = 0;
  let mut folders = list_folders("../synergyst-soundboard-util/sounds/").unwrap();
  for folder in &folders {
    println!("{}: {}", folder_position_max, folder);
    folder_position_max += 1;
  }
  println!("Count of folders: {}", folder_position_max);
  folder_position_max -= 1;
  let starting_dir = format!("{}{}", folders[folder_position], "/");
  let mut files = list_files(starting_dir.as_str()).unwrap();
  for file in &files {
    println!("{}: {}", file_position_max, file);
    file_position_max += 1;
  }
  println!("Count of files: {}", file_position_max);
  file_position_max -= 1;
  // TODO: change audio_file_list to sound_file_list
  let mut file_infos:KeyedInfoFile = KeyedInfoFile{snd_iter:&mut file_position,actual_audio_file_list_size:file_position_max,is_cycle_forward_file_down:false,is_cycle_backward_file_down:false,prev_cycle_forward_file_down:false,prev_cycle_backward_file_down:false,audio_file_list:&mut files};
  //
  let mut folder_infos:KeyedInfoFolder = KeyedInfoFolder{snd_dir_iter:&mut folder_position,actual_audio_dir_list_size:folder_position_max,is_cycle_forward_dir_down:false,is_cycle_backward_dir_down:false,prev_cycle_forward_dir_down:false,prev_cycle_backward_dir_down:false,sound_dir_list:&mut folders};
  /*let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
  let sink = rodio::Sink::try_new(&handle).unwrap();
  //
  let file = std::fs::File::open(&files[file_position]).unwrap();
  sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
  sink.sleep_until_end();*/
  //
  /*let handle = thread::spawn(move || {
    key_state_runner(&mut folder_infos, &mut file_infos);
  });
  //print!("{:?}", get_key_state(KeyInput::NextLibrary));
  // rest of the main function code
  handle.join().unwrap();*/
  key_state_runner(&mut folder_infos, &mut file_infos);
}
