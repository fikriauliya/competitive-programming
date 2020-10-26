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
    let r = scan.token::<usize>().unwrap();
    let c = scan.token::<usize>().unwrap();
    let mut m = vec![vec![Some(0); c+2]; r+2];
    let mut total = 0;
    for i in 0..r {
        let line = scan.line().unwrap();
        let chars = line.chars();
        for (j, c) in chars.enumerate() {
            if c != '.' { m[i+1][j+1] = None; total += 1; }
            else { m[i+1][j+1] = Some(0); }
        }
    }

    let mut cur = 0;
    let mut layer = 1;
    while cur < total {
        for i in 1..r+1 {
            for j in 1..c+1 {
                if let None = m[i][j] {
                    let surroundings = vec![m[i][j-1], m[i-1][j], m[i+1][j], m[i][j+1]];
                    let surroundings = surroundings.iter()
                        .filter(|it| if let Some(_) = it { true } else { false })
                        .map(|it| it.unwrap());
                    if let Some(min) = surroundings.min() {
                        if min + 1 == layer {
                            m[i][j] = Some(min + 1);
                            cur += 1;
                        }
                    }
                }
            }
        }
        layer += 1;
    }
    for i in 1..r+1 {
        for j in 1..c+1 {
            let cur = m[i][j].unwrap();
            let output = if cur == 0 { "...".to_string() } 
            else { format!("..{}", cur) };

            let output = if layer > 10 {
                &output[output.len() - 3..output.len()]
            } else {
                &output[output.len() - 2..output.len()]
            };

            print!("{}", output)
        }
        println!();
    }
}
