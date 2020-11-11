use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

struct Fenwick {
    data: Vec<i64>
}

impl Fenwick {
    fn new(n: usize) -> Fenwick {
        let data = vec![0;n+1];
        Fenwick { data }
    }

    fn ls_one(n: isize) -> isize {
        (n as isize & -(n as isize)) as isize
    }

    fn rsq(&self, i: usize) -> i64 {
        let mut sum = 0i64;
        let mut i = i as isize;
        while i > 0 {
            sum += self.data[i as usize];
            i -= Fenwick::ls_one(i);
        }
        sum
    }

    fn rsq_range(&self, l: usize, r: usize) -> i64 {
        self.rsq(r) - self.rsq(l - 1)
    }

    fn update(&mut self, i: usize, inc: i64) {
        let mut i = i as isize;
        while (i as usize) < self.data.len() {
            self.data[i as usize] += inc;
            i += Fenwick::ls_one(i);
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let k = scan.token::<usize>().unwrap();
    let mut fen = Fenwick::new(n);
    let mut nums = vec![0;n+1];

    for _ in 0..k {
        let cmd = scan.token::<String>().unwrap();
        if cmd == "F" {
            let i = scan.token::<usize>().unwrap();
            let inc = if nums[i] == 0 { 1 } else { -1 };
            nums[i] = if nums[i] == 0 { 1 } else { 0 };
            fen.update(i, inc);
        } else {
            let l = scan.token::<usize>().unwrap();
            let r = scan.token::<usize>().unwrap();
            let res = fen.rsq_range(l, r);
            writeln!(out, "{}", res);
        }
    }

}
