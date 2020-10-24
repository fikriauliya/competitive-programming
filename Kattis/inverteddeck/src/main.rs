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

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut v = vec![];
    for i in 0..n {
        let item = scan.token::<u32>().unwrap();
        v.push(item);
    }
    let mut sorted = v.clone();
    sorted.sort();

    let mut st = None;
    let mut ed = None;
    for i in 0..n {
        match (st, ed, v[i], sorted[i]) {
            (None, None, a, b) if a != b => { st = Some(i); },
            (Some(_), _, a, b) if a != b => { ed = Some(i); },
            _ => (),
        }
    }
    // dbg!(&st, &ed);
    match (st, ed) {
        (Some(_), None) => { ed = Some(n-1) }
        (None, None) => { st = Some(0); ed = Some(0) }
        _ => (),
    }
    if let (Some(st), Some(ed)) = (st, ed) {
        let sub1 = &v[st..ed+1];
        let sub2 = &sorted[st..ed+1];
        let mut sub1: Vec<u32> = sub1.iter().map(|it| *it).collect();
        sub1.reverse();
        if sub1 == sub2 { println!("{} {}", st+1, ed+1); } 
        else { println!("impossible"); }
    }
}
