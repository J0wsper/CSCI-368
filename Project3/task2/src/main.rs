use std::{cmp::min, env, fs::File, io::Read};

// Employing optimal substructure to get a dynamic programming solution
fn find_suffix(i: usize, j: usize, buf: &Vec<u8>, memo: &mut Vec<Vec<usize>>) -> usize {
    if j == buf.len() {
        return 0;
    }
    if memo[i][j] != usize::MAX {
        return memo[i][j];
    }
    if buf[i] == buf[j] && j > i {
        memo[i][j] = min(1 + find_suffix(i + 1, j + 1, buf, memo), j - i - 1);
    } else {
        memo[i][j] = 0;
    }
    memo[i][j]
}

// Using the optimal substructure to find our longest repeated substring
fn longest_substring(buf: &Vec<u8>) -> Vec<(usize, usize)> {
    // Getting our buffer length and making an n x n memoization table
    let len = buf.len();
    let mut memo: Vec<Vec<usize>> = vec![vec![usize::MAX; len]; len];

    // Filling out our table
    for (i, _) in buf.iter().enumerate() {
        for (j, _) in buf.iter().enumerate() {
            if j <= i {
                continue;
            }
            find_suffix(i, j, buf, &mut memo);
        }
    }

    let mut ans_len = 0;

    // Finding our optimal answer
    for (i, _) in buf.iter().enumerate() {
        for (j, _) in buf.iter().enumerate() {
            if j <= i {
                continue;
            }
            if memo[i][j] > ans_len && memo[i][j] != usize::MAX {
                ans_len = memo[i][j];
            }
        }
    }

    let mut longest_substrings = Vec::new();

    // Finding all occurances of our optimal answer
    for (i, _) in buf.iter().enumerate() {
        for (j, _) in buf.iter().enumerate() {
            if j <= i {
                continue;
            }
            if memo[i][j] == ans_len {
                let new_ans_1 = (i, i + ans_len - 1);
                let new_ans_2 = (j, j + ans_len - 1);
                longest_substrings.push(new_ans_1);
                longest_substrings.push(new_ans_2);
            }
        }
    }
    longest_substrings
}

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
    let longest_substring = longest_substring(&buf);
    dbg!(longest_substring);
}
