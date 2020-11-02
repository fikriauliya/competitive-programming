use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashSet;
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
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let n = scan.token::<u32>().unwrap();
        let mut set: HashSet<u32> = HashSet::new();
        let mut bag: VecDeque<u32> = VecDeque::new();
        let mut max = 0;
        for nn in 0..n {
            let item = scan.token::<u32>().unwrap();
            if set.contains(&item) {
                while let Some(front) = bag.pop_front() {
                    set.remove(&front);
                    if front == item { break; }
                }
            } 
            bag.push_back(item);
            set.insert(item);
            max = max.max(bag.len());
        }
        writeln!(out, "{}", max);
    }
}
