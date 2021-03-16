use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let h = scan.token::<usize>().unwrap() * 2;
    let mut down = Vec::new();
    let mut up = Vec::new();
    for i in 0..n {
        let x = scan.token::<usize>().unwrap();
        if i % 2 == 0 {
            down.push(x*2);
        } else {
            up.push(x*2);
        }
    }
    down.sort();
    up.sort();
    let mut min = std::usize::MAX;
    let mut min_h = std::usize::MAX;
    for i in (1..=h).step_by(2) {
        let d = down.len() - match down.binary_search(&i) {
            Ok(i) => unreachable!(),
            Err(i) => i,
        };
        let u = up.len() - match up.binary_search(&(h-i)) {
            Ok(i) => unreachable!(),
            Err(i) => i,
        };
        if d + u == min {
            min_h += 1;
        } else if d + u < min{
            min_h = 1;
            min = d + u;
        }
    }
    writeln!(out, "{} {}", min, min_h).unwrap();
}
