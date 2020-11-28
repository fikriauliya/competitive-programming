use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let r = scan.token::<isize>().unwrap();
    let c = scan.token::<isize>().unwrap();
    let mut s = vec![vec![false;c as usize];r as usize];
    for i in 0..r {
        let line = scan.line().unwrap();
        let chars:Vec<char> = line.chars().collect();
        for j in 0..c {
            s[i as usize][j as usize] = chars[j as usize] == 'o';
        }
    }
    let mut mirko = 0;
    let mut res = 0;
    for i in 0..r {
        for j in 0..c {
            if !s[i as usize][j as usize] {
                let mut hs = 0;
                for k in (i-1).max(0)..(i+2).min(r) {
                    for l in (j-1).max(0)..(j+2).min(c) {
                        if s[k as usize][l as usize] {
                            hs += 1;
                        }
                    }
                }
                mirko = mirko.max(hs);
            } else {
                let mut hs = 0;
                for k in (i-1).max(0)..(i+2).min(r) {
                    for l in (j-1).max(0)..(j+2).min(c) {
                        if !(i == k && j == l) && s[k as usize][l as usize] {
                            hs += 1;
                        }
                    }
                }
                res += hs;
            }
        }
    }
    dbg!(&res);
    dbg!(&mirko);
    res += mirko * 2;
    writeln!(out, "{}", res/2);
}
