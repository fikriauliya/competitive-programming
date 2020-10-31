use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::BinaryHeap;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let k = scan.token::<usize>().unwrap();
    let n = scan.token::<usize>().unwrap();

    let my_year = scan.token::<usize>().unwrap();
    let my_strength = scan.token::<usize>().unwrap();

    let mut p = BinaryHeap::new();
    let mut q = Vec::new();
    q.push((my_year, my_strength));

    loop {
        let year = scan.token::<usize>();
        if year.is_none() { break };
        let year = year.unwrap();
        let strength = scan.token::<usize>().unwrap();
        q.push((year, strength));
    }

    q.sort_by_key(|o| o.0);
    let mut i = 0;
    let mut year = None;
    while i < q.len() {
        let mut cur = q[i];
        p.push(cur.1);
        while i < q.len() - 1 && cur.0 == q[i+1].0 {
            i += 1;
            cur = q[i];
            p.push(cur.1);
        }
        i += 1;
        
        if my_strength == p.pop().unwrap() {
            year = Some(cur.0);
            break;
        }
    }
    if let Some(year) = year {
        println!("{}", year);
    } else {
        println!("unknown");
    }
}
