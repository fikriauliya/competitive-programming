use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(v: &mut Vec<isize>, i: isize, delta: Option<isize>, chance: usize, memo: &mut HashMap<(isize, usize, isize, usize), usize>) -> usize {
    // dbg!(&v, i, delta, chance);
    if i == -1 { return 0 };
    let i = i as usize;
    match delta {
        Some(delta) => {
            let mem = memo.get(&(v[i+1], i, delta, chance));
            if mem.is_some() { return *mem.unwrap(); }

            let res;
            if v[i+1] - v[i] == delta {
                res = 1 + rec(v, i as isize - 1, Some(delta), chance, memo);
            } else {
                if chance == 0 {
                    res = 0
                } else {
                    let prev = v[i];
                    v[i] = v[i+1] - delta;
                    res = 1 + rec(v, i as isize - 1, Some(delta), 0, memo); 
                    v[i] = prev;
                }
            };
            memo.insert((v[i+1], i, delta, chance), res);
            res
        }
        None => {
            if i == 0 { return 1; }
            let mut res = 1 + rec(v, i as isize - 1, Some(v[i] - v[i-1]), 1, memo);
            if i >= 2 {
                let delta = v[i-1]-v[i-2];
                let prev = v[i];
                v[i] = v[i-1] + delta;
                let b = 1 + rec(v, i as isize - 1, Some(delta), 0, memo);
                v[i] = prev;
                res = res.max(b);

                let delta = (v[i]-v[i-2])/2;
                let c = 1 + rec(v, i as isize - 1, Some(delta), 1, memo);
                res = res.max(c);
            };
            res
        }
    }

}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let n = scan.token::<usize>().unwrap();
        let mut v = Vec::new();
        for _ in 0..n {
            let x = scan.token::<isize>().unwrap();
            v.push(x);
        }
        // dbg!(&v);
        let mut res = 0;
        let mut memo = HashMap::new();
        for i in 0..n {
            let cur = rec(&mut v, i as isize, None, 1, &mut memo);
            // dbg!(&v, i, cur);
            res = res.max(cur);
        }
        writeln!(out, "Case #{}: {}", tt+1, res).unwrap();
    }
}
