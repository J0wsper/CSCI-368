use std::{env, fs::File, io::Read};

const BLOCK_SIZE: usize = 16;

// Enum to carry the request header types and their associated locations
#[derive(Debug)]
enum ReqHeader {
    Balance(usize),
    Invoice(usize),
    Transfer(usize),
}

#[derive(Debug)]
struct IsReqType {
    balance: bool,
    invoice: bool,
    transfer: bool,
}

impl IsReqType {
    pub fn new() -> Self {
        Self {
            balance: true,
            invoice: true,
            transfer: true,
        }
    }
}

// Helper function to create our file
fn create_buf() -> Vec<u8> {
    // Collects the command line arguments
    let args: Vec<String> = env::args().collect();
    // Gets the provided file path
    let file_path = &args[1];
    // Creates our buffer and writes the contents of the file to it
    // NOTE: This will be slow for giant files. For that, a BufReader is superior
    let mut buf: Vec<u8> = Vec::new();
    let _ = File::open(file_path)
        .expect("Could not find file")
        .read_to_end(&mut buf)
        .expect("Could not read file");
    buf
}

// Utility function to turn block numbers into (zero-indexed) line numbers
fn block_to_line(blocks: &[usize]) -> Vec<usize> {
    blocks.iter().map(|x| x / BLOCK_SIZE).collect()
}

// // Utility function to find the number of transfer requests based on the fact that 5 is prime
// fn num_transfers(buf: &[u8]) -> u32 {
//     let mut len = buf.len();
//     while len % 2 == 0 {
//         len /= 2;
//     }
//     (len / 5).try_into().unwrap()
// }

// Finds the minimum difference between any two consecutive elements in a vector
fn min_diff(vec: &[usize]) -> usize {
    vec.windows(2)
        .map(|x| x[0].abs_diff(x[1]))
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
}

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
// NOTE: There is no guarantee that this will find all of the account numbers. It does a best
// effort search based on the fact that we know the first block is a request header.
fn find_accs(buf: &[u8]) -> Vec<usize> {
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

// Trys all of the given headers to see if they're valid
fn find_reqs(buf: &[u8], accs: &[usize], init: &[usize]) -> ReqHeader {
    // Finding how many of the initial requests we have
    let init_num = init.len();

    // Bookkeeping the types of request headers we've tried for the initial variable
    let mut is_init = IsReqType::new();

    // Finding the smallest difference between any two consecutive initial requests
    let min_diff = min_diff(init);
    // dbg!(init);
    println!("Min diff: {}", min_diff);

    // Performing all of our initial ruling out

    // If the fifth block is an account number or if the minimum difference between any two
    // consecutive initial requests is less than 5, the initial request cannot be a transfer.
    if accs.binary_search(&(5 * BLOCK_SIZE)).is_ok() || min_diff < 5 * BLOCK_SIZE {
        is_init.transfer = false;
    }

    // If the second block is an account number or the minimum difference is 5, then the initial
    // request cannot be a balance request
    if accs.binary_search(&(2 * BLOCK_SIZE)).is_ok() || min_diff == 5 * BLOCK_SIZE {
        is_init.balance = false;
    }

    // If the minimum difference is less than 4 or equal to 5, then the initial request cannot be
    // an invoice request.
    if min_diff < 4 * BLOCK_SIZE || min_diff == 5 * BLOCK_SIZE {
        is_init.invoice = false;
    }

    // Error handling
    if (is_init.transfer && is_init.balance)
        || (is_init.transfer && is_init.invoice)
        || (is_init.balance && is_init.invoice)
    {
        panic!("Initial request type not well-defined");
    } else if !is_init.transfer && !is_init.balance && !is_init.invoice {
        panic!("Invalid request sequence detected, could not derive initial request type");
    }

    // If it passed the error handling, then we know the type of our initial request header
    if is_init.transfer {
        ReqHeader::Transfer(0)
    } else if is_init.invoice {
        ReqHeader::Invoice(0)
    } else {
        ReqHeader::Balance(0)
    }
}

fn main() {
    // Getting our file
    let buf = create_buf();
    println!("Total lines: {}", buf.len() / BLOCK_SIZE);
    // Finding (some of) our account numbers
    let accs = find_accs(&buf);
    // let acc_lines = block_to_line(&accs);
    // Finding our initial request headers
    let init = find_block(&buf, 0);
    // let init_lines = block_to_line(&init);
    let init_type = find_reqs(&buf, &accs, &init);
    dbg!(init_type);
}
