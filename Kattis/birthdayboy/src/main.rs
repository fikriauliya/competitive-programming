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

fn from_universal(months: &Vec<usize>, universal: usize) -> String {
    let mut ctr = 1;
    while months[ctr] < universal { ctr += 1; };
    let day = format!("{:02}", universal - months[ctr-1]);
    let month = format!("{:02}", ctr);
    format!("{}-{}", month, day)
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut months = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for i in 1..months.len() {
        months[i] = months[i] + months[i-1];
    };
    let mut dates = vec![];
    for _ in 0..n {
        let (_, date) = (scan.token::<String>().unwrap(), scan.token::<String>().unwrap());
        let splits = date.split("-").map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let (month, day) = (splits[0], splits[1]);
        let universal = to_universal(&months, month, day);
        dates.push(universal);
    }
    dates.sort();

    let mut gaps = vec![];
    for i in 1..dates.len() {
        gaps.push(dates[i] - dates[i-1]);
    }

    let total_days: usize = *months.last().unwrap();
    gaps.push(total_days - dates[dates.len() -1] + dates[0]);

    let max_gap = gaps.iter().max().unwrap();
    let mut proposed_dates: Vec<usize> = gaps.iter()
        .enumerate()
        .filter(|g| *g.1 == *max_gap)
        .map(|g| (g.0 + 1) % dates.len())
        .map(|d| match dates[d] - 1 {
            0 => to_universal(&months, 12, 31),
            s => s
        })
        .collect();

    proposed_dates.sort();
    let today = to_universal(&months, 10, 27);
    let nearest = proposed_dates.iter().filter(|d| **d > today).next();
    if let Some(nearest) = nearest {
        writeln!(out, "{}", from_universal(&months, *nearest));
    } else {
        writeln!(out, "{}", from_universal(&months, proposed_dates[0]));
    }
}
