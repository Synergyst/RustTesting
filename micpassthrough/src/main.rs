#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use miniaudio;
use nnnoiseless;
use std::env;
use std::path::Path;
use miniaudio::{Context, Device, DeviceConfig, DeviceType, DeviceId, DecoderConfig, DeviceConfigPlayback, DeviceIdAndName};
use std::ffi::OsStr;
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
  let mut config = DeviceConfig::new(DeviceType::Playback);
  config.playback_mut().set_format(miniaudio::Format::S16);
  config.playback_mut().set_channels(2);
  config.set_sample_rate(48000);
  //
  let mut play_dev_ids: Vec<Option<DeviceId>> = Vec::new();
  let mut play_dev_names: Vec<String> = Vec::new();
  let mut cap_dev_ids: Vec<Option<DeviceId>> = Vec::new();
  let mut cap_dev_names: Vec<String> = Vec::new();
  let context = Context::new(&[], None).expect("failed to create context");
  context.with_devices(|playback_devices, capture_devices| {
    println!("Playback Devices:");
    for (idx, device) in playback_devices.iter().enumerate() {
      println!("\t{}: [{}]", idx, device.name());
      //if idx == dev_infos.dev_index_output
    }
    println!("Capture Devices:");
    for (idx, device) in capture_devices.iter().enumerate() {
      println!("\t{}: {}", idx, device.name());
    }
  }).expect("failed to get devices");
  let mut preffered_play_dev_id: usize = 0;
  let mut preffered_cap_dev_id: usize = 0;
  let play_dev_count = context.playback_device_count() as usize;
  let cap_dev_count = context.capture_device_count() as usize;
  let playback_devs = context.playback_devices();
  let capture_devs = context.capture_devices();
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
        println!("No preffered device selected, using system default playback device");
        break;
      }
    }
  }
  //
  let playback_devs_infos: PlaybackDevsInfo = PlaybackDevsInfo{playback_devs_id: play_dev_ids, playback_devs_name: play_dev_names};
  config.playback_mut().set_device_id(Some(playback_devs[preffered_play_dev_id].id().clone()));
  //
  let capture_devs_infos: CaptureDevsInfo = CaptureDevsInfo{capture_devs_id: cap_dev_ids, capture_devs_name: cap_dev_names};
  config.capture_mut().set_device_id(Some(capture_devs[preffered_cap_dev_id].id().clone()));
  //
  loop {
    let mut device = Device::new(None, &config).expect("failed to open playback device");
    device.set_data_callback(move |_device, output, _frames| {
      //decoder.read_pcm_frames(output);
    });
    device.start().expect("failed to start device");
  }
}
  