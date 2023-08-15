use std::env;

fn argparse (arguments: Vec<String>) -> Vec<u32> {
  let mut numbers: Vec<u32> = Vec::new();
  let mut buffer: u32;  
  let arglength: usize = arguments.len();  
 
  // Rudimentary --help system. To be updated.
  if arguments[1].eq("--help"){
    println!("HELP SCREEN FOR QSCALETOOL");
    println!("Usage:\n./qscaletool <Texture Width> <Texture Height> <Brush Width> <Brush Height>");
    panic!();
  }

  if arglength > 5 {
    println!("Warning! All arguments past the fourth ({}) will be ignored!",arguments[4]);
  }

  // To be updated to include "presets" of sorts.
  if arglength < 4 {
    panic!("Error! Not enough arguments have been passed into program! Use --help for more info!");
  }

  // Writing arguments to numbers.
  for i in 1..arglength {

    buffer = arguments[i].parse::<u32>().unwrap();
    numbers.push(buffer)
  }

  numbers
}

fn main() { 
 
  let args: Vec<String> = env::args().collect();
  let numbers: Vec<u32> = argparse(args);
  println!("Pre-Calculation Stats:");
  println!("Texture Width (X-Size):\t{}\tTexture Height (Y-Size):\t{}",numbers[0],numbers[1]);
  println!("Brush Width (X-Size):\t{}\tBrush Height (Y-Size):\t\t{}",numbers[2],numbers[3]);

  let xscale: f32 = numbers[2] as f32 / numbers[0] as f32;
  let yscale: f32 = numbers[3] as f32 / numbers[1] as f32;
  println!("X Scale:\t{}\tY Scale:\t{}",xscale,yscale)
}
