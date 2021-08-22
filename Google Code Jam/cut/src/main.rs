use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let q = scan.token::<usize>().unwrap();
    let mut v = Vec::new();
    for i in 0..n {
        let num = scan.token::<usize>().unwrap();
        v.push(num);
    }

    let mut factors: HashMap<usize, Vec<usize>> = HashMap::new();
    let max = *v.iter().max().unwrap();
    let mut primes = vec![true;max+1];
    primes[0] = false; primes[1] = false;
    let mut ps = Vec::new();
    for i in 2..=(max as f32).sqrt() as usize {
        if primes[i] {
            for j in (i*i..=max).step_by(i) {
                primes[j] = false;
            }
        }
    }
    for i in 0..=max {
        if primes[i] {
            ps.push(i);
        }
    }
    factors.insert(1, vec![1]);
    for i in 0..n {
        for p in ps.iter() {
            if v[i] % p == 0 {
                match factors.get_mut(&v[i]) {
                    Some(v) => {v.push(*p);}
                    None => {factors.insert(v[i], vec![*p]);},
                }
            }
        }
    }
    // for i in 1..=max {
    //     dbg!(i, factors.get(&i).unwrap());
    // }

    for i in 0..q {
        let l = scan.token::<usize>().unwrap()-1;
        let r = scan.token::<usize>().unwrap()-1;
        let mut res = 1;
        let mut used = HashSet::new();
        for i in l..=r {
            let mut failed = false;
            for factor in factors.get(&v[i]).unwrap() {
                if used.contains(&factor) {
                    failed = true;
                    break;
                } 
                used.insert(factor);
            }
            if failed {
                res += 1;
                used.clear();
                for factor in factors.get(&v[i]).unwrap() {
                    used.insert(factor);
                }
            }
        }
        writeln!(out, "{}", res).unwrap();
    }
}
