#!/usr/bin/env rust
#![deny(missing_docs, unsafe_code)]
#![deny(clippy::all)]


//! A library for iterating over lines or characters from files or strings
//!
//! This file exposes and organizes components within library files of [src/] directory


/// Text file iterators may read from path, buffer, or file descriptor
///
/// # Example
///
/// ```rust
/// use iterate_text::file::lines::IterateFileLines;
///
/// let mut lines = IterateFileLines::new(".gitignore");
/// assert_eq!(lines.next(), Some("/target\n".to_string()));
/// assert_eq!(lines.next(), Some("Cargo.lock\n".to_string()));
/// assert_eq!(lines.next(), Some("*.swp\n".to_string()));
/// assert_eq!(lines.next(), None);
/// ```
pub mod file {
    pub mod characters;
    pub mod lines;
}


/// String iterators may assume ownership of any type that implements `ToString`
///
/// # Example
///
/// ```rust
/// use iterate_text::string::characters::IterateStringCharacters;
///
/// let mut expected = "This is a Test!".chars();
///
/// let string: String = String::from("This is a Test!");
/// let mut characters = IterateStringCharacters::new(string);
///
/// loop {
///     let c = characters.next();
///     let e = expected.next();
///     if c.is_none() && e.is_none() {
///         break;
///     }
///     assert_eq!(c, e);
/// }
/// ```
pub mod string {
    pub mod characters;
    pub mod lines;
}

