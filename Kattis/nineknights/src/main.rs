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
    let mut m = vec![vec![false;5];5];
    for i in 0..5 {
        let line = scan.line().unwrap();
        let line = line.chars();
        for (j, c) in line.enumerate() {
            if c == 'k' { m[i][j] = true; }
        }
    }
    let moves = vec![(-2, -1), (-1, -2), (2, -1), (-1, 2), 
                    (-2, 1), (1, -2), (2, 1), (1, 2)];
    let mut valid = true;
    dbg!(&m);
    'main: for i in 0..5 {
        for j in 0..5 {
            if m[i][j] {
                for mv in &moves {
                    let (dy, dx) = ((i as i32 + mv.0), (j as i32 + mv.1));
                    if (0..5).contains(&dy) && (0..5).contains(&dx) {
                        if m[dy as usize][dx as usize] {
                            valid = false; break 'main;
                        }
                    }
                }
            }
        }
    }
    if m.iter().map(|r| r.iter().filter(|v| **v).count()).sum::<usize>() 
        != 9 { println!("invalid") }
    else {
        if valid { println!("valid") }
        else { println!("invalid") }
    }
}
