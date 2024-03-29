use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{VecDeque, LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rev(v: &mut VecDeque<isize>, len: usize) {
    for i in 0..len/2 {
        let tmp = v[len-1-i];
        v[len-1-i] = v[i];
        v[i] = tmp;
    }
}

fn rec(n: isize, c: isize, ctr: isize) -> Option<VecDeque<isize>> {
    let rev_len = (c-(n-2)).min(n);

    if n < 1 { return None }
    if n == 1 {
        if c == 0 { return Some(vec![ctr].into_iter().collect()) }
        else { return None }
    }
    if rev_len < 1 { return None; }
    let tails = rec(n-1, c-rev_len, ctr+1);
    match tails {
        None => { None }
        Some(mut tails) => {
            tails.push_front(ctr);
            rev(&mut tails, rev_len as usize);
            Some(tails)
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();    
    for tt in 0..t {
        let n = scan.token::<isize>().unwrap();
        let c = scan.token::<isize>().unwrap();
        let res = rec(n, c, 1);
        match res {
            Some(res) => {
                let res: Vec<String> = res.into_iter().map(|s| s.to_string()).collect();
                let res = res.join(" ");
                writeln!(out, "Case #{}: {}", tt+1, res).unwrap();
            },
            None => {
                writeln!(out, "Case #{}: {}", tt+1, "IMPOSSIBLE").unwrap();
            }
        }
    }
}
