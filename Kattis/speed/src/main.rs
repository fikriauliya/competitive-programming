use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let t = scan.token::<f64>().unwrap();
    let mut m = Vec::with_capacity(n);
    for _ in 0..n {
        let d = scan.token::<f64>().unwrap();
        let v = scan.token::<f64>().unwrap();
        m.push((d, v));
    }

    let mut hi = std::f32::MAX as f64;
    let mut min = std::f64::MAX;
    for mm in m.iter() { if mm.1 < min { min = mm.1 } }
    let mut lo = min * -1f64;
    let mut c;
    loop {
        c = (hi + lo) / 2f64;
        if hi == lo { break; }
        let tt = m.iter().fold(0f64, |accum, x| accum + x.0 / (x.1 + c));
        let eps = (hi - lo).abs();
        if eps <= 1e-6 { break; }
        if tt > t {
            lo = c;
        } else {
            hi = c;
        }
    }
    writeln!(out, "{}", c).unwrap();
}
