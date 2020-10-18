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

fn to_universal(months: &Vec<usize>, month: usize, day: usize) -> usize {
    (months[month - 1] + day)
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let day = scan.token::<usize>().unwrap();
    let month = scan.token::<String>().unwrap();
    let month = month.trim();
    let month = match month {
        "JAN" => 1,
        "FEB" => 2,
        "MAR" => 3,
        "APR" => 4,
        "MAY" => 5,
        "JUN" => 6,
        "JUL" => 7,
        "AUG" => 8,
        "SEP" => 9,
        "OCT" => 10,
        "NOV" => 11,
        "DEC" => 12,
        _ => 0
    };
    let first_day = scan.line().unwrap();
    let first_day = match &first_day[..] {
        "SUN" => 0,
        "MON" => 1,
        "TUE" => 2,
        "WED" => 3,
        "THU" => 4,
        "FRI" => 5,
        "SAT" => 6,
        _ => 0,
    };
    let mut months = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for i in 1..months.len() { months[i] = months[i] + months[i-1]; };
    let mut leap_months = vec![0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for i in 1..leap_months.len() { leap_months[i] = leap_months[i] + leap_months[i-1]; };
    dbg!(&month, &day, &first_day);

    let cur_day = ((to_universal(&months, month, day) - 1) + first_day) % 7;
    let cur_leap_day = ((to_universal(&leap_months, month, day) - 1) + first_day) % 7;
    match (cur_day, cur_leap_day) {
        (5, 5) => writeln!(out, "TGIF"),
        (5, _) | (_, 5) => writeln!(out, "not sure"),
        _ => writeln!(out, ":(")
    };
}
