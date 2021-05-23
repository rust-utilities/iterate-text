#!/usr/bin/env rust


use iterate_text::file::lines::IterateFileLines;


#[test]
fn iterate_lines() {
    let list = vec![
        "First line\n",
        "Second line\n",
        "Third line\n"
    ];
    let mut expected = list.iter().map(|line| line.to_string());

    let path = "tests/file/lines/file.txt";
    let mut iter = IterateFileLines::new(path);

    loop {
        let l = iter.next();
        let e = expected.next();
        if l.is_none() && e.is_none() {
            break;
        }
        assert_eq!(l, e);
    }
}

