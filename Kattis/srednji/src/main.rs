use std::io;
use std::str;
use std::convert::TryFrom;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(v: &[usize], left_count: usize, right_count: usize, b: usize) -> usize {
    if v.is_empty() { return 0 };
    let mut res = 0;
    if left_count == right_count { res += 1; }
    {
        let lc = if v[0] < b {  left_count - 1 } else { left_count };
        let rc = if v[0] > b {  right_count - 1 } else { right_count };
        res += rec(&v[1..v.len()], lc, rc, b);
    }
    {
        let lc = if v[v.len()-1] < b {  left_count - 1 } else { left_count };
        let rc = if v[v.len()-1] > b {  right_count - 1 } else { right_count };
        res += rec(&v[0..v.len()-1], lc, rc, b);
    }
    res
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let b = scan.token::<usize>().unwrap();
    let mut v = Vec::new();
    for _ in 0..n {
        let item = scan.token::<usize>().unwrap();
        v.push(item);
    }
    let res = rec(&v, b-1, n-b, b);
    writeln!(out, "{}", res);
}
