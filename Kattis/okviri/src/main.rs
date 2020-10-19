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
    let line: Vec<char> = scan.line().unwrap().chars().collect();

    let u = 2;
    let l = 2;
    let r = 2;
    let d = 2;
    let m = 1;
    let n = 1;

    for i in 0..u+d+1 {
        for jj in 0..line.len() {
            for j in 0..l+r+1 {
                if jj < line.len() - 1 &&  j == l+r { continue }
                if i < u || i >= u + m || j < l || j >= n + l {
                    if (i + j) % 2 == 0 {
                        if i == 0 || i == u+d {
                            if j == 2  {
                                if (jj + 1) % 3 == 0 { print!("*"); }
                                else { print!("#"); }
                            } else { print!("."); }
                        } else {
                            if jj > 0 && j==0 && i==2 && jj % 3 == 0 { print!("*"); }
                            else if (jj + 1) % 3 == 0 { print!("*"); }
                            else { print!("#"); }
                        }
                    } else { print!("."); }
                } else { print!("{}", line[jj]); }
            }
        }
        println!();
    }
}
