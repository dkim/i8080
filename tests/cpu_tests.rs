#![warn(rust_2018_idioms)]

use i8080::Intel8080;

#[test]
fn cpu_tests_8080pre() -> Result<(), Box<dyn std::error::Error>> {
    Intel8080::new(&["tests/cpu_tests/8080PRE.COM"], 0)?;
    Ok(())
}
