// Copyright 2025 Google LLC

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

/// A reader that opens a new file handle for each clone.
/// This allows multiple threads to read from the same file concurrently
/// without lock contention.
pub(crate) struct MultiFileSeeker {
    path: PathBuf,
    file: File,
}

impl MultiFileSeeker {
    pub(crate) fn new(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            path: path.to_path_buf(),
            file,
        })
    }
}

impl Clone for MultiFileSeeker {
    fn clone(&self) -> Self {
        Self::new(&self.path).expect("Failed to re-open file for seeker clone")
    }
}

impl Read for MultiFileSeeker {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for MultiFileSeeker {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.file.seek(pos)
    }
}
