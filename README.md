# i8080

[![build status](https://github.com/dkim/i8080/workflows/build/badge.svg)](https://github.com/dkim/i8080/actions?query=workflow%3Abuild+branch%3Amaster)
[![docs](https://img.shields.io/badge/docs-1.0.2-blue.svg)](https://dkim.github.io/i8080/1.0.2/i8080/)

i8080 is an [Intel 8080] emulation library in Rust.

[Intel 8080]: https://en.wikipedia.org/wiki/Intel_8080

## Features

* Supports all the 8080 instructions.
* Passes rigorous 8080 CPU tests (TST8080.COM, 8080PRE.COM, 8080EXM.COM, and
  CPUTEST.COM).

## Requirements

### Rust

This program targets the latest stable version of Rust 1.47.0 or later.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
i8080 = { git = "https://github.com/dkim/i8080", tag = "1.0.2" }
```

## Example

This example shows how to load and execute a ROM file, printing each
instruction and the number of states that it took.

```rust
use std::process;

use i8080::Intel8080;

fn main() {
    if let Err(err) = example() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let mut i8080 = Intel8080::new(&["rom_file"], 0)?;
    while let Ok((instruction, states)) = i8080.fetch_execute_instruction() {
        println!("{:?} ({} states)", instruction, states);
    }
    Ok(())
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
