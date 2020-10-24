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
    let n = scan.token::<usize>().unwrap();
    let mut hor = vec![false;n];
    let mut ver = vec![false;n];
    let mut diag1 = vec![false;2*n+1];
    let mut diag2 = vec![false;2*n+1];

    let mut res = true;
    for i in 0..n {
        let x = scan.token::<i32>().unwrap();
        let y = scan.token::<i32>().unwrap();
        if hor[x as usize] { res = false; break };
        if ver[y as usize] { res = false; break };
        if diag1[(x+y) as usize] { res = false; break };
        if diag2[(x-y+n as i32) as usize] { res = false; break };
        hor[x as usize] = true;
        ver[y as usize] = true;
        diag1[(x+y) as usize] = true;
        diag2[(x-y+n as i32) as usize] = true;
    }
    if res { println!("CORRECT") }
    else { println!("INCORRECT") }
}
