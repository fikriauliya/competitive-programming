use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    for _ in 0..n {
        let mut a1 = scan.token::<i64>().unwrap();
        let a2 = scan.token::<i64>().unwrap();
        let mut b1 = scan.token::<i64>().unwrap();
        let mut b2 = scan.token::<i64>().unwrap();
        let mut c1 = scan.token::<i64>().unwrap();
        let mut c2 = scan.token::<i64>().unwrap();
        
        // TODO: Direct formula doesn't work, why?
        // let prev_a1 = a1;

        // a1 *= a2;
        // b1 *= a2;
        // c1 *= a2;

        // b2 *= prev_a1;
        // c2 *= prev_a1;

        // if b1 - b2 == 0 { writeln!(out, "?"); continue; }
        // let y = (c1 - c2) / (b1 - b2);
        // let rem = (c1 - c2) % (b1 - b2);
        // if rem != 0 { writeln!(out, "?"); continue;}

        // let x = (c1 - b1 * y) / a1;
        // let rem = (c1 - b1 * y) % a1;
        // if rem != 0 { writeln!(out, "?"); continue;}
        // if x <= 0 || y <= 0 { writeln!(out, "?"); continue;}

        let mut x = 1;
        let mut res = Vec::new();
        loop {
            let y = (c1 - a1 * x) / b1;
            let rem = (c1 - a1 * x) % b1;
            if y <= 0 { break; }
            if rem == 0 {
                if a2 * x + b2 * y == c2 {
                    res.push((x, y));
                }
            }
            x += 1;
        }
        if res.len() == 1 {
            writeln!(out, "{} {}", res[0].0, res[0].1);
        } else {
            writeln!(out, "?");
        }
    }
}
