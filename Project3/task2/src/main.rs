use std::{collections::BTreeMap, env, fs::File, io::Read};

// Block size. This is used everywhere
const BLOCK_SIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
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
    pub fn print(&self) {
        if self.headers.num_found() != 3 {
            println!("Valid assignment has not been found yet!");
        } else {
            for i in 0..3 {
                match i {
                    0 => {
                        let idx = find_header(self.buf, self.headers.balance.unwrap());
                        let locs = find_block(self.buf, idx);
                        println!("Number of balance requests: {}", locs.len());
                    }
                    1 => {
                        let idx = find_header(self.buf, self.headers.invoice.unwrap());
                        let locs = find_block(self.buf, idx);
                        println!("Number of invoice requests: {}", locs.len());
                    }
                    2 => {
                        let idx = find_header(self.buf, self.headers.transfer.unwrap());
                        let locs = find_block(self.buf, idx);
                        println!("Number of transfer requests: {}", locs.len());
                    }
                    _ => panic!("Incorrect number of headers filled in for state"),
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
        if line == req {
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

// Finds the minimum difference between two instances of a given request header
fn min_diff(buf: &[u8], loc: usize) -> usize {
    let locs = find_block(buf, loc);
    if locs.is_empty() {
        panic!("Could not find min diff of specified block");
    }
    if locs.len() == 1 {
        return diff_from_end(buf, loc);
    }
    locs.windows(2)
        .map(|x| x[0].abs_diff(x[1]))
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
}

// Finds the difference from the end of the given header
fn diff_from_end(buf: &[u8], loc: usize) -> usize {
    let locs = find_block(buf, loc);
    let last_loc = locs[locs.len() - 1];
    buf.len() - last_loc
}

// Produces chunks according to a request header assumption
fn make_chunks(buf: &[u8], idx: usize, req_type: ReqType) -> Vec<&[u8]> {
    let instances = find_block(buf, idx);
    let inc = match req_type {
        ReqType::Balance => 2 * BLOCK_SIZE,
        ReqType::Invoice => 4 * BLOCK_SIZE,
        ReqType::Transfer => 5 * BLOCK_SIZE,
    };
    let mut chunks = Vec::new();
    for inst_start in instances.iter() {
        let inst_end = inst_start + inc;
        let chunk = &buf[*inst_start..inst_end];
        chunks.push(chunk);
    }
    chunks
}

// We assume that our state.headers are all valid for a single iteration of the solve algorithm.
// From there, we can see if there is a valid way to parse the chunks. If there is, then we return
// that valid parsing. If there is not, we kill the process?
fn solve(state: &mut State) -> bool {
    // Base case: we check if there are any contradictions
    let buf_len = state.buf.len();
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
        while block < buf_len / BLOCK_SIZE && state.headers.num_found() != 3 {
            let block_index = block * BLOCK_SIZE;
            // Try to assign headers to our various blocks
            let header = &state.buf[block_index..block_index + BLOCK_SIZE];
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
            let min_diff = min_diff(state.buf, block_index);
            let diff_from_end = diff_from_end(state.buf, block_index);
            if state.headers.transfer.is_none()
                && diff_from_end >= 5 * BLOCK_SIZE
                && min_diff >= 5 * BLOCK_SIZE
            {
                state.headers.found_transfer(state.buf, block_index);
                block += 5;
            } else if state.headers.invoice.is_none()
                && diff_from_end >= 4 * BLOCK_SIZE
                && min_diff >= 4 * BLOCK_SIZE
            {
                state.headers.found_invoice(state.buf, block_index);
                block += 4;
            } else if state.headers.balance.is_none() && diff_from_end >= 2 * BLOCK_SIZE {
                state.headers.found_balance(state.buf, block_index);
                block += 2;
            } else {
                panic!("No valid assignment exists");
            }
        }
        // Once we've done this, we partition our trace into chunks and return the solved value
        block = 0;
        let mut done = BTreeMap::new();
        // While the blocks we've assigned are not comprehensive
        while block < buf_len / BLOCK_SIZE {
            let block_index = block * BLOCK_SIZE;
            // If we have done a particular block already in a previous iteration, we skip it.
            if done.contains_key(&block) {
                let inc = match done.get(&block).unwrap() {
                    ReqType::Balance => 2,
                    ReqType::Invoice => 4,
                    ReqType::Transfer => 5,
                };
                block += inc;
                continue;
            }
            // We take our header and all instances of that header, adding these to the indices we
            // have chunked already and sorting them.
            let header = &state.buf[block_index..block_index + BLOCK_SIZE];
            // TODO: This call can introduce out-of-bounds errors
            let instances = find_block(state.buf, block_index);
            let req_type;

            // Because we have a new request header, we perform the chunking for that type.
            // We need to verify that our chunk doesn't cause an out-of-bound error.
            let new_chunks;
            if Some(header) == state.headers.balance {
                req_type = ReqType::Balance;
                new_chunks = make_chunks(state.buf, block_index, req_type);
                block += 2;
            } else if Some(header) == state.headers.invoice {
                req_type = ReqType::Invoice;
                new_chunks = make_chunks(state.buf, block_index, req_type);
                block += 4;
            } else {
                req_type = ReqType::Transfer;
                new_chunks = make_chunks(state.buf, block_index, req_type);
                block += 5;
            }
            // Adding the new values we've just seen to our map
            for val in instances.iter() {
                done.insert(*val, req_type);
            }
            state.chunks = [state.chunks.clone(), new_chunks].concat();
        }
        state.chunks.sort();
        solve(state)
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // Getting our file
    let buf = create_buf();
    let mut state = State::new(&buf);
    solve(&mut state);
    state.print();
}
