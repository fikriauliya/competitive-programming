use std::{i64::MAX, io}; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Debug)]
struct Dance {
    data: usize
}

impl Dance {
    fn new(line: String) -> Dance {
        let mut res = Dance { data: 0 };
        for c in line.chars() {
            let ord = (c as u8) - ('A' as u8);
            res.data = res.data | (1 << ord);
        }
        res
    }

    fn diff(&self, other: &Dance) -> u8 {
        let mut tmp = other.data & self.data;
        let mut res = 0u8;
        while tmp > 0 {
            tmp &= tmp-1;
            res += 1 ;
        }
        res
    }
}

fn rec(v: &mut Vec<Dance>, pos: usize) -> usize {
    if pos == v.len() - 1 {
        let mut total_diff = 0;
        for i in 1..v.len() {
            total_diff += v[i].diff(&v[i-1]) as usize;
        }
        total_diff
    } else {
        let mut min_diff = std::usize::MAX;
        for i in pos..v.len() {
            v.swap(pos, i);
            let diff= rec(v, pos + 1);
            min_diff = min_diff.min(diff);
            v.swap(pos, i);
        }
        min_diff
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let r = scan.token::<usize>().unwrap();
    let mut v = Vec::new();
    for _ in 0..r {
        v.push(Dance::new(scan.line().unwrap()));
    }
    let res = rec(&mut v, 0);
    writeln!(out, "{}", res).unwrap();
}
