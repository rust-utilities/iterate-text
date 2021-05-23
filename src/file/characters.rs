#!/usr/bin/env rust


//! Reads from path, buffer, or descriptor and iterates over characters until EOF is reached
//!
//! > Depends on [IterateFileLines]
//!
//! # Example
//!
//! ```rust
//! use iterate_text::file::characters::IterateFileCharacters;
//!
//! let p = "tests/file/characters/file.txt";
//! let mut c = IterateFileCharacters::new(p);
//!
//! assert_eq!(c.next(), Some('T'));
//! assert_eq!(c.next(), Some('h'));
//! assert_eq!(c.next(), Some('i'));
//! assert_eq!(c.next(), Some('s'));
//! ```
//!
//! [IterateFileLines]: ../lines/index.html


use std::io::BufReader;
use std::fs::File;

use crate::file::lines::IterateFileLines;
use crate::string::characters::IterateStringCharacters;


/// Wraps [`IterateFileLines`][IterateFileLines] and implements Iterator for `.chars()`
///
/// [IterateFileLines]: ../lines/index.html
#[derive(Debug)]
pub struct IterateFileCharacters {
    line_buffer: IterateFileLines,
    char_buffer: Option<IterateStringCharacters>,
}


impl IterateFileCharacters {
    /// Initializes structure for file `path`
    pub fn new<S>(path: S) -> Self
    where
        S: Into<String>
    {
        let line_buffer = IterateFileLines::new(path);
        let char_buffer = None;
        Self { line_buffer, char_buffer }
    }
}

impl Iterator for IterateFileCharacters {
    type Item = char;

    /// Returns `Some<char>` when a character is available from `.read_line()`
    /// or `None` when EOF (End Of File) is reached.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(chars) = self.char_buffer.as_mut() {
                if let Some(c) = chars.next() {
                    return Some(c);
                }
            }

            if let Some(line) = self.line_buffer.next() {
                self.char_buffer = Some(IterateStringCharacters::new(line));
            } else {
                return None;
            }
        }
    }
}


impl From<File> for IterateFileCharacters {
    /// Initializes structure from `File` descriptor
    fn from(file_descriptor: File) -> Self {
        Self {
            line_buffer: IterateFileLines::from(file_descriptor),
            char_buffer: None,
        }
    }
}

impl From<BufReader<File>> for IterateFileCharacters {
    /// Initializes structure from `BufReader<File>`
    fn from(buffer_reader: BufReader<File>) -> Self {
        Self {
            line_buffer: IterateFileLines::from(buffer_reader),
            char_buffer: None,
        }
    }
}

