use std::{io, iter::FromIterator}; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
use std::time::Instant;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(v: &mut Vec<usize>, i: usize, res: &mut HashSet<usize>) {
    if i == v.len() - 1 { 
        let cur = v.iter().fold(0, |acc, x| acc * 10 + x);
        res.insert(cur);

        return; 
    }

    for j in 0..v.len() {
        v.swap(i, j);
        rec(v, i+1, res);
        v.swap(i, j);
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for _ in 0..t { 
        let mut v: Vec<usize> = scan.line().unwrap().replace(" ", "")
            .chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        let mut perms = HashSet::new();

        rec(&mut v, 0, &mut perms);

        let mut perms:Vec<&usize> = perms.iter().collect();
        perms.sort();

        let mut nearest = None;
        let mut counter = 0;
        for perm in perms {
            let day = perm % 100;
            let month = (perm / 100) % 100;
            let year = perm / 10000;
            if year < 2000 { continue }
            if month == 0 || month > 12 { continue }
            if day == 0 { continue; }

            if [1, 3, 5, 7, 8, 10, 12].contains(&month) && day > 31 { continue; }
            if [4, 6, 9, 11].contains(&month) && day > 30 { continue; }
            if month == 2 {
                let is_leap = if year % 400 == 0 { true }
                else {
                    if year % 100 == 0 && year % 400 != 0 { false }
                    else if year % 4 == 0 && year % 100 != 0 { true }
                    else if year % 4 != 0 { false }
                    else { false }
                };
                if is_leap && day > 29 { continue; }
                if !is_leap && day > 28 { continue; }
            }
            if nearest.is_none() {
                nearest = Some(format!("{:02} {:02} {:04}", day, month, year));
            }
            counter += 1;
        }
        if counter == 0 {
            writeln!(out, "0");
        } else {
            writeln!(out, "{} {}", counter, nearest.unwrap());
        }
    }
}
