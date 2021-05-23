#!/usr/bin/env rust


use iterate_text::string::characters::IterateStringCharacters;


#[test]
fn characters() {
    let string = String::from("This is just a test!");

    let mut iter = IterateStringCharacters::new(string.clone());
    let mut expected = string.chars();

    loop {
        let c = iter.next();
        let e = expected.next();
        if c.is_none() && e.is_none() {
            break;
        }
        assert_eq!(c, e);
    }
}


#[test]
fn unicode() {
    let string = String::from("a̐éö̲");

    let mut character_iterator = IterateStringCharacters::new(string.clone());
    let mut expected_iterator = string.chars();

    assert_eq!(character_iterator.next(), expected_iterator.next());
}


#[test]
#[should_panic]
fn unequal() {
    let string = String::from("This is just a test!");

    let mut iter = IterateStringCharacters::new(string);
    let mut expected = "This is Just a test!".chars();

    loop {
        let c = iter.next();
        let e = expected.next();
        if c.is_none() && e.is_none() {
            break;
        }
        assert_eq!(c, e);
    }
}

