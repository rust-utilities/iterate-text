#!/usr/bin/env rust


extern crate argparse;
use argparse::{ ArgumentParser, StoreOption };

extern crate regex;
use regex::Regex;


extern crate iterate_text;
use iterate_text::file::lines::IterateFileLines;
use iterate_text::file::characters::IterateFileCharacters;


fn ends_with_new_line(s: &str) -> bool {
    let regex = Regex::new(r"\n|\r\n$").unwrap();
    return regex.is_match(s);
}


fn main() {
    let mut file_path: Option<String> = None;
    let mut line_limit: Option<usize> = None;
    let mut character_limit: Option<usize> = None;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Read file for number of lines or characters");

        ap.refer(&mut file_path)
          .add_option(&["--file"], StoreOption, "File to read and iterate through");

        ap.refer(&mut line_limit)
          .add_option(&["-n"], StoreOption, "Number of lines to iterate");

        ap.refer(&mut character_limit)
          .add_option(&["-c"], StoreOption, "Number of characters to iterate");

        ap.parse_args_or_exit();
    }

    if file_path.is_none() {
        panic!("No file path provided");
    }

    if (line_limit.is_some() && character_limit.is_some())
    || (line_limit.is_none() && character_limit.is_none())
    {
        panic!("Please choose either -n **or** -c");
    }

    let mut count = 0;
    if line_limit.is_some() {
        let line_limit = line_limit.unwrap();
        let line_iterator = IterateFileLines::new(file_path.unwrap());
        for line in line_iterator {
            count += 1;
            print!("{}", line);
            if count >= line_limit {
                if !ends_with_new_line(&line) {
                    println!("");
                }
                break;
            }
        }
    } else if character_limit.is_some() {
        let character_limit = character_limit.unwrap();
        let character_iterator = IterateFileCharacters::new(file_path.unwrap());
        for character in character_iterator {
            count += 1;
            print!("{}", character);
            if count >= character_limit {
                if !ends_with_new_line(&character.to_string()) {
                    println!("");
                }
                break;
            }
        }
    }
}

