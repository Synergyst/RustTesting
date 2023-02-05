#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(overflowing_literals)]
use winapi::um::winuser::GetAsyncKeyState;
use std::net::UdpSocket;
use std::fs;
use std::path::Path;
use std::mem::MaybeUninit;
use std::{thread, time};
use std::io::BufReader;
use anyhow;
use clap::Parser;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SizedSample,
};
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

/*fn list_files(dir: &str) -> Vec<String> {
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
}*/
fn listen_for_key_press(key: u32) -> bool {
  unsafe {
      let key_state = GetAsyncKeyState(key as i32);
      key_state & 0x8000 != 0
  }
}
fn key_state_runner() {
  loop {
    let duration = time::Duration::from_millis(50);
    let now = time::Instant::now();
    thread::sleep(duration);
    let is_numpad0_pressed = listen_for_key_press(0x60);
    let is_numpad1_pressed = listen_for_key_press(0x61);
    let is_numpad2_pressed = listen_for_key_press(0x62);
    let is_numpad3_pressed = listen_for_key_press(0x63);
    let is_numpad4_pressed = listen_for_key_press(0x64);
    let is_numpad5_pressed = listen_for_key_press(0x65);
    let is_numpad6_pressed = listen_for_key_press(0x66);
    let is_numpad7_pressed = listen_for_key_press(0x67);
    let is_numpad8_pressed = listen_for_key_press(0x68);
    let is_numpad9_pressed = listen_for_key_press(0x69);
    assert!(now.elapsed() >= duration);
    print!("\r                                                                                                                                                                        \r");
    print!("NUM 0: {:?}, NUM 1: {:?}, NUM 2: {:?}, NUM 3: {:?}, NUM 4: {:?}, NUM 5: {:?}, NUM 6: {:?}, NUM 7: {:?}, NUM 8: {:?}, NUM 9: {:?}",
    is_numpad0_pressed, is_numpad1_pressed, is_numpad2_pressed, is_numpad3_pressed, is_numpad4_pressed, is_numpad5_pressed, is_numpad6_pressed, is_numpad7_pressed, is_numpad8_pressed, is_numpad9_pressed);
  }
}

/*fn modify_folders(folders: &mut Vec<String>) {
  // modify the `folders` value as needed
}

fn modify_files(files: &mut Vec<String>) {
  // modify the `files` value as needed
}*/

fn main() {
  let folders = list_folders("../synergyst-soundboard-util/sounds/").unwrap();
  let starting_dir = format!("{}{}", folders[0], "/");
  let files = list_files(starting_dir.as_str()).unwrap();

  let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
  let sink = rodio::Sink::try_new(&handle).unwrap();
  let file = std::fs::File::open(&files[0]).unwrap();
  sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
  sink.sleep_until_end();

  //modify_files(&mut files);
  //modify_folders(&mut folders);

  
  //print!("{:?}", get_key_state(KeyInput::NextLibrary));
  /*let files = list_files("./");
  let folders = list_folders("./");
  for file in files {
    println!("File: {}", file);
  }
  for folder in folders {
    println!("Folder: {}", folder);
  }
  let handle = thread::spawn(|| {
    key_state_runner();
});
// rest of the main function code
handle.join().unwrap();*/
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

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error> where T: SizedSample + FromSample<f32>, {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32) where T: Sample + FromSample<f32>, {
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}