use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{Read, Write},
};

// Block size. This is used everywhere
const BLOCK_SIZE: usize = 16;
const TRANSFER_SIZE: usize = 5 * BLOCK_SIZE;
const INVOICE_SIZE: usize = 4 * BLOCK_SIZE;
const BALANCE_SIZE: usize = 2 * BLOCK_SIZE;

#[derive(Debug, Clone, Copy)]
enum ReqType {
    Balance,
    Invoice,
    Transfer,
}

enum ClearType {
    Full,
    Partial(ReqType),
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
    fn is_solved(&self) -> bool {
        self.headers.transfer.is_some()
            && self.headers.invoice.is_some()
            && self.headers.balance.is_some()
    }
    pub fn print_headers(&self) {
        for chunk in self.chunks.iter() {
            let header = &chunk[0..BLOCK_SIZE];
            if header == self.headers.balance.unwrap() {
                println!("BALANCE");
            } else if header == self.headers.invoice.unwrap() {
                println!("INVOICE");
            } else {
                println!("TRANSFER");
            }
        }
    }
    pub fn print_pt2(&self) {
        if !self.is_solved() {
            println!(
                "State not properly found! Attempt to initialize first with the solve function"
            );
            return;
        }
        self.print_headers();
    }
    pub fn print_pt3(&self) {
        if !self.is_solved() {
            println!(
                "State not properly found! Attempt to initialize first with the solve function"
            );
            return;
        }
        self.print_headers();
        let our_acc = find_our_acc(&self.chunks, &self.headers, false);
        let our_acc_pos = find_header(self.buf, &our_acc);
        let transfer_header = &self.buf[our_acc_pos - (2 * BLOCK_SIZE)..our_acc_pos - BLOCK_SIZE];
        let src_acc = &self.buf[our_acc_pos - BLOCK_SIZE..our_acc_pos];
        let time = &self.buf[our_acc_pos + BLOCK_SIZE..our_acc_pos + (2 * BLOCK_SIZE)];
        let amount = &self.buf[our_acc_pos + (2 * BLOCK_SIZE)..our_acc_pos + (3 * BLOCK_SIZE)];
        let forged = [transfer_header, src_acc, &our_acc, time, amount].concat();
        let full_stream = [self.buf, &forged].concat();
        // Writing our forged request to task3.out
        let mut out = File::create("task3.out").expect("task3.out file name already taken");
        let _ = out.write_all(&full_stream);
    }
    pub fn print_pt4(&self) {
        if !self.is_solved() {
            println!(
                "State not properly found! Attempt to initialize first with the solve function"
            );
            return;
        }
        self.print_headers();
        // Finding our account
        let our_acc = find_our_acc(&self.chunks, &self.headers, false);
        let our_acc_pos = find_header(self.buf, &our_acc);
        // Getting the stream slices before and after the request we're modifying
        let after_slice = &self.buf[our_acc_pos + (3 * BLOCK_SIZE)..];
        let before_slice = &self.buf[..our_acc_pos - (2 * BLOCK_SIZE)];
        // Getting all of the appropriate data fields
        let transfer_header = self.headers.transfer.unwrap();
        let src_acc = &self.buf[our_acc_pos - BLOCK_SIZE..our_acc_pos];
        let time = &self.buf[our_acc_pos + BLOCK_SIZE..our_acc_pos + (2 * BLOCK_SIZE)];
        let original_amount =
            &self.buf[our_acc_pos + (2 * BLOCK_SIZE)..our_acc_pos + (3 * BLOCK_SIZE)];
        let mut new_amount = Vec::new();
        // Finding our new amount to transfer
        for chunk in self.chunks.iter() {
            let header = &chunk[0..BLOCK_SIZE];
            if header == self.headers.invoice.unwrap() {
                let cand_amount = &chunk[3 * BLOCK_SIZE..4 * BLOCK_SIZE];
                if cand_amount != original_amount {
                    new_amount = cand_amount.to_vec();
                    break;
                }
            }
        }
        // Forging a new request stream
        let forged = [transfer_header, src_acc, &our_acc, time, &new_amount].concat();
        let new_stream = [before_slice, &forged, after_slice].concat();
        // Writing our forged request to task3.out
        let mut out = File::create("task4.out").expect("task4.out file name already taken");
        let _ = out.write_all(&new_stream);
    }
    pub fn print_pt5(&self) {
        if !self.is_solved() {
            println!(
                "State not properly found! Attempt to initialize first with the solve function"
            );
            return;
        }
        self.print_headers();
        let transfer_header = self.headers.transfer.unwrap().to_vec();
        let our_acc = find_our_acc(&self.chunks, &self.headers, true);
        let our_acc_pos = find_header(self.buf, &our_acc);
        let after_slice = &self.buf[our_acc_pos + (2 * BLOCK_SIZE)..];
        let before_slice = &self.buf[..our_acc_pos - (2 * BLOCK_SIZE)];
        let req_account = &self.buf[our_acc_pos - BLOCK_SIZE..our_acc_pos];
        let amount = &self.buf[our_acc_pos + BLOCK_SIZE..our_acc_pos + (2 * BLOCK_SIZE)];
        let mut time = Vec::new();
        for chunk in self.chunks.iter() {
            let header = &chunk[0..BLOCK_SIZE];
            if header == self.headers.transfer.unwrap() {
                time = chunk[3 * BLOCK_SIZE..4 * BLOCK_SIZE].to_vec();
                break;
            }
        }
        let forged = [&transfer_header, req_account, &our_acc, &time, amount].concat();
        let new_stream = [before_slice, &forged, after_slice].concat();
        let mut out = File::create("task5.out").expect("task5.out file name already taken");
        let _ = out.write_all(&new_stream);
    }
}

// Helper function to create our file
fn create_buf() -> Vec<u8> {
    // Collects the command line arguments
    let args: Vec<String> = env::args().collect();
    // Gets the provided file path as the last command line argument
    let file_path = &args[args.len() - 1];
    // Creates our buffer and writes the contents of the file to it
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
        let acc = &buf[*req + BLOCK_SIZE..*req + 2 * BLOCK_SIZE];
        accs.insert(acc);
    }
    accs
}

// Finds our account block depending on whether or not it is part 5 or not
fn find_our_acc(chunks: &Vec<&[u8]>, headers: &Headers, is_part5: bool) -> Vec<u8> {
    let mut all_accs = Vec::new();
    for chunk in chunks.iter() {
        all_accs.push(&chunk[BLOCK_SIZE..2 * BLOCK_SIZE]);
        if chunk.len() > BALANCE_SIZE {
            all_accs.push(&chunk[2 * BLOCK_SIZE..3 * BLOCK_SIZE]);
        }
    }
    let mut reqs = Vec::new();
    for (i, chunk) in chunks.iter().enumerate() {
        let header = &chunk[0..BLOCK_SIZE];
        if (header == headers.transfer.unwrap() && !is_part5)
            || (header == headers.invoice.unwrap() && is_part5)
        {
            reqs.push(chunks[i]);
        }
    }
    let cand_accs: Vec<&[u8]> = reqs
        .iter()
        .map(|x| &x[2 * BLOCK_SIZE..3 * BLOCK_SIZE])
        .collect();
    // Finding our account
    let our_acc: Vec<&&[u8]> = all_accs
        .iter()
        .filter(|x| all_accs.iter().filter(|y| y == x).count() == 1 && cand_accs.contains(x))
        .collect();
    let our_acc = match our_acc.len() {
        0 => panic!("Could not find our account"),
        _ => *our_acc[0],
    };
    our_acc.to_owned()
}

// Splits a chunk at a specified index.
fn split_chunks(chunk: &[u8], req_type: ReqType) -> (&[u8], &[u8]) {
    let div = match req_type {
        ReqType::Balance => BALANCE_SIZE,
        ReqType::Invoice => INVOICE_SIZE,
        ReqType::Transfer => TRANSFER_SIZE,
    };
    let first_chunk = &chunk[0..div];
    let second_chunk = &chunk[div..];
    (first_chunk, second_chunk)
}

// Composes a new state with split chunks at the specified index.
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
    let new_accs = find_accs(state.buf, &chunk[0..BLOCK_SIZE]);
    state.accs.extend(&new_accs);
    state
}

// Validates that none of our request headers are also account numbers.
fn validate_chunks(chunks: &Vec<&[u8]>, accs: &HashSet<&[u8]>) -> bool {
    for chunk in chunks {
        if accs.contains(chunk) {
            return false;
        }
    }
    true
}

// Performing an initial screen on whether a chunk can be properly split into the specified request
// type.
fn initial_screen(
    chunk: &[u8],
    accs: &HashSet<&[u8]>,
    headers: &Headers,
    req_type: ReqType,
) -> bool {
    let chunk_len = chunk.len();
    let (div, header_holder) = match req_type {
        ReqType::Balance => (BALANCE_SIZE, headers.balance),
        ReqType::Invoice => (INVOICE_SIZE, headers.invoice),
        ReqType::Transfer => (TRANSFER_SIZE, headers.transfer),
    };
    chunk_len > div && !accs.contains(&chunk[div..div + BLOCK_SIZE]) && header_holder.is_none()
}

// Clearing a chunk as valid
fn is_clear(chunk: &[u8], headers: &Headers) -> Option<ClearType> {
    let header = &chunk[0..BLOCK_SIZE];
    let len = chunk.len();
    if len % BLOCK_SIZE != 0 {
        panic!("Invalid buffer length when attempting to parse header");
    }
    if (Some(header) == headers.balance && len == BALANCE_SIZE)
        || (Some(header) == headers.invoice && len == INVOICE_SIZE)
        || (Some(header) == headers.transfer && len == TRANSFER_SIZE)
    {
        Some(ClearType::Full)
    } else if Some(header) == headers.balance && len > BALANCE_SIZE {
        Some(ClearType::Partial(ReqType::Balance))
    } else if Some(header) == headers.invoice && len > INVOICE_SIZE {
        Some(ClearType::Partial(ReqType::Invoice))
    } else if Some(header) == headers.transfer && len > TRANSFER_SIZE {
        Some(ClearType::Partial(ReqType::Transfer))
    } else {
        None
    }
}

// We assume that our state.headers are all valid for a single iteration of the solve algorithm.
// From there, we can see if there is a valid way to parse the chunks. If there is, then we return
// that valid parsing. If there is not, we kill the process?
fn solve(state: State) -> Option<State> {
    // Step 1: find an unsolved chunk
    for (i, chunk) in state.chunks.iter().enumerate() {
        let header = &chunk[0..BLOCK_SIZE];
        if let Some(clear_type) = is_clear(chunk, &state.headers) {
            match clear_type {
                // If we have a fully-cleared chunk, continue
                ClearType::Full => continue,
                // If we have a partially-cleared chunk, we split it
                ClearType::Partial(req_type) => {
                    let new_state = compose_state(state.clone(), chunk, i, req_type);
                    let ret = solve(new_state);
                    if let Some(cand) = ret {
                        if validate_chunks(&cand.chunks, &cand.accs) {
                            return Some(cand);
                        }
                    }
                    return None;
                }
            }
        }
        for j in 0..3 {
            let req_type = match j {
                0 => ReqType::Transfer,
                1 => ReqType::Invoice,
                2 => ReqType::Balance,
                _ => panic!("Out of bound req type attempt"),
            };
            // If our assignment does not pass the initial screening, then we simply continue
            // and try a different assignment
            if !initial_screen(chunk, &state.accs, &state.headers, req_type) {
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
    }
    Some(state)
}

fn main() {
    // Getting our file
    let buf = create_buf();
    let state = State::new(&buf);
    let ret = solve(state);
    match ret {
        Some(i) => i.print_pt4(),
        None => println!("No valid assignment found"),
    }
}
