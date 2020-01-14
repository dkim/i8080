#![warn(rust_2018_idioms)]

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
