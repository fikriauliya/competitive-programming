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
    let a = scan.token::<usize>().unwrap();
    let b = scan.token::<usize>().unwrap();
    let min = a.max(b/2+1);

    let mut alphabets = Vec::new();
    for c in 'a' as u8..='z' as u8 { alphabets.push(c as char); }

    let mut word = Vec::new();
    word.push('a');
    let mut ctr = 0;
    'case: for a in 'a' as u8..='z' as u8 {
        for b in 'a' as u8..='z' as u8 {
            for c in 'a' as u8..='z' as u8 {
                for d in 'a' as u8..='z' as u8 {
                    ctr += 1;
                    write!(out,"{}{}{}{} ", a as char, b as char, c as char, d as char);
                    if ctr == min { break 'case; }
                }
            }
        }
    }
    writeln!(out, "");
}
