use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let score = scan.token::<usize>().unwrap();
    for i in 1..=20 {
        for ic in 0..=3 {
            for j in 1..=20 {
                for jc in 0..=3 {
                    for k in 1..=20 {
                        for kc in 0..=3 {
                            let s = i * ic + j * jc + k * kc;
                            if s == score {
                                let counts = [i, j, k];
                                let words = [ic, jc, kc];
                                let words = words.iter().map(|c| {
                                    match c {
                                        1 => "single",
                                        2 => "double",
                                        3 => "triple",
                                        _ => "zero"
                                    }
                                });
                               counts
                                   .into_iter()
                                   .filter(|c| **c > 0)
                                   .zip(words)
                                   .map(|(c, w)|
                                        writeln!(out, "{} {}", w, c)
                                        ).count();
                               return;
                            }
                        }
                    }
                }
            }
        }
    }
    writeln!(out, "impossible");
}
