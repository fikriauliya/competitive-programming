use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

struct UnionFind {
    ranks: Vec<usize>,
    parents: Vec<usize>,
    set_size: Vec<usize>
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        let ranks = vec![1; size];
        let mut parents = vec![0; size];
        let set_size = vec![1; size];
        for i in 0..size { parents[i] = i }
        UnionFind { ranks, parents, set_size }
    }

    fn find(&mut self, val: usize) -> usize {
        if self.parents[val] == val { return val }
        self.parents[val] = self.find(self.parents[val]);
        self.parents[val]
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn union(&mut self, a: usize, b: usize) -> usize {
        if self.same(a, b) { return self.find(a); }
        let a = self.find(a); let b = self.find(b);
        if self.ranks[a] > self.ranks[b] { 
            self.parents[b] = a; self.set_size[a] += self.set_size[b]; 
            a
        }
        else if self.ranks[a] < self.ranks[b] { 
            self.parents[a] = b; self.set_size[b] += self.set_size[a];
            b
        }
        else {
            self.parents[b] = a;
            self.ranks[a] += 1; self.set_size[a] += self.set_size[b];
            a
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let k = scan.token::<usize>().unwrap();
    let mut uf = UnionFind::new(n+1);
    for _ in 0..k {
        let a = scan.token::<usize>().unwrap();
        let b = scan.token::<usize>().unwrap();
        uf.union(a, b);
    }
    let mut res = true;
    for i in 1..=n/2 {
        if !uf.same(i, n-i+1) { res = false; break; }
    }
    if res { writeln!(out, "Yes"); } else { writeln!(out, "No"); }
}
