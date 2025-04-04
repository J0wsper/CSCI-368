use std::{collections::HashSet, env, fs::File, io::Read};

// Block size. This is used everywhere
const BLOCK_SIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
enum ReqType {
    Balance,
    Invoice,
    Transfer,
}

#[derive(Debug, Clone)]
struct Headers<'a> {
    balance: Option<&'a [u8]>,
    invoice: Option<&'a [u8]>,
    transfer: Option<&'a [u8]>,
}

// Struct to carry information about which headers we have found

// Holds the state that we're in when we're recursing
#[derive(Debug, Clone)]
struct State<'a> {
    buf: &'a [u8],
    chunks: Vec<&'a [u8]>,
    accs: HashSet<&'a [u8]>,
    headers: Headers<'a>,
}

impl<'a> State<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            accs: find_accs(buf, &buf[0..BLOCK_SIZE]),
            chunks: vec![buf],
            headers: Headers {
                balance: None,
                invoice: None,
                transfer: None,
            },
        }
    }
    pub fn print(&self) {
        if self.headers.transfer.is_none()
            || self.headers.invoice.is_none()
            || self.headers.balance.is_none()
        {
            println!(
                "State not properly found! Attempt to initialize first with the solve function"
            );
        } else {
            for i in 0..3 {
                match i {
                    0 => {
                        let idx = find_header(self.buf, self.headers.balance.unwrap());
                        let occurences = find_block(self.buf, idx).len();
                        println!("Balance occurences: {}", occurences);
                    }
                    1 => {
                        let idx = find_header(self.buf, self.headers.invoice.unwrap());
                        let occurences = find_block(self.buf, idx).len();
                        println!("Invoice occurences: {}", occurences);
                    }
                    2 => {
                        let idx = find_header(self.buf, self.headers.transfer.unwrap());
                        let occurences = find_block(self.buf, idx).len();
                        println!("Transfer occurences: {}", occurences);
                    }
                    _ => {
                        panic!("Out of bound req type when printing");
                    }
                }
            }
        }
    }
}

// Helper function to create our file
fn create_buf() -> Vec<u8> {
    // Collects the command line arguments
    let args: Vec<String> = env::args().collect();
    // Gets the provided file path as the last command line argument
    let file_path = &args[args.len() - 1];
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
    let req = &buf[start..start + BLOCK_SIZE];
    let mut found = Vec::new();
    let mut block = 0;
    while block < buf.len() / BLOCK_SIZE {
        let block_index = block * BLOCK_SIZE;
        let line = &buf[block_index..block_index + BLOCK_SIZE];
        if *line == *req {
            found.push(block_index);
        }
        block += 1;
    }
    found
}

// Finds the instance of a given header
fn find_header(buf: &[u8], header: &[u8]) -> usize {
    let mut block = 0;
    while block < buf.len() / BLOCK_SIZE {
        let block_index = block * BLOCK_SIZE;
        let line = &buf[block_index..block_index + BLOCK_SIZE];
        if line == header {
            return block_index;
        }
        block += 1;
    }
    panic!("Could not find header in trace");
}

// Utility function to find account numbers under the assumption that loc is a request header
fn find_accs<'a>(buf: &'a [u8], header: &'a [u8]) -> HashSet<&'a [u8]> {
    let mut accs = HashSet::new();
    let loc = find_header(buf, header);
    let instances = find_block(buf, loc);
    for req in instances.iter() {
        let acc = &buf[*req..*req + BLOCK_SIZE];
        accs.insert(acc);
    }
    accs
}

fn validate_chunks(chunks: &Vec<&[u8]>, accs: &HashSet<&[u8]>) -> bool {
    for chunk in chunks {
        if accs.contains(chunk) {
            return false;
        }
    }
    true
}

fn split_chunks(chunk: &[u8], req_type: ReqType) -> (&[u8], &[u8]) {
    let div = match req_type {
        ReqType::Balance => 2 * BLOCK_SIZE,
        ReqType::Invoice => 4 * BLOCK_SIZE,
        ReqType::Transfer => 5 * BLOCK_SIZE,
    };
    let first_chunk = &chunk[0..div];
    let second_chunk = &chunk[div..];
    (first_chunk, second_chunk)
}

fn compose_state<'a>(
    mut state: State<'a>,
    chunk: &'a [u8],
    idx: usize,
    req_type: ReqType,
) -> State<'a> {
    let new_chunks = split_chunks(chunk, req_type);
    state.chunks.remove(idx);
    state.chunks.push(new_chunks.0);
    state.chunks.push(new_chunks.1);
    state.chunks.sort();
    match req_type {
        ReqType::Balance => {
            let new_accs = find_accs(state.buf, &chunk[BLOCK_SIZE..2 * BLOCK_SIZE]);
            state.accs.extend(&new_accs);
        }
        _ => {
            let new_accs1 = find_accs(state.buf, &chunk[BLOCK_SIZE..2 * BLOCK_SIZE]);
            let new_accs2 = find_accs(state.buf, &chunk[2 * BLOCK_SIZE..3 * BLOCK_SIZE]);
            state.accs.extend(&new_accs1);
            state.accs.extend(&new_accs2);
        }
    }
    state
}

fn initial_screen(
    header: &[u8],
    chunk: &[u8],
    accs: &HashSet<&[u8]>,
    taken_headers: &Headers,
    req_type: ReqType,
) -> bool {
    let chunk_len = chunk.len();
    let div = match req_type {
        ReqType::Balance => 2 * BLOCK_SIZE,
        ReqType::Invoice => 4 * BLOCK_SIZE,
        ReqType::Transfer => 5 * BLOCK_SIZE,
    };
    let header_holder = match req_type {
        ReqType::Balance => taken_headers.balance,
        ReqType::Invoice => taken_headers.invoice,
        ReqType::Transfer => taken_headers.transfer,
    };
    (chunk_len > div && !accs.contains(&chunk[div..div + BLOCK_SIZE]) && header_holder.is_none())
        || header_holder == Some(header)
}

fn is_req_header(buf: &[u8], headers: &Headers) -> bool {
    if buf.len() != BLOCK_SIZE {
        panic!("Invalid buffer length when attempting to parse header");
    }
    Some(buf) == headers.balance || Some(buf) == headers.invoice || Some(buf) == headers.transfer
}

// We assume that our state.headers are all valid for a single iteration of the solve algorithm.
// From there, we can see if there is a valid way to parse the chunks. If there is, then we return
// that valid parsing. If there is not, we kill the process?
fn solve(state: State) -> Option<State> {
    // Step 1: find an unsolved chunk
    for (i, chunk) in state.chunks.iter().enumerate() {
        let len = chunk.len();
        let header = &chunk[0..BLOCK_SIZE];
        // If our chunk length is invalid
        if is_req_header(header, &state.headers) {
            continue;
        }
        if len != 2 * BLOCK_SIZE && len != 4 * BLOCK_SIZE && len != 5 * BLOCK_SIZE {
            for j in 0..3 {
                let req_type = match j {
                    0 => ReqType::Transfer,
                    1 => ReqType::Invoice,
                    2 => ReqType::Balance,
                    _ => panic!("Out of bound req type attempt"),
                };
                // If our assignment does not pass the initial screening, then we simply continue
                // and try a different assignment
                if !initial_screen(header, chunk, &state.accs, &state.headers, req_type) {
                    continue;
                }
                // Otherwise, we try out that configuration and create a new state object
                let new_state = state.clone();
                let mut new_state = compose_state(new_state, chunk, i, req_type);
                match j {
                    0 => {
                        new_state.headers.transfer = Some(header);
                    }
                    1 => {
                        new_state.headers.invoice = Some(header);
                    }
                    2 => {
                        new_state.headers.balance = Some(header);
                    }
                    _ => panic!("Out of bound req attempt"),
                };
                let ret = solve(new_state);
                if let Some(cand) = ret {
                    if validate_chunks(&cand.chunks, &cand.accs) {
                        return Some(cand);
                    }
                }
            }
            return None;
        } else {
            continue;
        }
    }
    Some(state)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // Getting our file
    let buf = create_buf();
    let state = State::new(&buf);
    let ret = solve(state);
    match ret {
        Some(res) => res.print(),
        None => println!("Could not find a valid assignment"),
    }
}
