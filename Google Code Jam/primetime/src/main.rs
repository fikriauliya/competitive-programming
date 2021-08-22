
use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(v: &mut Vec<usize>, mult: usize, sum: usize, rem_sum: usize) -> Option<usize> {
    if mult > sum + rem_sum {  return None; }
    if v.len() == 0 {
        if sum == mult { return Some(sum) }
        return None;
    }
    let num = v.pop().unwrap();
    let res = rec(v, mult * num, sum, rem_sum - num);
    let res1 = rec(v, mult, sum + num, rem_sum - num);
    v.push(num);
    match (res, res1) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (None, Some(x)) => Some(x),
        (Some(x), None) => Some(x),
        _ => None
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let m = scan.token::<usize>().unwrap();
        let mut v = Vec::new();
        for i in 0..m {
            let num = scan.token::<usize>().unwrap();
            let count = scan.token::<usize>().unwrap();
            for _ in 0..count {
                v.push(num);
            }
        }

        let sum = v.iter().sum();
        let res = rec(&mut v, 1, 0, sum);
        writeln!(out, "Case #{}: {}", tt+1, res.unwrap_or(0)).unwrap();
    }
}
