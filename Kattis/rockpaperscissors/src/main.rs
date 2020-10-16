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
    let mut first = true;
    loop {
        let n= scan.token::<i32>().unwrap();
        if n == 0 { break; }
        if first { first = false } else { writeln!(out, ""); () }
        let k = scan.token::<i32>().unwrap();
        let mut win_count = vec![0; (n+1) as usize];
        let mut lost_count = vec![0; (n+1) as usize];
        for i in 0..(k *n * (n -1) / 2) as usize {
            let (p1, o1, p2, o2) = (scan.token::<usize>().unwrap(),
                scan.token::<String>().unwrap(),
                scan.token::<usize>().unwrap(),
                scan.token::<String>().unwrap());
            let winner = match (&o1[..], &o2[..]) {
                ("rock", "scissors") => p1,
                ("scissors", "rock") => p2,
                ("paper", "rock") => p1,
                ("rock", "paper") => p2,
                ("scissors", "paper") => p1,
                ("paper", "scissors") => p2,
                _ => 0
            };
            if winner != 0 {
                win_count[winner] += 1;
                let losser = if winner == p1 { p2 } else { p1 };
                lost_count[losser] += 1;
            }
        }
        for i in 1..(n+1) as usize {
            let divisor = win_count[i]+lost_count[i];
            if divisor == 0 {
                writeln!(out, "-");
            } else {
                let score = win_count[i] as f32/divisor as f32;
                writeln!(out, "{:.3}", score);
            }
        }
    }
}
