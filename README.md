# qscaletool
A simple CLI tool used to determine the proper scaling for a texture in the Quake Engine.

Version: 0.2.0  Release Date: 2023/08/15


# Usage
This software uses the Cargo build system. It requires that the user already has Rust installed. If not, please go to https://rustup.rs to install Rust.

The software can be compiled by running ``cargo build --release``. The binary is located in ``./target/release``.

You can run the binary in the terminal using one of the two following formulas:

```
"Default" mode, calculates the scale in two dimensions:
./qscaletool <Texture Width> <Texture Height> <Brush Width> <Brush Height>

"Simple" mode, calculates the scale in only one dimension:
./qscaletool <Texture Width/Height> <Brush Width/Height>
```

# Changelog

Version 0.2.0 (2023/08/15)
  - Added ``./src/argparse.rs``. Includes the function ``pub fn argparse(arguments: Vec<String>) -> Vec<u32>`` and ``fn help()``. The former is a modified and refined version of ``argpase()`` that was included in ``main.rs``. The latter is a "macro" to print the Help screen. Until further development in the way of error handling occurs, this help screen will always end in a ``panic!()``.
  - Removed ``argparse()`` from ``main.rs``. Added crates for the usage of functions in ``argparse.rs``. Modified how the program works, as there's now two ways to use the program.
  - Updated ``./Cargo.toml``. 

Version 0.1.0 (2023/08/14)
  - Added ``./src/main.rs``. Includes the function ``main()`` and ``argparse(arguments: Vec<String>) -> Vec<u32>``. The former is the main function that handles printing the results, the latter is a function that processes the environment arguments.

