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
 
  // Writing arguments to numbers.
  for i in 1..arglength {
    match arguments[i].as_str() {
    
    "--help" => help(),  
   
    "a" => {
        numbers.push(64);
        numbers.push(64);
      }
 
    "b" => {
        numbers.push(128);
        numbers.push(128);
      }
    
    "c" => {
        numbers.push(192);
        numbers.push(192);
      }
   

    "d" => {
        numbers.push(128);
        numbers.push(192);
      }

    "d0" => {
        numbers.push(64);
        numbers.push(192);
    }
    
 
    "e" => {
        numbers.push(32);
        numbers.push(64);
      }

     _ => {
      buffer = arguments[i].parse::<u32>().unwrap();
      numbers.push(buffer);
      }

    }
 
  }

  // Then we return the vector.
  numbers
}

fn help () { 
  println!("\nHELP SCREEN FOR QSCALETOOL");
  println!("Usage 00: Default\n./qscaletool <Texture Width> <Texture Height> <Brush Width> <Brush Height>\nThis can be considered the '2D' usage of qscaletool.\n");
  println!("Usage 01: Simple\n./qscaletool <Texture Width/Height> <Brush Width/Height>\nThis only scales one axis of the texture. Recommended if you already have the texture scaled for one dimension.\n");
  println!();
  println!("Usage 02: Templates\n./qscaletool <Template ID> <Brush Width> <Brush Height>");
  println!("Uses one of many templates, which are categorized as numbers, to automatically set the texture width and height.");
  println!("List of templates:\na - 64 x 64\nb - 128 x 128\nc - 192 x 192\nd - 128 x 192\nd0 - 64 x 128\ne - 32 x 64");
  panic!("Used Help.");
}
