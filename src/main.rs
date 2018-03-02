use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::io::prelude::*;

extern crate detect_indent;
extern crate atty;

use detect_indent::detect_indent;
use atty::Stream;

fn print(indent: &str) {
    io::stdout().write(indent.as_bytes()).expect("printing to stoud to work");
}

fn usage() {
    print("Usage:
    $ detect-indent <file>
    echo <string> | detect-indent

    Example
      $ echo '  foo\\n  bar' | detect-indent | wc --chars
      2
");
}

fn indent_from_stdin() {
    if atty::is(Stream::Stdin) {
        usage();
    } else {
        let mut contents = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut contents).expect("Could not read line");
        print(detect_indent(&contents).indent());
    }
}

fn indent_from_file(filename: String) {
    let mut contents = String::new();
    let mut file = File::open(&filename).expect(&format!("file not found: '{}'", &filename));

    file.read_to_string(&mut contents)
        .expect(&format!("something went wrong reading the file: '{}'", &filename));

    print(detect_indent(&contents).indent());
}

fn main() {
    env::args().nth(1).map_or_else(indent_from_stdin, indent_from_file);
}
