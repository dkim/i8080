#![warn(rust_2018_idioms)]

/// An Intel 8080 CPU.
#[derive(Default)]
pub struct Cpu {
    /// Program counter.
    pub pc: u16,
}
