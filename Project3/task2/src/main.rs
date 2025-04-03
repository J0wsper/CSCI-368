use std::{env, fs::File, io::Read};

const BLOCK_SIZE: usize = 16;

// Struct to carry information about which headers we have found
#[derive(Debug)]
struct Headers {
    balance: Option<usize>,
    invoice: Option<usize>,
    transfer: Option<usize>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            balance: None,
            invoice: None,
            transfer: None,
        }
    }
    pub fn found_balance(&mut self, loc: usize) {
        self.balance = Some(loc);
    }
    pub fn found_invoice(&mut self, loc: usize) {
        self.invoice = Some(loc)
    }
    pub fn found_transfer(&mut self, loc: usize) {
        self.transfer = Some(loc)
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
    headers: Headers,
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

fn solve(state: State) -> State {
    // Base case: we check if there are any contradictions
    if state.headers.num_found() == 3 {}
    todo!()
}

fn main() {
    // Getting our file
    let buf = create_buf();
    let state = State::new(&buf);
    solve(state);
}
