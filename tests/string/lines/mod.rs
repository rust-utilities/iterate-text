#!/usr/bin/env rust


use iterate_text::string::lines::IterateStringLines;


#[test]
fn iterate_lines () {
    let string = String::from("This is\na test string\nwith multiple lines\n");

    let mut lines = IterateStringLines::new(string);
    let mut expected = "This is\na test string\nwith multiple lines\n".lines();

    loop {
        let l = lines.next();
        let e = expected.next();
        if l.is_none() && e.is_none() {
            break;
        }

        assert_eq!(l, e.map(|s| {
            let mut s = s.to_string();
            s.push('\n');
            return s;
        }));
    }
}

