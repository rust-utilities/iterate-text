#!/usr/bin/env rust


//! Provides structure to iterate over lines within a string, or block of text, that includes
//! new-line separators.
//!
//! > Depends on [IterateStringCharacters] 
//!
//! # Example
//!
//! ```rust
//! use iterate_text::string::lines::IterateStringLines;
//!
//! let s = String::from("This is\na \\n test string\n");
//! let mut l = IterateStringLines::new(s);
//!
//! assert_eq!(l.next(), Some("This is\n".to_string()));
//! assert_eq!(l.next(), Some("a \\n test string\n".to_string()));
//! assert_eq!(l.next(), None);
//! ```
//!
//! [IterateStringCharacters]: ../characters/index.html 


use crate::string::characters::IterateStringCharacters;


/// Iterates over lines within string and includes new-line separator
#[derive(Debug)]
pub struct IterateStringLines {
    iter: IterateStringCharacters,
}


impl IterateStringLines {
    /// Passes ownership of `string` to instance of [`IterateStringCharacters`][IterateStringCharacters]
    ///
    /// [IterateStringCharacters]: ../characters/index.html
    pub fn new<S>(string: S) -> Self
    where
        S: Into<String>
    {
        let iter = IterateStringCharacters::new(string);
        Self { iter }
    }
}


impl Iterator for IterateStringLines {
    type Item = String;

    /// Builds and returns `String` by consuming characters from [`IterateStringCharacters`][IterateStringCharacters] instance
    ///
    /// > Note, escaped newline characters (`\\n`) _should_ be ignored and preserved correctly
    /// > during parsing
    ///
    /// [IterateStringCharacters]: ../characters/index.html
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        let mut escape = false;
        loop {
            if let Some(character) = self.iter.next() {
                line.push(character);
                match character {
                    '\\' => escape = true,
                    _ => {
                        if escape {
                            escape = false;
                        } else if character == '\n' {
                            return Some(line);
                        }
                    },
                };
            } else if !line.is_empty() {
                return Some(line);
            } else {
                return None;
            }

        }
    }
}

