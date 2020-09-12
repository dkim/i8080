use std::{
    fs::File,
    io::{self, Read},
    ops::{Deref, DerefMut, Index, IndexMut, Range, RangeFrom},
    path::Path,
};

use backtrace::Backtrace;

use crate::{Error, Result};

const MEMORY_SIZE: usize = 65536;

/// A 64K memory.
pub struct Memory([u8; MEMORY_SIZE]);

impl Memory {
    /// Creates a 64K memory.
    pub fn new() -> Self {
        Self([0; MEMORY_SIZE])
    }

    /// Loads ROM files located at `paths` into memory starting at `start_address`.
    ///
    /// # Errors
    ///
    /// This function will return an error ([`Error::FileNotFound`], [`Error::Io`], or
    /// [`Error::TooLargeFile`]) if a ROM file in `paths` cannot be read successfully or the total
    /// size of the ROM files is too largo to be loaded into memory starting at `start_address`.
    ///
    /// [`Error::FileNotFound`]: ../enum.Error.html#variant.FileNotFound
    /// [`Error::Io`]: ../enum.Error.html#variant.Io
    /// [`Error::TooLargeFile`]: ../enum.Error.html#variant.TooLargeFile
    pub fn load_files<P: AsRef<Path>>(
        &mut self,
        paths: &[P],
        mut start_address: u16,
    ) -> Result<u16> {
        for path in paths {
            start_address = self.load_file(path, start_address)?;
        }
        Ok(start_address)
    }

    /// Loads a ROM file located at `path` into memory starting at `start_address`.
    ///
    /// # Errors
    ///
    /// This function will return an error ([`Error::FileNotFound`], [`Error::Io`], or
    /// [`Error::TooLargeFile`]) if the ROM file at `path` cannot be read successfully or its size
    /// is too largo to be loaded into memory starting at `start_address`.
    ///
    /// [`Error::FileNotFound`]: ../enum.Error.html#variant.FileNotFound
    /// [`Error::Io`]: ../enum.Error.html#variant.Io
    /// [`Error::TooLargeFile`]: ../enum.Error.html#variant.TooLargeFile
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P, start_address: u16) -> Result<u16> {
        let path = path.as_ref();
        let mut file = File::open(path).map_err(|err| {
            if let io::ErrorKind::NotFound = err.kind() {
                Error::FileNotFound {
                    path: path.to_path_buf(),
                    source: err,
                    backtrace: Backtrace::new(),
                }
            } else {
                Error::Io { source: err, backtrace: Backtrace::new() }
            }
        })?;
        let size = file.metadata()?.len();
        if u64::from(start_address) + size > self.len() as u64 {
            return Err(Error::TooLargeFile { path: path.to_path_buf(), size, start_address });
        }
        let end_address = start_address + size as u16;
        file.read_exact(&mut self[start_address..end_address])?;
        Ok(end_address)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Memory {
    type Target = [u8; MEMORY_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Memory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, address: u16) -> &Self::Output {
        &self.0[usize::from(address)]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, address: u16) -> &mut Self::Output {
        &mut self.0[usize::from(address)]
    }
}

impl Index<Range<u16>> for Memory {
    type Output = [u8];

    fn index(&self, addresses: Range<u16>) -> &Self::Output {
        &self.0[usize::from(addresses.start)..usize::from(addresses.end)]
    }
}

impl IndexMut<Range<u16>> for Memory {
    fn index_mut(&mut self, addresses: Range<u16>) -> &mut Self::Output {
        &mut self.0[usize::from(addresses.start)..usize::from(addresses.end)]
    }
}

impl Index<RangeFrom<u16>> for Memory {
    type Output = [u8];

    fn index(&self, addresses: RangeFrom<u16>) -> &Self::Output {
        &self.0[usize::from(addresses.start)..]
    }
}

impl IndexMut<RangeFrom<u16>> for Memory {
    fn index_mut(&mut self, addresses: RangeFrom<u16>) -> &mut Self::Output {
        &mut self.0[usize::from(addresses.start)..]
    }
}
