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
    let n = scan.token::<u32>().unwrap();
    for nn in 0..n {
        let mut s:Vec<char> = scan.line().unwrap().chars().collect();
        let t:Vec<char> = scan.line().unwrap().chars().collect();

        let mut res = 0;

        let mut zero_needs = t.iter().filter(|a| a == &&'0').count() as i32 - s.iter().filter(|a| a == &&'0').count() as i32;
        for i in 0..s.len() {
            if zero_needs <= 0 { break }
            if s[i] == '?' && t[i] == '0' {
                s[i] = '0';
                res += 1;
                zero_needs -= 1;
            }
        }
        for i in 0..s.len() {
            if zero_needs <= 0 { break }
            if s[i] == '?' {
                s[i] = '0';
                res += 1;
                zero_needs -= 1;
            }
        }
        if zero_needs > 0 {
            writeln!(out, "Case {}: {}", nn+1, -1);
            continue;
        }
        for i in 0..s.len() {
            if s[i] == '?' {
                s[i] = '1';
                res += 1;
            }
        }

        let mut one_needs = t.iter().filter(|a| a == &&'1').count() as i32 - s.iter().filter(|a| a == &&'1').count() as i32;
        for i in 0..s.len() {
            if one_needs <= 0 { break }
            if s[i] == '0' && t[i] == '1' {
                s[i] = '1';
                res += 1;
                one_needs -= 1;
            }
        }

        let missplaced = s.iter().zip(t.iter()).filter(|(a, b)| a != b).count();
        res += missplaced/2;
        writeln!(out, "Case {}: {}", nn+1, res);
    }
}
