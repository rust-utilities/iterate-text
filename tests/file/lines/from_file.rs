#!/usr/bin/env rust


use std::fs::File;

use iterate_text::file::lines::IterateFileLines;


#[test]
fn from_file() {
    let list = vec![
        "First line\n",
        "Second line\n",
        "Third line\n"
    ];
    let mut expected = list.iter().map(|line| line.to_string());

    let path = "tests/file/lines/file.txt";
    let file_descriptor = File::open(path).unwrap();
    let mut iter = IterateFileLines::from(file_descriptor);

    loop {
        let c = iter.next();
        let e = expected.next();
        if c.is_none() && e.is_none() {
            break;
        }
        assert_eq!(c, e);
    }
}

