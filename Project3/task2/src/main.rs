use std::{env, fs::File, io::Read};

const BLOCK_SIZE: usize = 16;

#[derive(Debug)]
enum ReqType {
    Balance,
    Invoice,
    Transfer,
}

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
        self.balance = Some(&buf[loc..loc + BLOCK_SIZE]);
    }
    pub fn found_invoice(&mut self, buf: &'a [u8], loc: usize) {
        self.invoice = Some(&buf[loc..loc + BLOCK_SIZE]);
    }
    pub fn found_transfer(&mut self, buf: &'a [u8], loc: usize) {
        self.transfer = Some(&buf[loc..loc + BLOCK_SIZE]);
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
#[derive(Debug)]
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

// Produces chunks according to a request header assumption
fn make_chunks(buf: &[u8], idx: usize, req_type: ReqType) -> Vec<&[u8]> {
    let instances = find_block(buf, idx);
    let inc = match req_type {
        ReqType::Balance => 2,
        ReqType::Invoice => 4,
        ReqType::Transfer => 5,
    };
    let mut chunks = Vec::new();
    for instance in instances.iter() {
        let chunk = &buf[instance * BLOCK_SIZE..(instance + inc) * BLOCK_SIZE];
        chunks.push(chunk);
    }
    chunks
}

// We assume that our state.headers are all valid for a single iteration of the solve algorithm.
// From there, we can see if there is a valid way to parse the chunks. If there is, then we return
// that valid parsing. If there is not, we kill the process?
fn solve<'a>(state: &'a mut State<'a>) -> bool {
    // Base case: we check if there are any contradictions
    if state.headers.num_found() == 3 {
        // For each chunk in our collection of chunks
        for chunk in state.chunks.iter() {
            // While we have not verified every block in a given chunk
            let mut block = 0;
            while block < chunk.len() / BLOCK_SIZE {
                // Getting our different header types and their instances
                let balances = match state.headers.balance {
                    Some(instances) => instances,
                    None => panic!("Could not find balance instances in solved case"),
                };
                let transfers = match state.headers.transfer {
                    Some(instances) => instances,
                    None => panic!("Could not find transfer instances in solved case"),
                };
                let invoices = match state.headers.invoice {
                    Some(instances) => instances,
                    None => panic!("Could not find invoice instances in solved case"),
                };
                // We want to break up our chunks according to our headers and see if there is a
                // proper division.
                let header = &chunk[block * BLOCK_SIZE..(block + 1) * BLOCK_SIZE];
                // For each of our header request types, we increment the block we're looking at by
                // that request type's length
                if balances == header {
                    block += 2;
                } else if transfers == header {
                    block += 5
                } else if invoices == header {
                    block += 4;
                }
                // This is the invalid case so we say that our given header assumption are false
                // under the given partitioning scheme
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
        // While we have not assigned each of our header types
        let mut block = 0;
        while block < state.buf.len() / BLOCK_SIZE {
            // Try to assign headers to our various blocks
            let header = &state.buf[block * BLOCK_SIZE..(block + 1) * BLOCK_SIZE];
            // If this header is assigned, go to our next block and update the block we are
            // attempting to assign.
            if Some(header) == state.headers.balance {
                block += 2;
                continue;
            } else if Some(header) == state.headers.invoice {
                block += 4;
                continue;
            } else if Some(header) == state.headers.transfer {
                block += 5;
                continue;
            }
            // Otherwise, we try to assign our header and break up the trace wherever we see that
            // request into chunks
            if state.headers.balance.is_none() {
                state.headers.found_balance(state.buf, block);
                block += 2;
            } else if state.headers.transfer.is_none() {
                state.headers.found_transfer(state.buf, block);
                block += 5;
            } else if state.headers.invoice.is_none() {
                state.headers.found_invoice(state.buf, block);
                block += 4;
            }
        }
        // Once we've done this, we partition our trace into chunks and return the solved value
        block = 0;
        let mut done = Vec::new();
        // While the blocks we've assigned are not comprehensive
        while block < state.buf.len() / BLOCK_SIZE {
            // If we have done a particular block already in a previous iteration, we skip it.
            if done.binary_search(&block).is_ok() {
                println!("Done");
                continue;
            }
            // We take our header and all instances of that header, adding these to the indices we
            // have chunked already and sorting them.
            let header = &state.buf[block * BLOCK_SIZE..(block + 1) * BLOCK_SIZE];
            let instances = find_block(state.buf, block * BLOCK_SIZE);
            done = [done, instances].concat();
            done.sort();

            // Because we have a new request header, we perform the chunking for that type.
            let new_chunks;
            if Some(header) == state.headers.balance {
                new_chunks = make_chunks(state.buf, block, ReqType::Balance);
                block += 2;
            } else if Some(header) == state.headers.invoice {
                new_chunks = make_chunks(state.buf, block, ReqType::Invoice);
                block += 4;
            } else {
                new_chunks = make_chunks(state.buf, block, ReqType::Transfer);
                block += 5;
            }
            state.chunks = [state.chunks.clone(), new_chunks].concat();
            state.chunks.sort();
        }
        solve(state)
    }
}

fn main() {
    // Getting our file
    let buf = create_buf();
    let mut state = State::new(&buf);
    solve(&mut state);
}
