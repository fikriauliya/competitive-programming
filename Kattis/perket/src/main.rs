use std::{i64::MAX, io}; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut ss = Vec::new();
    let mut bs = Vec::new();
    for _ in 0..n {
        let s = scan.token::<i64>().unwrap();
        let b = scan.token::<i64>().unwrap();
        ss.push(s);
        bs.push(b);
    }
    let mut min_diff = std::i64::MAX;
    for i in 1..2usize.pow(n as u32) {
        let tot_s: i64 = ss.iter().enumerate().filter(|(j, _)| i & (1 << *j) > 0)
            .map(|v| v.1)
            .fold(1, |acc, cur| acc * cur);
        let tot_b: i64 = bs.iter().enumerate().filter(|(j, _)| i & (1 << *j) > 0)
            .map(|v| v.1).sum();
        min_diff = min_diff.min((tot_b - tot_s).abs());
    }
    writeln!(out, "{}", min_diff);
}
