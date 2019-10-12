#![warn(rust_2018_idioms)]

use std::{
    fmt::{self, Display, Formatter},
    io,
    path::{Path, PathBuf},
};

use backtrace::Backtrace;

pub mod cpu;
use cpu::Cpu;
pub mod memory;
use memory::Memory;

/// An error that can occur in this crate.
#[derive(Debug)]
pub enum Error {
    /// The specified file was not found.
    FileNotFound { path: PathBuf, source: io::Error, backtrace: Backtrace },
    /// An I/O error.
    Io { source: io::Error, backtrace: Backtrace },
    /// The specified file was too large to load at the specified memory address.
    TooLargeFile { path: PathBuf, size: u64, start_address: u16 },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::FileNotFound { path, source, .. } => {
                write!(f, "{}: '{}'", source, path.display())
            }
            Error::Io { source, .. } => source.fmt(f),
            Error::TooLargeFile { path, size, start_address } => write!(
                f,
                "File {} ({} bytes) is too large to load at address {:#06X}",
                path.display(),
                size,
                start_address
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::FileNotFound { source, .. } | Error::Io { source, .. } => Some(source),
            Error::TooLargeFile { .. } => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io { source: e, backtrace: Backtrace::new() }
    }
}

/// A specialized `std::result::Result` type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// An Intel 8080 system.
#[derive(Default)]
pub struct Intel8080 {
    /// An Intel 8080 CPU.
    pub cpu: Cpu,
    /// A 64K memory.
    pub memory: Memory,
}

impl Intel8080 {
    /// Creates an Intel 8080 system by loading ROM files located at `paths` into memory starting
    /// at `start_address` and setting the program counter to `start_address`.
    ///
    /// # Errors
    ///
    /// This function will return an error ([`Error::FileNotFound`], [`Error::Io`], or
    /// [`Error::TooLargeFile`]) if a ROM file in `paths` cannot be read successfully or the total
    /// size of the ROM files is too large to be loaded into memory starting at `start_address`.
    ///
    /// [`Error::FileNotFound`]: enum.Error.html#variant.FileNotFound
    /// [`Error::Io`]: enum.Error.html#variant.Io
    /// [`Error::TooLargeFile`]: enum.Error.html#variant.TooLargeFile
    pub fn new<P: AsRef<Path>>(paths: &[P], start_address: u16) -> Result<Self> {
        let mut cpu = Cpu::default();
        cpu.pc = start_address;
        let mut memory = Memory::new();
        memory.load_files(paths, start_address)?;
        Ok(Self { cpu, memory })
    }
}
