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

fn digit(val: usize, pos: usize) -> usize {
    (val % 10usize.pow(3u32-pos as u32)) / 10usize.pow(2u32-pos as u32)
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut rams = vec![0usize;1000];
    let mut regs = vec![0usize;10];

    let mut i = 0;
    let mut res = 0;
    loop {
        if let Some(line) = scan.token::<usize>() {
            rams[i] = line;
            i += 1;
        } else { break; }
    }

    i = 0;
    loop {
        let val = rams[i];
        res += 1;
        match (digit(val, 0), digit(val, 1), digit(val, 2)) {
            (1, _, _) => break,
            (2, d, n) => regs[d] = n,
            (3, d, n) => regs[d] = (regs[d] + n) % 1000,
            (4, d, n) => regs[d] = (regs[d] * n) % 1000,
            (5, d, s) => regs[d] = regs[s],
            (6, d, s) => regs[d] = (regs[d] + regs[s]) % 1000,
            (7, d, s) => regs[d] = (regs[d] * regs[s]) % 1000,
            (8, d, a) => regs[d] = rams[regs[a]],
            (9, s, a) => rams[regs[a]] = regs[s],
            (0, d, s) => if regs[s] != 0 { i = regs[d]; continue; } else { }, 
            _ => continue,
        }
        i += 1;
    }
    println!("{}", res);
}
