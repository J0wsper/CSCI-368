use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader, Read},
};

fn main() {

    // Getting our file and turning it into a hex dump
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let raw_contents = fs::read_to_string(file_path).expect("Could not find input file");
    let hex_contents = xxd::convert::bytes(&raw_contents);
    let hex_file = File::create_new("dump")

    println!("Hello, world!");
}
