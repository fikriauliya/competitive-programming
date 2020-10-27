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
    let n = scan.token::<usize>().unwrap();
    let c = scan.token::<usize>().unwrap();
    let mut freq = HashMap::new();
    let mut order = HashMap::new();
    for i in 0..n {
        let el = scan.token::<usize>().unwrap();
        match freq.get_mut(&el) {
            Some(s) => {*s += 1; ()},
            None => {
                freq.insert(el, 1);
                order.insert(el, i);
                ()
            },
        }
    }
    let mut counting: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, v) in freq {
        match counting.get_mut(&v) {
            Some(s) => {s.push(k); ()},
            None => {counting.insert(v, vec![k]); ()}
        }
    }
    let mut res = Vec::new();
    for i in (1..1001).rev() {
        match counting.get_mut(&i) {
            Some(c) => {
                c.sort_by_key(|k| order.get(k).unwrap());
                for it in c {
                    for _ in 0..i {
                        res.push(*it);
                    }
                }
            },
            None => ()
        }
    }
    let res: Vec<String>= res.iter().map(|it| it.to_string()).collect();
    println!("{}", res.join(" "));
}
