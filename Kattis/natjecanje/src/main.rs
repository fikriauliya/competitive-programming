use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn ls_one(n: isize) -> isize {
    (n as isize & -(n as isize)) as isize
}

fn rec(damaged: isize, reserves: isize) -> usize {
    if damaged == 0 || reserves == 0 { return 0; }
   
    let mut bit = ls_one(reserves);
    let mut new_reserves = reserves & !bit;
    let mut max = 0;
    while bit > 0 {
        if bit & damaged > 0 {
            return 1 + rec(damaged & !bit, reserves & !bit)
        }
        if (bit << 1) & damaged > 0 {
            max = max.max(1 + rec(damaged & !(bit << 1), reserves & !bit));
        }
        if (bit >> 1) & damaged > 0 {
            max = max.max(1 + rec(damaged & !(bit >> 1), reserves & !bit));
        }
        bit = ls_one(new_reserves);
        new_reserves = new_reserves & !bit;
    }
    max
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let s = scan.token::<usize>().unwrap();
    let r = scan.token::<usize>().unwrap();
    let mut damaged: usize = 0;
    let mut reserves: usize = 0;
    let mut damaged_count = 0;
    for _ in 0..s {
        let i = scan.token::<usize>().unwrap();
        damaged |= 1 << i;
        damaged_count += 1;
    }
    for _ in 0..r {
        let i = scan.token::<usize>().unwrap();
        reserves |= 1 << i;
    }
    let repaired = rec(damaged as isize, reserves as isize);
    writeln!(out, "{}", damaged_count - repaired); 
}
