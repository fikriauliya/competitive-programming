use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for _ in 0..t {
        let n = scan.token::<usize>().unwrap();
        let mut sum = 0;
        let mut v = Vec::new();
        for _ in 0..n {
            let num = scan.token::<usize>().unwrap();
            v.push(num);
            sum ^= num
        }
        let mut cur = 0;
        // dbg!(format!("{:b}", sum));
        let mut count = 0;
        for i in 0..n {
            // dbg!(format!("{:b}", v[i]));
            cur ^= v[i];
            // dbg!(format!("{:b}", cur));
            if cur == sum { count += 1; cur = 0 }
        }
        if n < 2 {
            writeln!(out, "NO",).unwrap();
        } else {
            if sum == 0 {
                writeln!(out, "YES").unwrap();
            } else {
                if count > 1 {
                    writeln!(out, "YES").unwrap();
                } else {
                    writeln!(out, "NO").unwrap();
                }
            }
        }
    }
}
