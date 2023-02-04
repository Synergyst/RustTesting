#![allow(dead_code)]
#![allow(unused_imports)]
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
fn hex_to_i32(hex: &str) -> i32 {
  let without_prefix = hex.trim_start_matches("0x");
  i32::from_str_radix(without_prefix, 16).unwrap()
}
fn hex_to_i64(hex: &str) -> i64 {
  let without_prefix = hex.trim_start_matches("0x");
  i64::from_str_radix(without_prefix, 16).unwrap()
}

fn main() {
  println!("The area of the rectangle is {} square pixels.", area(640, 360));
  print!("{:?}", get_key_state(KeyInput::NextLibrary));
}
fn area(width: u32, height: u32) -> u32 {
  width * height
}