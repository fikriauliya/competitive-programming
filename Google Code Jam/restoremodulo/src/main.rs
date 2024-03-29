use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let n = scan.token::<usize>().unwrap();
        let mut m = Vec::with_capacity(n);
        for i in 0..n {
            let a = scan.token::<isize>().unwrap();
            m.push(a);
        }
        let mut up = None;
        let mut down = None;
        let mut error = false;
        for i in 1..n {
            if m[i] >= m[i-1] {
                let delta = m[i] - m[i-1];
                match up {
                    None => { up = Some(delta); },
                    Some(d) => {
                        if d != delta {
                            error = true;
                            break;
                        }
                    }
                }
            }
        }
        for i in 1..n {
            if m[i] < m[i-1] {
                let delta = m[i-1] - m[i];
                match down {
                    None => { down = Some(delta); },
                    Some(d) => {
                        if d != delta {
                            error = true;
                            break;
                        }
                    }
                }
            }
        }
        if error {
            writeln!(out, "-1").unwrap();
        } else {
            match (up, down) {
                (None, None) => { writeln!(out, "0").unwrap(); }
                (Some(_), None) => { writeln!(out, "0").unwrap(); }
                (None, Some(_)) => { writeln!(out, "0").unwrap(); }
                (Some(up), Some(down)) => {
                    let modulo = up + down;
                    if m.into_iter().max().unwrap() >= modulo {
                        writeln!(out, "-1").unwrap();
                    } else {
                        writeln!(out, "{} {}", modulo, up).unwrap();
                    }
                }
            }
        }
    }
}
