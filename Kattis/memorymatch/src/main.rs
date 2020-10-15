use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::HashSet;

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
    let k = scan.token::<usize>().unwrap();
    let mut m:HashMap<String, HashSet<usize>> = HashMap::new();

    let mut remaining = n;

    for i in 0..k {
        let a = scan.token::<usize>().unwrap();
        let b = scan.token::<usize>().unwrap();
        
        let ca = scan.token::<String>().unwrap();
        let cb = scan.token::<String>().unwrap();

        if let Some(v) = m.get_mut(&ca) {
            v.insert(a);
        } else {
            m.insert(ca.clone(), [a].iter().cloned().collect());
        }
        if let Some(v) = m.get_mut(&cb) {
            v.insert(b);
        } else {
            m.insert(cb.clone(), [b].iter().cloned().collect());
        }

        if ca == cb {
            remaining -= 2;
            m.remove(&ca);
        }
    }

    let mut res = 0;
    dbg!(&remaining);
    res += m.values().filter(|v| v.len() == 2).count();
    remaining -= m.values().filter(|v| v.len() == 2).count() * 2;

    dbg!(&remaining);
    let opened = m.values().filter(|v| v.len() == 1).count();
    if remaining == 2 * opened {
        res += opened;
    }
    if remaining == 2 && opened == 0 {
        res += 1;
    }
    dbg!(&m);
    writeln!(out, "{}", res);
}
