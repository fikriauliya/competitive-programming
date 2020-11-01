use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let k = scan.token::<usize>().unwrap();
    let mut starts = BinaryHeap::new();
    let mut ends = BinaryHeap::new();
    for _ in 0..n {
        let st = scan.token::<usize>().unwrap();
        let ed = scan.token::<usize>().unwrap();
        starts.push(Reverse(st));
        ends.push(Reverse(ed));
    }

    let mut max_met = 0;
    let mut cur = 0;
    while let Some(ed) = ends.pop() {
        while !starts.is_empty() && 
            starts.peek().unwrap().0 < ed.0 { starts.pop(); cur += 1 }

        let to = ed.0 + k;
        let mut it_st = starts.iter();
        let mut met = cur;
        while let Some(st) = it_st.next() {
            if st.0 <= to { met += 1; }
            else { break; }
        }
        max_met = max_met.max(met);
        cur -= 1;
    }
    println!("{}", max_met);
}
