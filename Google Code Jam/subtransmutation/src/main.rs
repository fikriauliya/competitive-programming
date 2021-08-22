use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(n: usize, req: &mut HashMap<usize, usize>, a: usize, b: usize) {
    // dbg!(&n, &req);
    if n <= 0 { return }
    if req.is_empty() { return }
    match req.get(&n) {
        Some(rem) => {
            if *rem == 1 {
                req.remove(&n);
            } else {
                req.insert(n, rem - 1);
            }
        }
        None => {
            if n >= a { rec(n-a, req, a, b); }
            if n >= b { rec(n-b, req, a, b); }
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let n = scan.token::<usize>().unwrap();
        let a = scan.token::<usize>().unwrap();
        let b = scan.token::<usize>().unwrap();
        let mut req = HashMap::new();
        for i in 0..n {
            let num = scan.token::<usize>().unwrap();
            if num > 0 {
                req.insert(i+1, num);
            }
        }
        let mut res = None;
        let mut i = 1;
        loop {
            let mut cur_req = req.clone();
            rec(i, &mut cur_req, a, b);
            if cur_req.is_empty() { res = Some(i); break; }
            i += 1;
        }
        writeln!(out, "Case #{}: {}", tt+1, res.unwrap()).unwrap();
    }
}
