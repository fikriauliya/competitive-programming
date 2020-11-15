use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    loop {
        let line = scan.line();
        if line.is_none() { break; }
        let line = line.unwrap();
        let mut nums: Vec<usize> = line.split_ascii_whitespace().into_iter().map(|w| w.parse::<usize>().unwrap()).collect();
        nums.sort();
        
        let mut set = HashSet::new();
        let mut dupl = false;
        'outer: for i in 1..nums.len() {
            for j in 0..i {
                let delta = nums[i] - nums[j];
                if set.contains(&delta) {
                    dupl = true;
                    break 'outer;
                } else {
                    set.insert(delta);
                }
            }
        }
        if dupl {
            writeln!(out, "not a ruler");
        } else if set.len() == nums[nums.len() - 1]  {
            writeln!(out, "perfect");
        } else {
            let mut missing = Vec::new();
            for i in 1..nums[nums.len() - 1] {
                if !set.contains(&i) {
                    missing.push(i.to_string());
                }
            }
            writeln!(out, "missing {}", missing.join(" "));
        }
    }
}
