use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut h = Vec::new();
    for _ in 0..6 {
        h.push(scan.token::<usize>().unwrap())
    }
    let t1 = scan.token::<usize>().unwrap();
    let t2 = scan.token::<usize>().unwrap();

    for i in 0..h.len() {
        for j in i..h.len() {
            for k in j..h.len() {
                if h[i] + h[j] + h[k] == t1 {
                    let mut s1 = vec![h[i], h[j], h[k]];
                    let mut s2 = Vec::new();
                    for l in 0..h.len() {
                        if ![i, j, k].contains(&l) {
                            s2.push(h[l]);
                        }
                    }
                    if s2.iter().sum::<usize>() == t2 {
                        s1.sort_by(|a, b| b.cmp(&a));
                        s2.sort_by(|a, b| b.cmp(&a));
                        let s1: Vec<String> = s1.iter().map(|s| s.to_string()).collect();
                        let s2: Vec<String> = s2.iter().map(|s| s.to_string()).collect();
                        writeln!(out, "{} {}", s1.join(" "), s2.join(" "));
                        return;
                    }
                }
            }
        }
    }
}
