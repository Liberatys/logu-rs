use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_json_file(file_name: &str) -> String {
    let contents = fs::read_to_string(file_name).expect("Input file not found");
    return contents;
}
