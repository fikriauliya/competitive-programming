use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashSet;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn time_to_long(time: &str) -> i64 {
    let mut it = time.split(":");
    let h = it.next().unwrap().parse::<i32>().unwrap();
    let m = it.next().unwrap().parse::<i32>().unwrap();
    (h * 60 + m) as i64
}

fn long_to_time(time: i64) -> String {
    let m = time % 60;
    let h = (time % (60 * 24)) / 60;
    let d = (time / (60 * 24)) % 7;
    let days = vec!["Saturday", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    format!("{}\n{:02}:{:02}", days[d as usize], h, m)
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut st1 = time_to_long(&scan.line().unwrap());
    let mut st2 = time_to_long(&scan.line().unwrap());
    let int1 = time_to_long(&scan.line().unwrap());
    let int2 = time_to_long(&scan.line().unwrap());
    let mut pairs = HashSet::new();

    dbg!(&st1, &int1, &st2, &int2);
    let mut never = false;
    while st1 != st2 {
        if pairs.contains(&(st1-st2)) { never = true; break; }
        pairs.insert(st1-st2);
        if st1 < st2 { st1 += int1 }
        else { st2 += int2 }
    }
    if never {
        println!("Never");
    } else {
        println!("{}", long_to_time(st1));
    }
}
