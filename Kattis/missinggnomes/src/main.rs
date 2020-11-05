use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<u32>().unwrap();
    let m = scan.token::<u32>().unwrap();
    let mut rems =  HashSet::new();
    let mut items = VecDeque::new();

    for i in 1..=n {
        rems.insert(i);
    }

    for i in 0..m {
        let item = scan.token::<u32>().unwrap();
        rems.remove(&item);
        items.push_back(item);
    }

    let mut sorted: Vec<&u32> = rems.iter().collect();
    sorted.sort_by(|a, b| b.cmp(&a));

    while let Some(item) = items.pop_front() {
        let mut last = sorted.last();
        while let Some(rem) = last {
            if **rem > item { break; }

            writeln!(out, "{}", rem);
            sorted.pop();

            last = sorted.last();
        } 
        writeln!(out, "{}", item);
    }
    while let Some(rem) = sorted.pop() {
        writeln!(out, "{}", rem);
    }
}
