#!/usr/bin/env rust


use std::fs::File;

use iterate_text::file::characters::IterateFileCharacters;


#[test]
fn from_file() {
    let path = "tests/file/characters/file.txt";
    let file_descriptor = File::open(path).unwrap();

    let mut iter = IterateFileCharacters::from(file_descriptor);
    let mut expected = "This is just a test!\n".chars();

    loop {
        let c = iter.next();
        let e = expected.next();
        if c.is_none() && e.is_none() {
            break;
        }
        assert_eq!(c, e);
    }
}

