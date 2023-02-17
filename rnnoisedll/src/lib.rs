#![allow(unused_imports)]
#![allow(unused_variables)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;

#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
  a + b
}

//Given two files containing little-endian `i16`s, computes the correlation of the signals.
#[no_mangle]
pub extern "C" fn process_files(f1: String, f2: String) -> i32 {
  let data1 = std::fs::read(f1.clone()).unwrap_or_else(|e| {
    eprintln!("Failed to open \"{}\": {}", f1, e);
    return vec![1];
  });
  let data2 = std::fs::read(f2.clone()).unwrap_or_else(|e| {
    eprintln!("Failed to open \"{}\": {}", f1, e);
    return vec![1];
  });
  if data1.len() != data2.len() {
    eprintln!("File sizes differ");
    return 1;
  }
  if data1.len() % 2 != 0 {
    eprintln!("File sizes are odd");
    return 1;
  }

  let mut x = Vec::new();
  let mut y = Vec::new();
  for i in data1.chunks(2) {
    x.push(i16::from_le_bytes([i[0], i[1]]) as f64);
  }
  for i in data2.chunks(2) {
    y.push(i16::from_le_bytes([i[0], i[1]]) as f64);
  }

  /*let mut x: Vec<i16> = Vec::new();
  let mut y: Vec<i16> = Vec::new();
  for i in data1.chunks(2) {
    x.push(i16::from_le_bytes([i[0], i[1]]) as i16);
  }
  for i in data2.chunks(2) {
    y.push(i16::from_le_bytes([i[0], i[1]]) as i16);
  }
  let mut z: Vec<i16> = Vec::new();
  let vad_probability: f32;
  unsafe { vad_probability = rnnProcessFrame_s16_to_s16(&mut x, &mut z, 480, 1); }
  println!("{:#?}", vad_probability);*/

  let xx: f64 = x.iter().map(|&n| n * n).sum();
  let yy: f64 = y.iter().map(|&n| n * n).sum();
  let xy: f64 = x.iter().zip(y.iter()).map(|(&n, &m)| n * m).sum();
  let corr = xy / (xx.sqrt() * yy.sqrt());
  println!("{}", corr);

  if (corr - 1.0).abs() > 1e-6 {
    eprintln!("Bad correlation");
    return 1;
  }
  0
}
