# rust-integer-converter
A simple application built in Rust to convert an integer value into its binary and hexadecimal representation.

This application was created solely as a learning project to explore Rust concepts, including primitives, data types, and other fundamental aspects of the language.

## Build
To compile the package:
```sh
cargo build
```

## Usage
To convert an integer to binary and hexadecimal:
```sh
cargo run <integer>
```

Alternatively, to run the compiled debug binary directly:
```sh
./target/debug/rust-integer-converter <integer>
```

Replace `<integer>` with the unsigned integer value you want to convert.

## Example Output
```sh
cargo build
cargo run 25
  Int Value: 25
  Hex Value: 0x19
  Bin Value: 0b11001
```
