use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut boys = Vec::with_capacity(n);
    for _ in 0..n {
        boys.push(scan.token::<i64>().unwrap());
    }
    let mut a = scan.token::<i64>().unwrap();
    let mut b = scan.token::<i64>().unwrap();
    if a % 2 == 0 { a += 1 }
    if b % 2 == 0 { b -= 1 }
    boys.sort();

    let mut res = 0;
    let mut max_dist = 0;

    for num in vec![a, b] {
        match boys.binary_search(&num) {
            Ok(_) => { unreachable!() },
            Err(pos) => {
                let dist = if pos == 0 {
                    boys[0] - num
                } else if pos == n {
                    num - boys[n-1]
                } else {
                    (num - boys[pos - 1]).min(boys[pos] - num)
                };
                if dist > max_dist { 
                    res = num;
                    max_dist = dist;
                };
            }
        };
    }
    for i in 0..n-1 {
        let mut dist = (boys[i + 1] - boys[i]) / 2;
        let mid = boys[i] + dist;
        if mid % 2 == 1 {
            if (a..=b).contains(&mid) {
                if dist > max_dist {
                    res = mid;
                    max_dist = dist;
                }
            }
        } else {
            dist -= 1;
            for delta in vec![-1, 1] {
                if (a..=b).contains(&(mid + delta)) {
                    if dist > max_dist {
                        res = mid + delta;
                        max_dist = dist;
                    }
                }
            }            
        }
    }

    writeln!(out, "{}", res);
}
