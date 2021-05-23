#!/usr/bin/env rust


//! This file
//!
//! # Example
//!
//! ```rust
//! use iterate_text::string::characters::IterateStringCharacters;
//!
//! let s = String::from("test!");
//! let mut c = IterateStringCharacters::new(s);
//!
//! assert_eq!(c.next(), Some('t'));
//! assert_eq!(c.next(), Some('e'));
//! assert_eq!(c.next(), Some('s'));
//! assert_eq!(c.next(), Some('t'));
//! assert_eq!(c.next(), Some('!'));
//! assert_eq!(c.next(), None);
//! ```


/// Iterates over characters within string owned string
#[derive(Debug)]
pub struct IterateStringCharacters {
    data: Option<(String, usize)>,
}


impl IterateStringCharacters {
    /// Assumes ownership of `string` and returns new instance of `IterateStringCharacters`
    pub fn new<S>(string: S) -> Self
    where
        S: Into<String>
    {
        let string: String = string.into();
        let index: usize = 0;
        let data = Some((string, index));
        Self { data }
    }
}


impl Iterator for IterateStringCharacters {
    type Item = char;

    /// Increments index to the next character and returns current character, which avoids
    /// reallocation inefficacies. However, there are likely improvements that could be made by
    /// implementing some form of buffer that holds more than one line at a time in memory.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((string, index)) = self.data.take() {
            let mut iter = string.get(index..).unwrap().char_indices();
            if let Some((_, c)) = iter.next() {
                self.data = iter.next().map(|(i, _)| (string, index + i));
                return Some(c);
            }
        }
        None
    }
}

