use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::BTreeSet;
use std::time::Instant;

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
    let mut pals: Vec<i32> = Vec::new();

    // let bef = Instant::now();
    for i in 100000..=999999 {
        let s = i.to_string();
        let r: String = s.chars().rev().collect();
        if s == r { pals.push(i); }
    }
    // dbg!(bef.elapsed());
    for _ in 0..n {
        let num = scan.token::<i32>().unwrap();
        let res = match pals.binary_search(&num) {
            Ok(idx) => pals[idx],
            Err(idx) => {
                let opts = vec![pals.get(idx-1), pals.get(idx), pals.get(idx+1)];
                let min = opts.iter()
                    .filter(|p| p.is_some())
                    .min_by_key(|p| (p.unwrap()-num).abs())
                    .unwrap().unwrap();
                *min
            }
        };
        writeln!(out, "{}", res);
    }
}
