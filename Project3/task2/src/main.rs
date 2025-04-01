use std::{cmp::min, env, fs::File, io::Read};

// Employing optimal substructure to get a dynamic programming solution
fn find_suffix(i: usize, j: usize, buf: &Vec<u8>, memo: &mut Vec<Vec<usize>>) -> usize {
    if j == buf.len() {
        return 0;
    }
    if memo[i][j] != usize::MAX {
        return memo[i][j];
    }
    if buf[i] == buf[j] {
        memo[i][j] = min(1 + find_suffix(i + 1, j + 1, buf, memo), j - i - 1);
    } else {
        memo[i][j] = 0;
    }
    memo[i][j]
}

fn main() {
    // Getting our file and turning it into a hex dump
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut buf: Vec<u8> = Vec::new();
    let _ = File::open(file_path)
        .expect("Could not find file")
        .read_to_end(&mut buf)
        .expect("Could not read file");
    // let hex_file = match File::create_new("xxd_dump.txt") {
    //     Ok(contents) => contents,
    //     Err(..) => File::create("xxd_dump.txt").unwrap(),
    // };
    // let mut writer = BufWriter::new(&hex_file);
    // hxdmp::hexdump(&buf, &mut writer).expect("Could not perform hex dump");

    // Dynamic programming approach to find the longest repeated substring
    // We're using lines instead of characters here but the approach is the same.
    // We can do memoization to store
}
