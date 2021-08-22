use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn to_num(str: String) -> u64 {
    str.parse::<u64>().unwrap()
}

fn to_str(num: u64) -> String {
    num.to_string()
}

fn rec(cur: u64, init: u64, ctr: u64, max: u64, res: &mut Vec<u64>) -> u64 {
    // dbg!(cur, init, ctr, min);
    if ctr > 1 {
        res.push(cur);
    }
    if cur > max { 
        // dbg!(cur); 
        return cur; 
    }
    rec(to_num(to_str(cur) + &to_str(init+ctr)), init, ctr+1, max, res)
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    let mut all = Vec::new();
    let max = 10e20 as u64;
    for i in 1..=10e6 as u64 {
        rec(i, i, 1, max, &mut all);
    }
    dbg!(all.len());
    all.sort();
    // dbg!(&all);
    for tt in 0..t {
        let min = scan.token::<u64>().unwrap();
        let res = match all.binary_search(&min) {
            Ok(i) => {all[i+1]},
            Err(i) => {all[i]},
        };
        // let mut res = std::u64::MAX;
        // for i in 1..=to_str(min).len() {
        //     let cur = to_num(to_str(min)[..i].to_string());
        //     // dbg!(cur);
        //     res = res.min(rec(cur, cur, 1, min));
        //     if i != to_str(min).len() {
        //         res = res.min(rec(cur+1, cur+1, 1, min));
        //     }
        // }
        // let mut prefix = 1;
        // // loop {
        // //     if prefix > min { break; }
        // //     res = res.min(rec(prefix, prefix, 1, min));
        // //     prefix *= 10;
        // }
        writeln!(out, "Case #{}: {}", tt+1, res).unwrap();
    }
}
