use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn ls_one(n: usize) -> usize {
    (n as isize & -(n as isize)) as usize
}

fn rec(pairs: &HashSet<(String, String)>,
       names: &Vec<String>,
       not_visited: usize,
       visited: &mut Vec<String>) -> Option<Vec<String>> {
    if visited.len() == names.len() {
        return Some(visited.clone());
    }
    let mut tmp = not_visited;
    loop {
        let ls = ls_one(tmp);
        if ls == 0 { break; }
        let i = (ls as f32).log2() as usize;

        if visited.is_empty() || !pairs.contains(&(visited.last().unwrap().clone(), names[i].clone())) {
            visited.push(names[i].clone());
            let r = rec(pairs, names, not_visited & !ls, visited);
            if r.is_some() { return r; }
            visited.pop();
        }
        tmp = tmp & !ls;
    }
    return None;
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    loop {
        let n = scan.token::<usize>();
        if n.is_none() { return; }
        let n = n.unwrap();
        let mut names = Vec::new();
        for _ in 0..n {
            names.push(scan.line().unwrap());
        }
        let m = scan.token::<usize>().unwrap();
        let mut pairs = HashSet::new();
        for _ in 0..m {
            let l = scan.line().unwrap();
            let mut it = l.split_ascii_whitespace();
            let a = it.next().unwrap().clone().to_owned();
            let b = it.next().unwrap().clone().to_owned();
            pairs.insert((a.clone(), b.clone()));
            pairs.insert((b, a));
        }
        names.sort();
        let res = rec(&pairs, &names, (1<<names.len()) - 1, &mut vec![]);
        match res {
            Some(mut res) => writeln!(out, "{}", res.join(" ")),
            None => writeln!(out, "You all need therapy.")
        };
    }
}
