use std::io;
use std::str;
use std::convert::TryFrom;
use std::cmp::Ordering;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Eq, PartialEq)]
struct Word(String);

impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        let rev_self: String = self.0.to_string().chars().rev().collect();
        let rev_other: String = other.0.to_string().chars().rev().collect();
        rev_self.cmp(&rev_other)
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn s(mut words: Vec<Word>) -> Vec<Word> {
    words.sort();
    let max_len = words.iter().map(|it| it.0.len()).max().unwrap();
    for word in &words {
        println!("{:>width$}", word.0, width = max_len)
    }
    words
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut words = Vec::new();
    let mut ctr = 0;
    loop {
        let line = scan.line();
        if let None = line { 
            if ctr != 0 { println!() }
            s(words); break;
        }
        let line = line.unwrap();
        if line.len() == 0 { 
            if ctr != 0 { println!() }
            words = s(words); 
            ctr += 1;
            words.clear() 
        }
        else {
            words.push(Word(line));
        }
    }
}
