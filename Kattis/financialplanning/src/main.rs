use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(invs: &Vec<(isize, isize)>, m: isize, left: isize, right: isize) -> Option<isize> {
    let mid = (right + left) / 2;
    let tot = invs.iter().fold(0, |accum, x| accum + (x.0 * mid - x.1).max(0));
    if left > right { return None; }

    if tot == m { Some(mid) }
    else if tot > m { 
        match rec(invs, m, left, mid - 1) {
            Some(res) => Some(res),
            None => Some(mid),
        }
    } else {
        rec(invs, m, mid + 1, right)
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let m = scan.token::<isize>().unwrap();
    let mut invs = Vec::with_capacity(n);
    for _ in 0..n {
        let p = scan.token::<isize>().unwrap();
        let c = scan.token::<isize>().unwrap();
        invs.push((p, c));
    }
    let (tot_p, tot_c) = invs.iter().fold((0, 0), |accum, x| { (accum.0 + x.0, accum.1 + x.1)});
    let right = ((m + tot_c) + (tot_p- 1)) / tot_p;
    let left = 0;
    let res = rec(&invs, m, left, right);
    writeln!(out, "{}", res.unwrap()).unwrap();
}
