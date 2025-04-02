use std::{env, fs::File, io::Read};

const BLOCK_SIZE: usize = 16;

// // Employing optimal substructure to get a dynamic programming solution
// fn find_suffix(i: usize, j: usize, buf: &Vec<u8>, memo: &mut Vec<Vec<usize>>) -> usize {
//     if j == buf.len() {
//         return 0;
//     }
//     if memo[i][j] != usize::MAX {
//         return memo[i][j];
//     }
//     if buf[i] == buf[j] && j > i {
//         memo[i][j] = min(1 + find_suffix(i + 1, j + 1, buf, memo), j - i - 1);
//     } else {
//         memo[i][j] = 0;
//     }
//     memo[i][j]
// }
//
// // Using the optimal substructure to find our longest repeated substring
// fn longest_substring(buf: &Vec<u8>) -> Vec<(usize, usize)> {
//     // Getting our buffer length and making an n x n memoization table
//     let len = buf.len();
//     let mut memo: Vec<Vec<usize>> = vec![vec![usize::MAX; len]; len];
//
//     // Filling out our table
//     for (i, _) in buf.iter().enumerate() {
//         for (j, _) in buf.iter().enumerate() {
//             if j <= i {
//                 continue;
//             }
//             find_suffix(i, j, buf, &mut memo);
//         }
//     }
//
//     let mut ans_len = 0;
//
//     // Finding our optimal answer
//     for (i, _) in buf.iter().enumerate() {
//         for (j, _) in buf.iter().enumerate() {
//             if j <= i {
//                 continue;
//             }
//             if memo[i][j] > ans_len && memo[i][j] != usize::MAX {
//                 ans_len = memo[i][j];
//             }
//         }
//     }
//
//     let mut longest_substrings = Vec::new();
//
//     // Finding all occurances of our optimal answer
//     for (i, _) in buf.iter().enumerate() {
//         for (j, _) in buf.iter().enumerate() {
//             if j <= i {
//                 continue;
//             }
//             if memo[i][j] == ans_len {
//                 let new_ans_1 = (i, i + ans_len - 1);
//                 let new_ans_2 = (j, j + ans_len - 1);
//                 longest_substrings.push(new_ans_1);
//                 longest_substrings.push(new_ans_2);
//             }
//         }
//     }
//     longest_substrings
// }

// Finds all instances of the block that starts at start in buf
fn find_block(buf: &[u8], start: usize) -> Vec<usize> {
    // Doing some sanity checks to make sure the start is valid
    if start % BLOCK_SIZE != 0 {
        panic!("Invalid starting index; not a multiple of block size");
    }
    // Getting the initial reference block
    let initial_request = &buf[start..start + BLOCK_SIZE];
    let mut found = Vec::new();
    for (i, char) in buf.iter().enumerate() {
        // If we find a character that matches up with the first character of our reference block
        if *char == initial_request[0] {
            let mut curr = char;
            let mut buf_index = i;
            let mut initial_index = 0;
            // Loop through until we reach BLOCK SIZE and see if the similarity matches up
            while initial_index < BLOCK_SIZE && *curr == initial_request[initial_index] {
                buf_index += 1;
                initial_index += 1;
                curr = &buf[buf_index];
            }
            // If we've reached BLOCK SIZE and the request headers are similar, add it to our
            // vector
            if initial_index == BLOCK_SIZE {
                found.push(i);
            }
        }
    }
    found
}

// We know that if our initial request header is followed by two different blocks at different
// points in our buffer, then both of those blocks are account labels.
fn find_account_numbers(buf: &[u8]) -> Vec<usize> {
    // Both of these vectors are implicitly sorted based on how find_block operates.
    let first_req = find_block(buf, 0);
    let first_acc = find_block(buf, BLOCK_SIZE);
    let mut accs = first_acc.clone();
    // For each instance of the first request header, check which blocks follow it
    for block in first_req.iter() {
        let following = block + BLOCK_SIZE;
        match accs.binary_search(&following) {
            // If we found the following block in our first account vector, continue
            Ok(_) => continue,
            // Otherwise, we want to find all instances of it
            Err(_) => {
                let new_acc = find_block(buf, following);
                accs = [accs, new_acc].concat();
                accs.sort();
            }
        }
    }
    accs
}

// Gets the number of transfer requests in a given length
// fn transfer_number(buf: &[u8]) -> u32 {
//     let mut len = buf.len();
//     while len % 2 == 0 {
//         len /= 2;
//     }
//     (len / 5).try_into().unwrap()
// }

fn main() {
    // Getting our file
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut buf: Vec<u8> = Vec::new();
    let _ = File::open(file_path)
        .expect("Could not find file")
        .read_to_end(&mut buf)
        .expect("Could not read file");

    // Finding the longest repeated substring
    let accs = find_account_numbers(&buf);
    dbg!(accs);
    println!("{}", &buf.len());
}
