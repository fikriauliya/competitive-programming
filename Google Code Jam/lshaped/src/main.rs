use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn len(a: usize, b: usize) -> usize {
    let (a, b) = (a+1, b+1);
    let mut res1 = b/2;
    if a < res1 {
        res1 = a;
    }
    res1 = if res1 == 0 { 0 } else { res1 - 1 };
    let (b, a) = (a, b);
    let mut res2 = b/2;
    if a < res2 {
        res2 = a;
    }
    res2 = if res2 == 0 { 0 } else { res2 - 1 };
    res1 + res2
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let r = scan.token::<usize>().unwrap();
        let c = scan.token::<usize>().unwrap();
        let mut m = vec![vec![false;c];r];
        for i in 0..r {
            for j in 0..c {
                let b = scan.token::<usize>().unwrap();
                m[i][j] = b == 1;
            }
        }
        let mut left = vec![vec![0;c];r];
        let mut right = vec![vec![0;c];r];
        let mut up = vec![vec![0;c];r];
        let mut down = vec![vec![0;c];r];
        for i in 0..r {
            for j in 0..c {
                if j > 0 && m[i][j] && m[i][j] == m[i][j-1] {
                    left[i][j] = left[i][j-1] + 1;
                }
                if i > 0 && m[i][j] && m[i][j] == m[i-1][j] {
                    up[i][j] = up[i-1][j] + 1;
                }
            }
        }
        for i in (0..r).rev() {
            for j in (0..c).rev() {
                if j < c-1 && m[i][j] && m[i][j] == m[i][j+1] {
                    right[i][j] = right[i][j+1] + 1;
                }
                if i < r-1 && m[i][j] && m[i][j] == m[i+1][j] {
                    down[i][j] = down[i+1][j] + 1;
                }
            }
        }

        let mut res = 0;
        for i in 0..r {
            for j in 0..c {
                if m[i][j] {
                    res += len(left[i][j], up[i][j]);
                    res += len(left[i][j], down[i][j]);
                    res += len(right[i][j], up[i][j]);
                    res += len(right[i][j], down[i][j]);
                }
            }
        }
        // dbg!(up, down, left, right);
        writeln!(out, "Case #{}: {}", tt + 1, res).unwrap();
    }
}
