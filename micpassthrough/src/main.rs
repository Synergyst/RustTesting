#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use miniaudio::{self, FramesMut, Frames, RawDevice};
use nnnoiseless;
use std::env;
use std::path::Path;
use miniaudio::{Context, Device, DeviceConfig, DeviceType, DeviceId, DecoderConfig, DeviceConfigPlayback, DeviceIdAndName, DeviceConfigCapture, StreamLayout, StreamFormat};
use std::ffi::OsStr;
use std::sync::Arc;
use std::thread;
static VERSION: &str = "Version 0.1";
enum UserInput {
  UserInputName,
  UserInputId,
  NoUserInput,
}
struct DevInfo {
  dev_index_output: usize,
  dev_index_input: usize,
}
struct PlaybackDevsInfo {
  playback_devs_id: Vec<Option<DeviceId>>,
  playback_devs_name: Vec<String>,
}
struct CaptureDevsInfo {
    capture_devs_id: Vec<Option<DeviceId>>,
    capture_devs_name: Vec<String>,
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
  \t{} --playname <playback device name>
  \t\t- preferred playback device name
  \t{} --playid <playback device ID>
  \t\t- preferred playback device ID
  \t{} --capname <playback device name>
  \t\t- preferred playback device name
  \t{} --capid <playback device ID>
  \t\t- preferred playback device ID\n", prog_name, VERSION, prog_name, prog_name, prog_name, prog_name, prog_name, prog_name, prog_name);
}
fn main() {
  let mut user_input_enum_play:UserInput = UserInput::NoUserInput;
  let mut user_input_enum_cap:UserInput = UserInput::NoUserInput;
  let mut dev_infos:DevInfo = DevInfo{dev_index_input:0,dev_index_output:0};
  let mut preffered_play_dev_name: String = "".to_string();
  let mut preffered_cap_dev_name: String = "".to_string();
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
      "--playname" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
          user_input_enum_play = UserInput::UserInputName;
          preffered_play_dev_name = cli_config;
          //let preffered_dev_name: String = "VoiceMeeter Input (VB-Audio VoiceMeeter VAIO)".to_string();
          println!("Will look for preffered device name of: [{}]", preffered_play_dev_name);
        } else {
          panic!("No value specified for parameter: --playname");
        }
      }
      "--playid" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
          let id_num = cli_config.parse::<usize>().unwrap();
          user_input_enum_play = UserInput::UserInputId;
          dev_infos.dev_index_output = id_num;
          println!("Will look for preffered device ID of: [{}]", id_num);
        } else {
          panic!("No value specified for parameter: --playid");
        }
      }
      "--capname" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
          user_input_enum_cap = UserInput::UserInputName;
          preffered_cap_dev_name = cli_config;
          //let preffered_dev_name: String = "VoiceMeeter Input (VB-Audio VoiceMeeter VAIO)".to_string();
          println!("Will look for preffered device name of: [{}]", preffered_cap_dev_name);
        } else {
          panic!("No value specified for parameter: --capname");
        }
      }
      "--capid" => {
        if let Some(arg_config) = args.next() {
          cli_config = arg_config;
          let id_num = cli_config.parse::<usize>().unwrap();
          user_input_enum_cap = UserInput::UserInputId;
          dev_infos.dev_index_input = id_num;
          println!("Will look for preffered device ID of: [{}]", id_num);
        } else {
          panic!("No value specified for parameter: --capid");
        }
      }
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
  let mut play_config = DeviceConfig::new(DeviceType::Playback);
  let mut cap_config = DeviceConfig::new(DeviceType::Capture);
  play_config.playback_mut().set_format(miniaudio::Format::S16);
  cap_config.capture_mut().set_format(miniaudio::Format::S16);
  play_config.playback_mut().set_channels(2);
  cap_config.capture_mut().set_channels(2);
  play_config.set_sample_rate(48000);
  cap_config.set_sample_rate(48000);
  play_config.playback_mut().set_share_mode(miniaudio::ShareMode::Shared);
  cap_config.capture_mut().set_share_mode(miniaudio::ShareMode::Shared);
  //
  let mut play_dev_ids: Vec<Option<DeviceId>> = Vec::new();
  let mut play_dev_names: Vec<String> = Vec::new();
  let mut cap_dev_ids: Vec<Option<DeviceId>> = Vec::new();
  let mut cap_dev_names: Vec<String> = Vec::new();
  //
  let play_context = Context::new(&[], None).expect("failed to create playback context");
  play_context.with_playback_devices(|playback_devices| {
    println!("Playback Devices:");
    for (idx, device) in playback_devices.iter().enumerate() {
      println!("\t{}: [{}]", idx, device.name());
      //if idx == dev_infos.dev_index_output
    }
  }).expect("failed to get playback devices");
  //
  let cap_context = Context::new(&[], None).expect("failed to create capture context");
  cap_context.with_capture_devices(|capture_devices| {
    println!("Capture Devices:");
    for (idx, device) in capture_devices.iter().enumerate() {
      println!("\t{}: {}", idx, device.name());
    }
  }).expect("failed to get capture devices");
  let mut preffered_play_dev_id: usize = 0;
  let mut preffered_cap_dev_id: usize = 0;
  let play_dev_count = play_context.playback_device_count() as usize;
  let cap_dev_count = cap_context.capture_device_count() as usize;
  let playback_devs = play_context.playback_devices();
  let capture_devs = cap_context.capture_devices();
  //
  for idx in 0..play_dev_count {
    play_dev_ids.push(Some(playback_devs[idx].id().clone()));
    play_dev_names.push(playback_devs[idx].name().to_string());
    match user_input_enum_play {
      UserInput::UserInputName => {
        //println!("Comparing [{}] to [{}]", dev_names[idx], preffered_dev_name);
        if play_dev_names[idx] == preffered_play_dev_name {
          preffered_play_dev_id = idx;
          println!("Using [{}] (preffered device) at [{}] (index) by manual name input", preffered_play_dev_name, idx);
          break;
        }
      },
      UserInput::UserInputId => {
        if idx == dev_infos.dev_index_output {
          preffered_play_dev_id = idx;
          println!("Using [{}] (preffered device) at [{}] (index) by manual index input", play_dev_names[dev_infos.dev_index_output], idx);
          break;
        }
      },
      UserInput::NoUserInput => {
        println!("No preffered device selected, using system default playback device");
        break;
      }
    }
  }
  //
  //
  //
  for idx in 0..cap_dev_count {
    cap_dev_ids.push(Some(capture_devs[idx].id().clone()));
    cap_dev_names.push(capture_devs[idx].name().to_string());
    match user_input_enum_cap {
      UserInput::UserInputName => {
        //println!("Comparing [{}] to [{}]", dev_names[idx], preffered_dev_name);
        if cap_dev_names[idx] == preffered_cap_dev_name {
          preffered_cap_dev_id = idx;
          println!("Using [{}] (preffered device) at [{}] (index) by manual name input", preffered_cap_dev_name, idx);
          break;
        }
      },
      UserInput::UserInputId => {
        if idx == dev_infos.dev_index_input {
          preffered_cap_dev_id = idx;
          println!("Using [{}] (preffered device) at [{}] (index) by manual index input", cap_dev_names[dev_infos.dev_index_input], idx);
          break;
        }
      },
      UserInput::NoUserInput => {
        println!("No preffered device selected, using system default capture device");
        break;
      }
    }
  }
  //
  let playback_devs_infos: PlaybackDevsInfo = PlaybackDevsInfo{playback_devs_id: play_dev_ids, playback_devs_name: play_dev_names};
  play_config.playback_mut().set_device_id(Some(playback_devs[preffered_play_dev_id].id().clone()));
  //
  let capture_devs_infos: CaptureDevsInfo = CaptureDevsInfo{capture_devs_id: cap_dev_ids, capture_devs_name: cap_dev_names};
  cap_config.capture_mut().set_device_id(Some(capture_devs[preffered_cap_dev_id].id().clone()));
  //
  //
  //
  /*let mut cap_device = Device::new(Some(cap_context), &cap_config).expect("failed to open capture device");
  cap_device.set_data_callback(move |_device: &RawDevice, input: &mut miniaudio::FramesMut, _frames: &Frames| {
    let mut data = input.frames_mut();
    //
    unsafe {
      let mut play_device = Arc::new(Device::new(Some(play_context), &play_config)).expect("failed to open playback device");
      let mut play_dev = Arc::clone();
      play_device.set_data_callback(move |_device: &RawDevice, output: &mut miniaudio::FramesMut, _frames: &Frames| {
        
      });
      play_device.start().expect("failed to start device");
    }
  });
  cap_device.start().expect("failed to start device");
  loop {
    //
  }*/
}
