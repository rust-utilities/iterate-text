#!/usr/bin/env rust


//! This file exposes and organizes components within [tests/] directory


mod file {
    mod lines {
        mod path;
        mod from_file;
        mod from_buf_reader;
    }

    mod characters {
        mod path;
        mod from_file;
        mod from_buf_reader;
    }
}

mod string {
    mod characters;
    mod lines;
}

