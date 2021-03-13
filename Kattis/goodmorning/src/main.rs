use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn generate(accum: usize, x: usize, y: usize, len: usize) -> Vec<usize> {
    if len >= 3 { return vec![accum] }

    let new_accum = accum * 10 + match (y, x) {
        (0, 0) => 1,
        (0, 1) => 2,
        (0, 2) => 3,
        (1, 0) => 4,
        (1, 1) => 5,
        (1, 2) => 6,
        (2, 0) => 7,
        (2, 1) => 8,
        (2, 2) => 9,
        (3, 1) => 0,
        (_, _) => return vec![accum]
    };
    let mut res = Vec::new();
    res.append(&mut generate(new_accum, x, y, len + 1));
    res.append(&mut generate(new_accum, x + 1, y, len + 1));
    res.append(&mut generate(new_accum, x, y + 1, len + 1));
    res.append(&mut generate(accum, x + 1, y, len));
    res.append(&mut generate(accum, x, y + 1, len));
    return res;
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let nums: HashSet<usize> = generate(0, 0, 0, 0).into_iter().collect();
    let mut nums: Vec<usize> = nums.into_iter().collect();
    nums.sort();
    for _ in 0..n {
        let k = scan.token::<usize>().unwrap();
        let res = match nums.binary_search(&k) {
            Ok(res) => { nums[res] }
            Err(i) => match (nums.get(i-1), nums.get(i)) {
                (Some(prev), Some(next)) => {
                    if (k as isize - *prev as isize).abs() > 
                        (k as isize - *next as isize).abs() {
                        *next
                    } else {
                        *prev
                    }
                },
                (None, Some(next)) => *next,
                (Some(prev), None) => *prev,
                (None, None) => unreachable!()
            }
        };
        writeln!(out, "{}", res);
    }
}
