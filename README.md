# qscaletool
A simple CLI tool used to determine the proper scaling for a texture in the Quake Engine.

Version: 0.1.0  Release Date: 2023/08/14


# Usage
This software uses the Cargo build system. It requires that the user already has Rust installed. If not, please go to https://rustup.rs to install Rust.

The software can be compiled by running ``cargo build --release``. The binary is located in ``./target/release``.

You can run the binary in the terminal using the following "formula":

```
./qscaletool <Texture Width> <Texture Height> <Brush Width> <Brush Height>
```

Texture width and height are the obvious width and height of the texture you wish to scale. Brush width and height are the width and height of the brush (Specifically, the surface of the brush) you wish to scale to. Each of the four values are expected to be whole numbers. After running the binary, it will print the proper scale values.


# Changelog

Version 0.1.0 (2023/08/14)
  - Added ``./src/main.rs``. Includes the function ``main()`` and ``argparse(arguments: Vec<String>) -> Vec<u32>``. The former is the main function that handles printing the results, the latter is a function that processes the environment arguments.

