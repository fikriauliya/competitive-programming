use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let p = scan.token::<u16>().unwrap();
    let t = scan.token::<u16>().unwrap();
    let mut opinions:HashMap<u16, u128> = HashMap::new();
    for i in 1..=p { opinions.insert(i, 0); }
    loop {
        let line = scan.line();
        if line.is_none() { break; }
        let line = line.unwrap();
        let mut it = line.split_ascii_whitespace();

        let person = it.next().unwrap().parse::<u16>().unwrap();
        let tree = it.next().unwrap().parse::<u16>().unwrap();
        match opinions.get_mut(&person) {
            Some(opinion) => {*opinion |= 1 << tree; },
            None => {opinions.insert(person, 1 << tree); },
        }
    }
    let mut uniques = HashSet::new();
    for (_, op) in opinions {
        uniques.insert(op);
    }
    writeln!(out, "{}", uniques.len());
}
