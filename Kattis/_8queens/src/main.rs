use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn count_bits(n: usize) -> u8 {
    let mut count = 0;
    let mut n = n;
    while n != 0 {
        n = n & (n - 1);
        count += 1;
    }
    count
}
fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut hor = 0u8;
    let mut ver = 0u8;
    let mut diag1 = 0u16;
    let mut diag2 = 0u16;
    for i in 0..8 {
        let line = scan.line().unwrap();
        let line: Vec<char> = line.chars().collect();
        for j in 0..8 {
            if line[j] == '*' {
                ver ^= 1 << (7-j);
                hor ^= 1 << (7-i);
                diag1 ^= 1 << (i+j);
                diag2 ^= 1 << (7 + i as i8-j as i8) as usize;
            }
        }
    }
    let all_set = (((1 as u16) << 8) - 1) as u8;
    if hor == all_set && ver == all_set && count_bits(diag1 as usize) == 8 && count_bits(diag2 as usize) == 8 {
        writeln!(out, "valid");
    } else {
        writeln!(out, "invalid");
    }
}
