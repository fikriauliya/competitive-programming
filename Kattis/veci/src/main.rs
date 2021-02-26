use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(v: &mut Vec<u32>, i: usize, res: &mut HashSet<u32>) {
    if i == v.len() - 1 { 
        res.insert(v.iter().fold(0, |acc, x| acc * 10 + x));
        return; 
    }

    for j in 0..v.len() {
        v.swap(i, j);
        rec(v, i+1, res);
        v.swap(i, j);
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut permutations: HashSet<u32> = HashSet::new();
    let x = scan.token::<u32>().unwrap();
    let mut nums: Vec<u32> = x.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    rec(&mut nums, 0, &mut permutations);
    let mut permutations: Vec<u32> = permutations.into_iter().collect();
    permutations.sort();
    let res = match permutations.binary_search(&x) {
        Ok(x) => permutations.get(x+1),
        Err(x) => permutations.get(x)
    };
    let res = res.unwrap_or(&0);
    writeln!(out, "{}", res).unwrap();
}
