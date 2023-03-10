#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use winapi::um::winuser::GetAsyncKeyState;
use windows::Win32::System::Com::IAddrExclusionControl;
use std::net::UdpSocket;
use std::fs;
use std::path::Path;
use std::mem::MaybeUninit;
use std::{thread, time, time::Duration};
use std::io::{BufReader, Sink};
use miniaudio::{Context, Decoder, Device, DeviceConfig, DeviceType, DeviceId, DecoderConfig, DeviceConfigPlayback, DeviceIdAndName};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use std::{error::Error, io};
use std::env;
use std::ffi::OsStr;
static VERSION: &str = "Version 0.2";
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
enum UserInput {
  UserInputName,
  UserInputId,
  NoUserInput,
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
struct DevInfo {
  dev_index_output: usize,
  dev_index_input: usize,
  is_voice_down: bool,
  prev_voice_down: bool,
}
struct PlaybackDevsInfo {
  playback_devs_id: Vec<Option<DeviceId>>,
  playback_devs_name: Vec<String>,
}
fn prog() -> Option<String> {
  env::args().next()
    .as_ref()
    .map(Path::new)
    .and_then(Path::file_name)
    .and_then(OsStr::to_str)
    .map(String::from)
}
fn help() {
  let prog_name: String = prog().unwrap_or_default();
  println!("{} {}
  Usage:
  \t{}
  \t\t- Runs the program with system default playback device
  \t{} -h, --help
  \t\t- shows this help message
  \t{} --version
  \t\t- shows the program version
  \t{} -s, --sounds <sounds directory>
  \t\t- the directory containing the sound libraries (default: sounds)
  \t{} -n, --name <playback device name>
  \t\t- preferred playback device name
  \t{} -i, --id <playback device ID>
  \t\t- preferred playback device ID
  \t{} -t, --voice <Windows hex keycode>
  \t\t- voice transmit keycode. Choose 'hexadecimal' values from: https://nehe.gamedev.net/article/msdn_virtualkey_codes/15009/\n", prog_name, VERSION, prog_name, prog_name, prog_name, prog_name, prog_name, prog_name, prog_name);
}
fn bar(n: usize, offset: OffSet) -> Option<usize> {
  match offset {
      OffSet::Pos(offset) => n.checked_add(offset),
      OffSet::Neg(offset) => n.checked_sub(offset),
  }
}
fn hex_to_i32(hex: &str) -> i32 {
  let binding = hex.to_lowercase();
  let without_prefix = binding.trim_start_matches("0x");
  i32::from_str_radix(without_prefix, 16).unwrap()
}
fn hex_to_i16(hex: &str) -> i16 {
  let binding = hex.to_lowercase();
  let without_prefix = binding.trim_start_matches("0x");
  i16::from_str_radix(without_prefix, 16).unwrap()
}
fn hex_to_u32(hex: &str) -> u32 {
  let binding = hex.to_lowercase();
  let without_prefix = binding.trim_start_matches("0x");
  u32::from_str_radix(without_prefix, 16).unwrap()
}
fn hex_to_u8(hex: &str) -> u8 {
  let binding = hex.to_lowercase();
  let without_prefix = binding.trim_start_matches("0x");
  u8::from_str_radix(without_prefix, 16).unwrap()
}
fn hex_to_i64(hex: &str) -> i64 {
  let binding = hex.to_lowercase();
  let without_prefix = binding.trim_start_matches("0x");
  i64::from_str_radix(without_prefix, 16).unwrap()
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
fn listen_for_key_press(key: u32) -> bool {
  unsafe {
      let key_state = GetAsyncKeyState(key as i32);
      key_state & 0x8000 != 0
  }
}

fn main() {
  let mut user_input_enum:UserInput = UserInput::NoUserInput;
  let mut dev_infos:DevInfo = DevInfo{dev_index_input:0,dev_index_output:0,is_voice_down:false,prev_voice_down:false};
  let mut preffered_dev_name: String = "".to_string();
  let mut voice_down_key_val: String = "0x12".to_string();
  let mut sound_dir_basename: String = "./sounds/".to_string();
  //
  let mut cli_config: String;
  let mut args = env::args().skip(1);
  while let Some(arg) = args.next() {
    match &arg[..] {
      "-h" | "--help" => {
        help();
        std::process::exit(0);
      }
      "--version" => {
        println!("{} {}", prog().unwrap_or_default(), VERSION);
        std::process::exit(0);
      }
      "-n" | "--name" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
          user_input_enum = UserInput::UserInputName;
          preffered_dev_name = cli_config;
          //let preffered_dev_name: String = "VoiceMeeter Input (VB-Audio VoiceMeeter VAIO)".to_string();
          println!("Will look for preffered device name of: [{}]", preffered_dev_name);
        } else {
          panic!("No value specified for parameter: --name");
        }
      }
      "-i" | "--id" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
          let id_num = cli_config.parse::<usize>().unwrap();
          user_input_enum = UserInput::UserInputId;
          dev_infos.dev_index_output = id_num;
          println!("Will look for preffered device ID of: [{}]", id_num);
        } else {
          panic!("No value specified for parameter: --id");
        }
      }
      "-t" | "--voice" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
          //voice_down_key_val = cli_config.parse::<u32>().unwrap();
          voice_down_key_val = cli_config;
        } else {
          panic!("No value specified for parameter: --voice");
        }
      }
      "-s" | "--sounds" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
          sound_dir_basename = "".to_string();
          sound_dir_basename = format!("./{}/", cli_config);
        } else {
          panic!("No value specified for parameter: --sounds");
        }
      }
      /*"-q" | "--quiet" => {
        println!("Quiet mode is not supported yet.");
      }
      "-v" | "--verbose" => {
        println!("Verbose mode is not supported yet.");
      }
      "-c" | "--config" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
        } else {
          panic!("No value specified for parameter: --config");
        }
      }*/
      _ => {
        if arg.starts_with('-') {
          println!("Unkown argument {}", arg);
        } else {
          println!("Unkown positional argument {}", arg);
        }
      }
    }
  }
  //
  let voice_down_key_val_u8: u8 = hex_to_u8(&voice_down_key_val.to_string());
  let voice_down_key_val_u32: u32 = hex_to_u32(&voice_down_key_val.to_string());
  let voice_down_key_val_i16: i16 = hex_to_i16(&voice_down_key_val.to_string());
  println!("Voice transmit keycode: WIN hex [{}], ASCII hex [{:#?}], char [{:?}]", &voice_down_key_val.to_string().to_lowercase().trim_start_matches("0x"), voice_down_key_val_u8, voice_down_key_val_u8 as char);
  //
  let mut folder_position: usize = 0;
  let mut folder_position_max: usize = 0;
  let mut file_position: usize = 0;
  let mut file_position_max: usize = 0;
  let mut folders = list_folders(&sound_dir_basename).unwrap();
  for folder in &folders {
    println!("{}: {}", folder_position_max, folder);
    folder_position_max += 1;
  }
  println!("Count of folders: {}", folder_position_max);
  folder_position_max -= 1;
  //
  let starting_dir = format!("{}{}", folders[folder_position], "/");
  let mut files = list_files(starting_dir.as_str()).unwrap();
  for file in &files {
    println!("{}: {}", file_position_max, file);
    file_position_max += 1;
  }
  println!("Count of files: {}", file_position_max);
  file_position_max -= 1;
  //
  // TODO: change audio_file_list to sound_file_list
  let mut file_infos:KeyedInfoFile = KeyedInfoFile{snd_iter:&mut file_position,actual_audio_file_list_size:file_position_max,is_cycle_forward_file_down:false,is_cycle_backward_file_down:false,prev_cycle_forward_file_down:false,prev_cycle_backward_file_down:false,audio_file_list:&mut files};
  let mut folder_infos:KeyedInfoFolder = KeyedInfoFolder{snd_dir_iter:&mut folder_position,actual_audio_dir_list_size:folder_position_max,is_cycle_forward_dir_down:false,is_cycle_backward_dir_down:false,prev_cycle_forward_dir_down:false,prev_cycle_backward_dir_down:false,sound_dir_list:&mut folders};
  //
  /*let result = enumerate_devices();
  println!("{:?}", result);*/
  //
  //
  let mut config = DeviceConfig::new(DeviceType::Playback);
  config.playback_mut().set_format(miniaudio::Format::S16);
  config.playback_mut().set_channels(2);
  config.set_sample_rate(48000);
  //
  let mut dev_ids: Vec<Option<DeviceId>> = Vec::new();
  let mut dev_names: Vec<String> = Vec::new();
  let context = Context::new(&[], None).expect("failed to create context");
  context.with_playback_devices(|playback_devices| {
    println!("Playback Devices:");
    for (idx, device) in playback_devices.iter().enumerate() {
      println!("\t{}: [{}]", idx, device.name());
      //if idx == dev_infos.dev_index_output
    }
    /*println!("Capture Devices:");
    for (idx, device) in capture_devices.iter().enumerate() {
      println!("\t{}: {}", idx, device.name());
    }*/
  }).expect("failed to get devices");
  let mut preffered_dev_id: usize = 0;
  let dev_count = context.playback_device_count() as usize;
  let playback_devs = context.playback_devices();
  //
  for idx in 0..dev_count {
    dev_ids.push(Some(playback_devs[idx].id().clone()));
    dev_names.push(playback_devs[idx].name().to_string());
    match user_input_enum {
      UserInput::UserInputName => {
        //println!("Comparing [{}] to [{}]", dev_names[idx], preffered_dev_name);
        if dev_names[idx] == preffered_dev_name {
          preffered_dev_id = idx;
          println!("Using [{}] (preffered device) at [{}] (index) by manual name input", preffered_dev_name, idx);
          break;
        }
      },
      UserInput::UserInputId => {
        if idx == dev_infos.dev_index_output {
          preffered_dev_id = idx;
          println!("Using [{}] (preffered device) at [{}] (index) by manual index input", dev_names[dev_infos.dev_index_output], idx);
          break;
        }
      },
      UserInput::NoUserInput => {
        println!("No preffered device selected, using system default playback device");
        break;
      }
    }
  }
  /*let mut items: Vec<Vec<&str>> = vec![vec![]];
  let binding = preffered_dev_name.to_string();
  items[0].insert(0, &binding);
  let binding = sound_dir_basename.to_string();
  items[0].insert(1, &binding);
  println!("{:#?}, {:#?}", items[0].get(0), items[0].get(1));*/
  //
  let playback_devs_infos: PlaybackDevsInfo = PlaybackDevsInfo{playback_devs_id: dev_ids, playback_devs_name: dev_names};
  config.playback_mut().set_device_id(Some(playback_devs[preffered_dev_id].id().clone()));
  //
  //
  loop {
    dev_infos.is_voice_down = listen_for_key_press(hex_to_u32(&voice_down_key_val.to_string()));
    file_infos.is_cycle_forward_file_down = listen_for_key_press(0x60);
    file_infos.is_cycle_backward_file_down = listen_for_key_press(0x61);
    folder_infos.is_cycle_backward_dir_down = listen_for_key_press(0x64);
    folder_infos.is_cycle_forward_dir_down = listen_for_key_press(0x65);
    //
    //
    //
    let mut temp_file_iter: i32 = *file_infos.snd_iter as i32;
    if file_infos.is_cycle_forward_file_down && file_infos.is_cycle_forward_file_down != file_infos.prev_cycle_forward_file_down {
      temp_file_iter += 1;
      if temp_file_iter > file_infos.actual_audio_file_list_size.try_into().unwrap() {
        temp_file_iter = 0;
      }
      *file_infos.snd_iter = temp_file_iter as usize;
      println!("[{}] -> [{}]", *file_infos.snd_iter, file_infos.audio_file_list[*file_infos.snd_iter]);
      //cycle_sound_dir(keyed_infos_folder);
    }
    if file_infos.is_cycle_backward_file_down && file_infos.is_cycle_backward_file_down != file_infos.prev_cycle_backward_file_down {
      temp_file_iter -= 1;
      if temp_file_iter < 0 {
        temp_file_iter = file_infos.actual_audio_file_list_size as i32;
      }
      *file_infos.snd_iter = temp_file_iter as usize;
      println!("[{}] -> [{}]", *file_infos.snd_iter, file_infos.audio_file_list[*file_infos.snd_iter]);
      //cycle_sound_dir(keyed_infos_folder);
    }
    //
    //
    //
    let mut temp_folder_iter: i32 = *folder_infos.snd_dir_iter as i32;
    if folder_infos.is_cycle_forward_dir_down && folder_infos.is_cycle_forward_dir_down != folder_infos.prev_cycle_forward_dir_down {
      temp_folder_iter += 1;
      if temp_folder_iter > folder_infos.actual_audio_dir_list_size.try_into().unwrap() {
        temp_folder_iter = 0;
      }
      //
      let mut file_position_max: usize = 0;
      let starting_dir = format!("{}{}", folder_infos.sound_dir_list[*folder_infos.snd_dir_iter], "/");
      *file_infos.audio_file_list = list_files(starting_dir.as_str()).unwrap();
      for file in &mut *file_infos.audio_file_list {
        file_position_max += 1;
      }
      println!("\t[{}] files found in -> [{}]", file_position_max, folder_infos.sound_dir_list[*folder_infos.snd_dir_iter]);
      file_position_max -= 1;
      file_infos.actual_audio_file_list_size = file_position_max;
      *file_infos.snd_iter = 0;
      *folder_infos.snd_dir_iter = temp_folder_iter as usize;
      let mut temp_iter = 0;
      for file in &mut *file_infos.audio_file_list {
        println!("[{}] -> [{}]", temp_iter, file);
        temp_iter += 1;
      }
      //println!("{}", folder_infos.sound_dir_list[*folder_infos.snd_dir_iter]);
      //println!("{}", keyed_infos_file.audio_file_list[*keyed_infos_file.snd_iter]);
      println!("");
      //
    }
    if folder_infos.is_cycle_backward_dir_down && folder_infos.is_cycle_backward_dir_down != folder_infos.prev_cycle_backward_dir_down {
      temp_folder_iter -= 1;
      if temp_folder_iter < 0 {
        temp_folder_iter = folder_infos.actual_audio_dir_list_size as i32;
      }
      //
      let mut file_position_max: usize = 0;
      let starting_dir = format!("{}{}", folder_infos.sound_dir_list[*folder_infos.snd_dir_iter], "/");
      *file_infos.audio_file_list = list_files(starting_dir.as_str()).unwrap();
      for file in &mut *file_infos.audio_file_list {
        file_position_max += 1;
      }
      println!("Count of files: {}", file_position_max);
      file_position_max -= 1;
      file_infos.actual_audio_file_list_size = file_position_max;
      *file_infos.snd_iter = 0;
      *folder_infos.snd_dir_iter = temp_folder_iter as usize;
      let mut temp_iter = 0;
      for file in &mut *file_infos.audio_file_list {
        println!("[{}] -> [{}]", temp_iter, file);
        temp_iter += 1;
      }
      //println!("{}", folder_infos.sound_dir_list[*folder_infos.snd_dir_iter]);
      //println!("{}", keyed_infos_file.audio_file_list[*keyed_infos_file.snd_iter]);
      println!("");
      //
    }
    while listen_for_key_press(hex_to_u32(&voice_down_key_val.to_string())) && dev_infos.prev_voice_down != dev_infos.is_voice_down {
      let mut decoder = Decoder::from_file(&file_infos.audio_file_list[*file_infos.snd_iter], None).expect("failed to initialize decoder from file");
      let mut device = Device::new(None, &config).expect("failed to open playback device");
      device.set_data_callback(move |_device, output, _frames| {
        decoder.read_pcm_frames(output);
      });
      device.start().expect("failed to start device");
      //
      while listen_for_key_press(hex_to_u32(&voice_down_key_val.to_string())) {
        let duration = time::Duration::from_millis(1);
        let now = time::Instant::now();
        thread::sleep(duration);
        assert!(now.elapsed() >= duration);
      }
      //
    }
    //
    let duration = time::Duration::from_millis(1);
    let now = time::Instant::now();
    thread::sleep(duration);
    assert!(now.elapsed() >= duration);
    //
    file_infos.prev_cycle_forward_file_down = file_infos.is_cycle_forward_file_down;
    file_infos.prev_cycle_backward_file_down = file_infos.is_cycle_backward_file_down;
    folder_infos.prev_cycle_forward_dir_down = folder_infos.is_cycle_forward_dir_down;
    folder_infos.prev_cycle_backward_dir_down = folder_infos.is_cycle_backward_dir_down;
    dev_infos.prev_voice_down = dev_infos.is_voice_down;
  }
}
