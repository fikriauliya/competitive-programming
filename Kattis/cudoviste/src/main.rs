use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let r = scan.token::<usize>().unwrap();
    let c = scan.token::<usize>().unwrap();
    let mut map = vec![vec![' '; c]; r];
    for i in 0..r {
        let line = scan.line().unwrap();
        let line: Vec<char> = line.chars().collect();
        for j in 0..c {
            map[i][j] = line[j];
        }
    }
    for moved in 0..=4 {
        let mut res = 0;
        for i in 0..r-1 {
            for j in 0..c-1 {
                let mut cars = 0;
                let mut free = 0;
                for l in 0..2 {
                    for m in 0..2 {
                        match map[i+l][j+m] {
                            'X' => cars += 1,
                            '.' => free += 1,
                            _ => (),
                        };
                    }
                }
                if free + cars == 4 && cars == moved {
                    res += 1;
                }
            }
        }
        writeln!(out, "{}", res);
    }
}
