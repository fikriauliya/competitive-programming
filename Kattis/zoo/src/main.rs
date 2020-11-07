use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashMap;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut t = 1;
    loop {
        let n = scan.token::<u16>().unwrap();
        if n == 0 { break; }
        let mut animals = HashMap::new();
        for _ in 0..n {
            let animal = scan.line().unwrap();
            let animal = animal.to_lowercase(); 
            let mut split = animal.split_ascii_whitespace();
            let family = split.next_back().unwrap().to_string();
            match animals.get_mut(&family) {
                None => { animals.insert(family, 1); },
                Some(c) => {*c += 1; },
            };
        }
        let mut keys: Vec<&String> = animals.keys().collect();
        keys.sort();
        writeln!(out, "List {}:", t);
        for k in keys {
            writeln!(out, "{} | {}", k, animals.get(k).unwrap());
        }
        t += 1;
    }
}
