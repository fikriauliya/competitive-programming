use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn check(v: &mut Vec<Vec<isize>>, i: usize, j: usize, r: usize, c: usize) -> usize {
    let surs = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut res = 0;
    for d in &surs {
        let sur = (i as isize + d.0, j as isize + d.1);
        if (0..r as isize).contains(&sur.0) && (0..c as isize).contains(&sur.1) {
            let sur = (sur.0 as usize, sur.1 as usize);
            if v[sur.0][sur.1] > v[i][j] + 1 {
                let delta = v[sur.0][sur.1] - v[i][j] - 1;
                v[i][j] += delta;
                res += delta as usize;
            }
        }
    }
    for d in &surs {
        let sur = (i as isize + d.0, j as isize + d.1);
        if (0..r as isize).contains(&sur.0) && (0..c as isize).contains(&sur.1) {
            let sur = (sur.0 as usize, sur.1 as usize);
            if v[i][j] > v[sur.0][sur.1] + 1 {
                res += check(v, sur.0, sur.1, r, c);
            }
        }
    }
    res
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let r = scan.token::<usize>().unwrap();
        let c = scan.token::<usize>().unwrap();
        let mut v = vec![vec![0;c];r];
        for i in 0..r {
            for j in 0..c {
                v[i][j] = scan.token::<isize>().unwrap();
            }
        }
        let mut res = 0;
        for i in 0..r {
            for j in 0..c {
                res += check(&mut v, i, j, r, c);
            }
        }
        writeln!(out, "Case #{}: {}", tt+1, res).unwrap();
    }
}
