use std::io; use std::str; use std::time::SystemTime;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

 #[derive(Debug)]
 struct Rnd {
     x: u32
 }
 impl Rnd {
     fn new() -> Rnd {
         let now = SystemTime::now();
         let duration = now.duration_since(SystemTime::UNIX_EPOCH);
         Rnd { x: duration.unwrap().subsec_millis() }
     }
     fn next(&mut self) -> u32 { 
         let x = self.x;
         let x = x ^ x << 13; let x = x ^ x >> 17; let x = x ^ x << 5;
         self.x = x; x
     }
 }

 fn swap(v: &mut Vec<usize>, x: usize, y: usize) {
     let tmp = v[x]; v[x] = v[y]; v[y] = tmp;
 }

 fn partition(v: &mut Vec<usize>, left: usize, right: usize, rnd: &mut Rnd) -> usize {
     let pivot = rnd.next() as usize % (right - left + 1) + left;
     swap(v, left, pivot);
     let mut p = left;
     for i in left+1..=right {
         if v[i] < v[left] { p += 1; swap(v, p, i); }
     }
     swap(v, left, p); p
 }

 fn select(v: &mut Vec<usize>, pos: usize, left: usize, right: usize, rnd: &mut Rnd) -> usize {
     // dbg!(&pos);
     // dbg!(&left, &right, &v);
     let pivot = partition(v, left, right, rnd);
     // dbg!(&pivot, &v);
     if pivot == pos {
         v[pivot]
     } else {
         if pos < pivot {
             select(v, pos, left, pivot - 1, rnd)
         } else {
             select(v, pos, pivot + 1, right, rnd)
         }

     }
 }

 fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
     let t = scan.token::<usize>().unwrap();
     let mut rnd = Rnd::new();
     for _ in 0..t {
         let n = scan.token::<usize>().unwrap();
         let mut v = Vec::new();
         for _ in 0..n {
             let item = scan.token::<usize>().unwrap();
             v.push(item);
         } 
         let mut res = 0;
         for i in 1..=n {
             let n = i;
             let med = if i % 2 == 1 {
                 select(&mut v.clone(), (n-1)/2, 0, n-1, &mut rnd)
             } else {
                 let med_1 = select(&mut v.clone(), (n-1)/2, 0, n-1, &mut rnd);
                 let med_2 = select(&mut v.clone(), n/2, 0, n-1, &mut rnd);
                 (med_1 + med_2) / 2
             };
             res += med;
         }
         writeln!(out, "{}", res);
     }
 }

