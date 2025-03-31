use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
};

fn main() {
    // Getting our file and turning it into a hex dump
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut buf: Vec<u8> = Vec::new();
    let _ = File::open(file_path)
        .expect("Could not find file")
        .read_to_end(&mut buf)
        .expect("Could not read file");
    let hex_file = match File::create_new("xxd_dump.txt") {
        Ok(contents) => contents,
        Err(..) => File::create("xxd_dump.txt").unwrap(),
    };
    let mut writer = BufWriter::new(&hex_file);
    let _ = hxdmp::hexdump(&buf, &mut writer);

    // Reading the xxd dump and seeing what we can find
    let reader = BufReader::new(&hex_file);
}
