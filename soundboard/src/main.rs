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
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
  backend::{Backend, CrosstermBackend},
  layout::{Constraint, Layout},
  style::{Color, Modifier, Style},
  widgets::{Block, Borders, Cell, Row, Table, TableState},
  Frame, Terminal,
};
struct App<'a> {
  state: TableState,
  items: Vec<Vec<&'a str>>,
}
impl<'a> App<'a> {
  fn new() -> App<'a> {
    App {
      state: TableState::default(),
      items: vec![
        vec!["folder1", "filename1"],
        vec!["folder2", "filename2"],
      ],
    }
  }
  pub fn next(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
  }
  pub fn previous(&mut self) {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.items.len() - 1
        } else {
          i - 1
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
  }
}

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
fn configure_next_sound_dir(sound_dir: &str) {
  print!("\n{:?}\n", sound_dir);
}
fn configure_next_sound_file(sound_file: &str) {
  print!("\n{}\n", sound_file);
}
fn main() {
  //
  //
  let mut folder_position: usize = 0;
  let mut folder_position_max: usize = 0;
  let mut file_position: usize = 0;
  let mut file_position_max: usize = 0;
  let mut folders = list_folders("../synergyst-soundboard-util/sounds/").unwrap();
  //let mut folders = list_folders("./sounds/").unwrap();
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
  //
  let mut folder_infos:KeyedInfoFolder = KeyedInfoFolder{snd_dir_iter:&mut folder_position,actual_audio_dir_list_size:folder_position_max,is_cycle_forward_dir_down:false,is_cycle_backward_dir_down:false,prev_cycle_forward_dir_down:false,prev_cycle_backward_dir_down:false,sound_dir_list:&mut folders};
  //
  let context = Context::new(&[], None).expect("failed to create context");
  context.with_devices(|playback_devices, capture_devices| {
    println!("Playback Devices:");
    for (idx, device) in playback_devices.iter().enumerate() {
      println!("\t{}: {}", idx, device.name());
    }
    /*println!("Capture Devices:");
    for (idx, device) in capture_devices.iter().enumerate() {
      println!("\t{}: {}", idx, device.name());
    }*/
  }).expect("failed to get devices");
  //playbackInfo.
  //
  let mut dev_infos:DevInfo = DevInfo{dev_index_input:1,dev_index_output:1,is_voice_down:false,prev_voice_down:false};
  //
  /*let result = enumerate_devices();
  println!("{:?}", result);*/
  //
  //
  // setup terminal
  /*enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  // create app and run it
  let app = App::new();
  let res = run_app(&mut terminal, app);
  // restore terminal
  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;
  if let Err(err) = res {
    println!("{:?}", err)
  }*/
  //
  loop {
    dev_infos.is_voice_down = listen_for_key_press(0x12);
    file_infos.is_cycle_forward_file_down = listen_for_key_press(0x60);
    file_infos.is_cycle_backward_file_down = listen_for_key_press(0x61);
    /*let is_numpad2_pressed = listen_for_key_press(0x62);
    let is_numpad3_pressed = listen_for_key_press(0x63);*/
    folder_infos.is_cycle_backward_dir_down = listen_for_key_press(0x64);
    folder_infos.is_cycle_forward_dir_down = listen_for_key_press(0x65);
    /*let is_numpad6_pressed = listen_for_key_press(0x66);
    let is_numpad7_pressed = listen_for_key_press(0x67);
    let is_numpad8_pressed = listen_for_key_press(0x68);
    let is_numpad9_pressed = listen_for_key_press(0x69);*/
    /*print!("\r                                                                                                                                                                        \r");
    print!("NUM 0: {:?}, NUM 1: {:?}, NUM 2: {:?}, NUM 3: {:?}, NUM 4: {:?}, NUM 5: {:?}, NUM 6: {:?}, NUM 7: {:?}, NUM 8: {:?}, NUM 9: {:?}",
    is_numpad0_pressed, is_numpad1_pressed, is_numpad2_pressed, is_numpad3_pressed, is_numpad4_pressed, is_numpad5_pressed, is_numpad6_pressed, is_numpad7_pressed, is_numpad8_pressed, is_numpad9_pressed);*/
    //
    //
    //
    //
    //
    let mut temp_file_iter: i32 = *file_infos.snd_iter as i32;
    //
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
    //
    //
    let mut temp_folder_iter: i32 = *folder_infos.snd_dir_iter as i32;
    //
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
      for file in &mut *file_infos.audio_file_list {
        println!("[{}] -> [{}]", *folder_infos.snd_dir_iter, file);
      }
      //println!("{}", folder_infos.sound_dir_list[*folder_infos.snd_dir_iter]);
      //println!("{}", keyed_infos_file.audio_file_list[*keyed_infos_file.snd_iter]);
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
      for file in &mut *file_infos.audio_file_list {
        println!("[{}] -> [{}]", *folder_infos.snd_dir_iter, file);
      }
      //println!("{}", folder_infos.sound_dir_list[*folder_infos.snd_dir_iter]);
      //println!("{}", keyed_infos_file.audio_file_list[*keyed_infos_file.snd_iter]);
      //
    }
    while listen_for_key_press(0x12) && dev_infos.prev_voice_down != dev_infos.is_voice_down {
      //
      //play_audio_file(&keyed_infos_file.audio_file_list[*keyed_infos_file.snd_iter]);
      //thread::sleep(Duration::from_millis(500));
      //let mut decoder = Decoder::from_file(&keyed_infos_file.audio_file_list[*keyed_infos_file.snd_iter], None).expect("failed to initialize decoder from file");
      //
      let mut config = DeviceConfig::new(DeviceType::Playback);
      config.playback_mut().set_format(miniaudio::Format::S16);
      config.playback_mut().set_channels(2);
      config.set_sample_rate(48000);
      //config.playback().set_device_id(dev_infos.dev_index_output);
      /*config.set_stop_callback(|_device| {
        println!("Device Stopped.");
      });*/
      //
      //
      let mut dev_ids: Vec<Option<DeviceId>> = Vec::new();
      let mut dev_names: Vec<String> = Vec::new();
      let context = Context::new(&[], None).expect("failed to create context");
      context.with_playback_devices(|playback_devices| {
        /*println!("Playback Devices:");
        for (idx, device) in playback_devices.iter().enumerate() {
          println!("\t{}: {}", idx, device.name());
          //if idx == dev_infos.dev_index_output
        }*/
      }).expect("failed to get devices");
      //
      let mut preffered_dev_id: usize = 0;
      let preffered_dev_name: String = "VoiceMeeter Input (VB-Audio VoiceMeeter VAIO)".to_string();
      let dev_count = context.playback_device_count() as usize;
      let playback_devs = context.playback_devices();
      for idx in 0..dev_count {
        dev_ids.push(Some(playback_devs[idx].id().clone()));
        dev_names.push(playback_devs[idx].name().to_string());
        if dev_names[idx] == preffered_dev_name {
          preffered_dev_id = idx;
        }
        //println!("{}: {}", idx, dev_names[idx]);
      }
      let playback_devs_infos: PlaybackDevsInfo = PlaybackDevsInfo{playback_devs_id: dev_ids, playback_devs_name: dev_names};
      //println!();
      //
      //
      config.playback_mut().set_device_id(Some(playback_devs[preffered_dev_id].id().clone()));
      //
      let mut decoder = Decoder::from_file(&file_infos.audio_file_list[*file_infos.snd_iter], None).expect("failed to initialize decoder from file");
      //
      let mut device = Device::new(None, &config).expect("failed to open playback device");
      //
      device.set_data_callback(move |_device, output, _frames| {
        decoder.read_pcm_frames(output);
      });
      //
      device.start().expect("failed to start device");
      //
      //
      while listen_for_key_press(0x12) {
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
    //
    folder_infos.prev_cycle_forward_dir_down = folder_infos.is_cycle_forward_dir_down;
    folder_infos.prev_cycle_backward_dir_down = folder_infos.is_cycle_backward_dir_down;
    //
    dev_infos.prev_voice_down = dev_infos.is_voice_down;
    //
  }
  //Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
  loop {
    terminal.draw(|f| ui(f, &mut app))?;
    if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Char('q') => return Ok(()),
        KeyCode::Down => app.next(),
        KeyCode::Up => app.previous(),
        _ => {}
      }
    }
  }
}
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  let rects = Layout::default().constraints([Constraint::Percentage(100)].as_ref()).margin(5).split(f.size());
  let selected_style = Style::default().add_modifier(Modifier::REVERSED);
  let normal_style = Style::default().bg(Color::Blue);
  //let header_cells = ["Header1", "Header2", "Header3"].iter().map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
  let header_cells = ["CurrentFolder", "CurrentFile"].iter().map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
  let header = Row::new(header_cells).style(normal_style).height(1).bottom_margin(1);
  let rows = app.items.iter().map(|item| {
    let height = item.iter().map(|content| content.chars().filter(|c| *c == '\n').count()).max().unwrap_or(0)+ 1;
    let cells = item.iter().map(|c| Cell::from(*c));
    Row::new(cells).height(height as u16).bottom_margin(1)
  });
  let t = Table::new(rows).header(header).block(Block::default().borders(Borders::NONE).title("Soundboard")).highlight_style(selected_style).highlight_symbol("-->").widths(&[
    Constraint::Percentage(50),
    Constraint::Length(30),
    Constraint::Min(10),
  ]);
  f.render_stateful_widget(t, rects[0], &mut app.state);
}