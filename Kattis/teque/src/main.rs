use std::io;
use std::str;
use std::convert::TryFrom;
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
    let n = scan.token::<usize>().unwrap();
    let mut left = VecDeque::new();
    let mut right = VecDeque::new();
    for _ in 0..n {
        let line = scan.line().unwrap();
        let mut split = line.split_ascii_whitespace();
        let op = split.next().unwrap();
        let param = split.next().unwrap().parse::<usize>().unwrap();
        match op {
            "push_back" => { 
                right.push_back(param);
                if right.len() > left.len() { 
                    let el = right.pop_front().unwrap();
                    left.push_back(el);
                }
            },
            "push_front" => { 
                left.push_front(param);
                if left.len() > right.len() + 1 {
                    let el = left.pop_back().unwrap();
                    right.push_front(el);
                }
            },
            "push_middle" => { 
                if right.len() == left.len() {
                    left.push_back(param);
                } 
                else {
                    right.push_front(param);
                }
            },
            "get" => {
                if param < left.len() {
                    println!("{}", left[param]);
                } else {
                    println!("{}", right[param - left.len()]);
                }
            },
            _ => panic!()
        }
    }
}
