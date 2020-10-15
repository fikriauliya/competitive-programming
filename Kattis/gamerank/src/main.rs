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
    let seqs = scan.line().unwrap();
    let mut consequetive_wins = 0;
    let mut rank = 25;
    let mut point = 0;
    let mut points = vec![0];
    for i in 0..10 { points.push(5) };
    for i in 0..5 { points.push(4) };
    for i in 0..5 { points.push(3) };
    for i in 0..5 { points.push(2) };

    for s in seqs.chars() {
        if rank == 0 { break }
        if s == 'W' { consequetive_wins += 1 }
        else { consequetive_wins = 0 }

        if s == 'L' { 
            if (1..21).contains(&rank) {
                point -= 1;
                if point == -1 {
                    if rank < 20 {
                        rank += 1;
                        point = points[rank]-1;
                    } else {
                        point = 0;
                    }
                }
            }
        }

        if s == 'W' { 
            point += 1;
            if (6..26).contains(&rank) && consequetive_wins >= 3 { point += 1 }
            if point > points[rank] {
                point -= points[rank];
                rank -= 1;
            }
        }
    }

    if rank == 0 {
        writeln!(out, "Legend");
    } else {
        writeln!(out, "{}", rank);
    }
}
