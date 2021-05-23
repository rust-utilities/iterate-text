#!/usr/bin/env rust


use iterate_text::file::characters::IterateFileCharacters;


#[test]
fn iterate_characters() {
    let path = "tests/file/characters/file.txt";

    let mut iter = IterateFileCharacters::new(path);
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

