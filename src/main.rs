use std::env;

mod argparse;
use crate::argparse::argparse;

fn main() { 
  // First we collect arguments, and length of the arguments. 
  let args: Vec<String> = env::args().collect();
  let arguments: Vec<u32> = argparse(args);
  let arglength: usize = arguments.len();

  // Then we see which mode we're using.
  if arglength == 2 { // We're doing "Simple" mode.
    println!("Texture size: {}",arguments[0]);
    println!("Brush size:   {}",arguments[1]);
    println!("Scale:        {}",arguments[1] as f32 / arguments[0] as f32);
  }

  if arglength == 4 { // We're doing "Default" mode.
    println!("Texture width: {}\tTexture height: {}",arguments[0],arguments[1]);
    println!("Brush width:   {}\tBrush height:   {}",arguments[2],arguments[3]);
    println!("X-Scale:       {}\tY-Scale:        {}",arguments[2] as f32 / arguments[0] as f32, arguments[3] as f32 / arguments[1] as f32);
  }
}
