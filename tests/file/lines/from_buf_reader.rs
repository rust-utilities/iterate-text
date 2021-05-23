#!/usr/bin/env rust


use std::io::BufReader;
use std::fs::File;

use iterate_text::file::lines::IterateFileLines;


#[test]
fn from_buf_reader() {
    let list = vec![
        "First line\n",
        "Second line\n",
        "Third line\n"
    ];
    let mut expected = list.iter().map(|line| line.to_string());

    let path = "tests/file/lines/file.txt";
    let file_descriptor = File::open(path).unwrap();
    let file_buffer = BufReader::new(file_descriptor);

    let mut iter = IterateFileLines::from(file_buffer);

    loop {
        let c = iter.next();
        let e = expected.next();
        if c.is_none() && e.is_none() {
            break;
        }
        assert_eq!(c, e);
    }
}

