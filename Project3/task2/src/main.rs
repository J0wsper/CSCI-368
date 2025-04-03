use std::{env, fs::File, io::Read};

const BLOCK_SIZE: usize = 16;

// Struct to carry information about which headers we have found
#[derive(Debug)]
struct Headers<'a> {
    balance: Option<&'a [u8]>,
    invoice: Option<&'a [u8]>,
    transfer: Option<&'a [u8]>,
}

impl<'a> Headers<'a> {
    pub fn new() -> Self {
        Self {
            balance: None,
            invoice: None,
            transfer: None,
        }
    }
    pub fn found_balance(&mut self, buf: &'a [u8], loc: usize) {
        let instances = find_block(buf, loc);
        self.balance = Some(instances);
    }
    pub fn found_invoice(&mut self, buf: &'a [u8], loc: usize) {
        let instances = find_block(buf, loc);
        self.invoice = Some(instances);
    }
    pub fn found_transfer(&mut self, buf: &'a [u8], loc: usize) {
        let instances = find_block(buf, loc);
        self.transfer = Some(instances);
    }
    pub fn num_found(&self) -> u8 {
        let mut found = 0;
        if self.balance.is_some() {
            found += 1;
        }
        if self.transfer.is_some() {
            found += 1;
        }
        if self.invoice.is_some() {
            found += 1;
        }
        found
    }
}

// Holds the state that we're in when we're recursing
struct State<'a> {
    buf: &'a [u8],
    chunks: Vec<&'a [u8]>,
    headers: Headers<'a>,
}

impl<'a> State<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            chunks: vec![buf],
            headers: Headers::new(),
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

// We assume that our state.headers are all valid for a single iteration of the solve algorithm.
// From there, we can see if there is a valid way to parse the chunks. If there is, then we return
// that valid parsing. If there is not, we kill the process?
fn solve(state: &mut State) -> bool {
    // Base case: we check if there are any contradictions
    if state.headers.num_found() == 3 {
        // For each chunk in our collection of chunks
        for chunk in state.chunks.iter() {
            // While we have not verified every block in a given chunk
            let mut block = 0;
            while block < chunk.len() / BLOCK_SIZE {
                // Getting our different header types and their instances
                let balances = match state.headers.balance {
                    Some(ref instances) => instances,
                    None => panic!("Could nto find balance instances"),
                };
                let transfers = match state.headers.transfer {
                    Some(ref instances) => instances,
                    None => panic!("Could not find transfer instances"),
                };
                let invoices = match state.headers.invoice {
                    Some(ref instances) => instances,
                    None => panic!("Could not find invoice instances"),
                };
                // We want to break up our chunks according to our headers and see if there is a
                // proper division.
                if balances.binary_search(&block).is_ok() {
                    block += 2;
                } else if transfers.binary_search(&block).is_ok() {
                    block += 5
                } else if invoices.binary_search(&block).is_ok() {
                    block += 4;
                }
                // This is the invalid case
                else {
                    return false;
                }
            }
        }
        // If we manage to partition all of our chunks according to our header assumptions, then we
        // will return that this is an assignment that might work.
        true
    }
    // Otherwise, we try different assignment and split our trace
    else {
        // We're trying to find some way to split our unsolved chunks that works.
        // Any chunk that begins with a block not contained in one of our state.header vectors is
        // unsolved.
        // If transfers are not assigned, we try that first.
        // If they are but invoices are not assigned, we try that next.
        // If they are but balances are not assigned, we try that last.
        true
    }
}

fn main() {
    // Getting our file
    let buf = create_buf();
    let mut state = State::new(&buf);
    solve(&mut state);
}
