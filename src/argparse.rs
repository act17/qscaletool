pub fn argparse (arguments: Vec<String>) -> Vec<u32> {
  let mut numbers: Vec<u32> = Vec::new();
  let mut buffer: u32;  
  let arglength: usize = arguments.len();  

  // In the case that the User did not supply enough arguments.
  if arglength < 2 {
    println!("Error! User did not supply enough arguments!");
    println!("Printing help message and terminating...");
    help();
  }

  // In the case the user wants to see the help screen.
  if arguments[1].eq("--help") {
    help();
  }
   
  // Writing arguments to numbers.
  for i in 1..arglength {
    buffer = arguments[i].parse::<u32>().unwrap();
    numbers.push(buffer)
  }

  // Then we return the vector.
  numbers
}

fn help () { 
  println!("\nHELP SCREEN FOR QSCALETOOL");
  println!("Usage 00: Default\n./qscaletool <Texture Width> <Texture Height> <Brush Width> <Brush Height>\nThis can be considered the '2D' usage of qscaletool.");
  println!("Usage 01: Simple\n./qscaletool <Texture Width/Height> <Brush Width/Height>\nThis only scales one axis of the texture. Recommended if you already have the texture scaled for one dimension.\n");
  panic!("Used Help.");
}
