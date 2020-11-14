use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut i = 1;
    loop {
        let n = scan.token::<usize>();
        if n.is_none() { break; }
        let n = n.unwrap();
        let mut nums = Vec::new();
        for _ in 0..n {
            let num = scan.token::<isize>().unwrap();
            nums.push(num);
        }
        let mut sums = BTreeMap::new();
        for i in 0..n-1 {
            for j in i+1..n {
                sums.insert(nums[i] + nums[j], ());
            }
        }
        let m = scan.token::<isize>().unwrap();
        writeln!(out, "Case {}:", i);
        for _ in 0..m {
            let q = scan.token::<isize>().unwrap();
            let prev = sums.range(..q).next_back().map(|v| v.0);
            let next = sums.range(q..).next().map(|v| v.0);
            let res = match (prev, next) {
                (Some(p), Some(n)) => if (p-q).abs() > (n-q).abs() { n } else { p},
                (None, Some(n)) => n,
                (Some(p), None) => p,
                _ => panic!()
            };
            writeln!(out, "Closest sum to {} is {}.", q, res);
        }
        i += 1;
    }
}
