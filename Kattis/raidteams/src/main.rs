use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::binary_heap::BinaryHeap;
use std::cmp::{Ordering,Reverse};
use std::collections::HashSet;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Debug, Clone, Eq, PartialEq)]
struct Skill {
    value: usize,
    name: String
}

impl PartialOrd for Skill {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Skill {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value).then_with(|| self.name.cmp(&other.name))
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut p1 = BinaryHeap::new();
    let mut p2 = BinaryHeap::new();
    let mut p3 = BinaryHeap::new();
    let mut available = HashSet::new();
    for _ in 0..n {
        let player = scan.token::<String>().unwrap();
        let s1 = scan.token::<usize>().unwrap();
        let s2 = scan.token::<usize>().unwrap();
        let s3 = scan.token::<usize>().unwrap();
        p1.push(Reverse(Skill{ value: s1, name: player.clone() }));
        p2.push(Reverse(Skill{ value: s2, name: player.clone() }));
        p3.push(Reverse(Skill{ value: s3, name: player.clone() }));
        available.insert(player.clone());
    }

    loop {
        let mut team = Vec::new();
        let ps = vec![&mut p1, &mut p2, &mut p3];
        for p in ps {
            while let Some(m) = p.pop() {
                if available.contains(&m.0.name) {
                    team.push(m.0.name.clone());
                    available.remove(&m.0.name);
                    break;
                }
            }
        }
        team.sort();
        match (team.pop(), team.pop(), team.pop()) {
            (Some(m3), Some(m2), Some(m1)) => {
                writeln!(out, "{} {} {}", m1, m2, m3);
            },
            _ => break
        }
    }
}
