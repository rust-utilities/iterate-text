#!/usr/bin/env rust


//! Reads from file path, buffer, or descriptor and iterates over lines until EOF is reached
//!
//! # Example
//!
//! ```rust
//! use iterate_text::file::lines::IterateFileLines;
//!
//! let p = "tests/file/lines/file.txt";
//! let mut l = IterateFileLines::new(p);
//!
//! assert_eq!(l.next(), Some("First line\n".to_string()));
//! assert_eq!(l.next(), Some("Second line\n".to_string()));
//! assert_eq!(l.next(), Some("Third line\n".to_string()));
//! assert_eq!(l.next(), None);
//! ```


use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


/// Wraps `BufReader<File>` and wraps `.read_line()` from `BufReader` as an Iterator
#[derive(Debug)]
pub struct IterateFileLines {
    file_buffer: BufReader<File>,
}


impl IterateFileLines {
    /// Initializes structure with `BufReader` for file `path`
    pub fn new<S: Into<String>>(path: S) -> Self {
        let file_descriptor = File::open(path.into()).unwrap();
        let file_buffer = BufReader::new(file_descriptor);
        Self { file_buffer }
    }
}


impl Iterator for IterateFileLines {
    type Item = String;

    /// Returns `Option<String>` when a line is available from `.read_line()` or `None` when EOF is
    /// reached.
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.file_buffer.read_line(&mut line) {
            Ok(state) => match state {
                0 => None,
                _ => Some(line),
            },
            _ => None,
        }
    }
}


impl From<File> for IterateFileLines {
    /// Initializes structure from `File` descriptor
    fn from (file_descriptor: File) -> Self {
        Self { file_buffer: BufReader::new(file_descriptor) }
    }
}

impl From<BufReader<File>> for IterateFileLines {
    /// Initializes structure from `BufReader<File>`
    fn from(buffer_reader: BufReader<File>) -> Self {
        Self { file_buffer: buffer_reader }
    }
}

