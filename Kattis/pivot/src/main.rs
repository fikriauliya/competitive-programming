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
    let mut arr = Vec::new();
    let mut max = Vec::new();
    let mut min = Vec::new();
    let mut cur_max = None;
    let mut cur_min = None;
    for i in 0..n {
        let item = scan.token::<i64>().unwrap();
        arr.push(item);
    }
    for i in 0..n {
        cur_max = match cur_max {
            None => Some(arr[i]),
            Some(ma) => Some(arr[i].max(ma)),
        };
        max.push(cur_max.unwrap());
    }
    for i in (0..n).rev() {
        cur_min = match cur_min {
            None => Some(arr[i]),
            Some(mi) => Some(arr[i].min(mi)),
        };
        min.push(cur_min.unwrap());
    }
    min.reverse();
    let res = arr.iter().enumerate().filter(|(i, it)| *it >= &max[*i] && *it <= &min[*i]).count();
    println!("{}", res);
}
