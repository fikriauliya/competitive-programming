use std::io; use std::str;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn flip(hbits: usize, n: usize) -> usize {
}

fn can(hbits: usize, n: usize) -> bool {
    let mask = (1 << n) - 1;
    if hbits == (1 << (n*n)) - 1 { return true; }

    let mut cur = hbits;
    for i in 0..n {
        let part = cur & mask;
        let neg = part ^ mask;
        let lg = (neg as f32).log2();
        if lg == lg.floor() {
            let hbits = hbits | (neg << (i * n));

            return can(hbits, n);
        }
        cur = cur >> n;
    }

    let mut cur = flip(hbits, n);
    for i in 0..n {
        let part = cur & mask;
        let neg = part ^ mask;
        let lg = (neg as f32).log2();
        if lg == lg.floor() {
            let hbits = hbits | (neg << (i * n));

            return can(hbits, n);
        }
        cur = cur >> n;
    }
    return false;
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let n = scan.token::<usize>().unwrap();
        let mut a = vec![vec![0isize; n];n];
        let mut hknowns = 0;
        let mut vknowns = 0;
        for i in 0..n {
            for j in 0..n {
                a[i][j] = scan.token::<isize>().unwrap();
                if a[i][j] != -1 {
                    hknowns |= 1 << (i * n + j);
                    vknowns |= 1 << (j * n + i);
                }
            }
        }
        let mut b = vec![vec![0; n];n];
        for i in 0..n {
            for j in 0..n {
                b[i][j] = scan.token::<usize>().unwrap();
            }
        }
        let mut rows = vec![0isize; n];
        let mut cols = vec![0isize; n];
        for i in 0..n {
            rows[i] = scan.token::<isize>().unwrap();
        }
        for i in 0..n {
            cols[i] = scan.token::<isize>().unwrap();
        }
        let mut res = std::usize::MAX;
        for k in 0..(2usize.pow((n*n) as u32)) {
            let hknowns = hknowns | k;
            let mut cost = 0;
            for i in 0..n {
                for j in 0..n {
                    if (1 << (i*n + j)) & k > 0 {
                        cost += b[i][j]
                    }
                }
            }
            if cost < res && can(hknowns, vknowns, n) {
                res = cost;
            }
        }
    }
}
