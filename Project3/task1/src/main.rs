use std::{
    fs::File,
    io::{BufWriter, Write},
};

const CHAR_OFFSET: u8 = 48;
const DIGIT_OFFSET: u8 = 97;
const LENGTH: u8 = 4;
// (26 + 10)**4
const NUM_PASSES: u32 = 1679616;

fn main() {
    let file = File::create("password_attempts.txt").expect("Could not find file");
    let mut writer = BufWriter::new(file);
    for i in 0..NUM_PASSES {
        let mut str: Vec<u8> = vec![108, 105, 107, 101, 115];
        let mut val = i;
        for _ in 0..LENGTH {
            let digit = (val % 36) as u8;
            if digit < 10 {
                str.push(digit + CHAR_OFFSET);
            } else {
                str.push((digit - 10) + DIGIT_OFFSET);
            }
            val /= 36;
        }
        let hash = md5::compute(&str);
        let pass = String::from_utf8(str).unwrap();
        let hash_str = format!("{:x}", hash);
        let final_pass = pass + ": " + &hash_str;
        let _ = writeln!(writer, "{}", final_pass);
    }
}
