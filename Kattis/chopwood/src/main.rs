use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut count = vec![1;n+2];

    let mut tos = Vec::new();
    for i in 0..n {
        let to = scan.token::<usize>().unwrap();
        if i != n - 1 {
            count[to] += 1;
        }
        tos.push(to);
    }

    let mut pq = BTreeMap::new();
    for i in 1..=n+1 {
        pq.insert((count[i], i), ());
    }

    let mut res = Vec::new();
    for i in 0..tos.len() {
        let to = tos[i];
        let from = pq.iter().next();
        if let Some((fr,_)) = from {
            if fr.0 != 1 { writeln!(out, "Error"); return }
            if count[to] == 0 { writeln!(out, "Error"); return }
            count[fr.1] = fr.0 - 1;

            res.push(fr.1.to_string());
            pq.remove(&fr.clone());

            let mut to_replace = (count[to], to);
            pq.remove(&to_replace);
            to_replace.0 -= 1;
            pq.insert(to_replace, ());

            count[to] = to_replace.0;
        } else { writeln!(out, "Error"); return; }
    }
    writeln!(out, "{}", res.join("\n"));
}
