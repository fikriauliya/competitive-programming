use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap, VecDeque};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(x: isize, y: isize, begin: char, end: Option<char>, delta: isize) -> isize {
    if delta < 0 { return 0; }

    return match (begin, end) {
        ('C', Some('J')) | ('J', Some('C')) => {
            let ab = delta / 2;
            let c = if begin == 'C' { x } else { y };
            (ab * (x + y)).min(0) + c
        },
        ('C', Some('C')) | ('J', Some('J')) => {
            let ab = (delta + 1) / 2;
            (ab * (x + y)).min(0)
        },
        (_, None) => {
            let res1 = rec(x, y, begin, Some('C'), delta - 1);
            let res2 = rec(x, y, begin, Some('J'), delta - 1);
            res1.min(res2)
        }
        ('?', end) => {
            let res1 = rec(x, y, 'J', end, delta);
            let res2 = rec(x, y, 'C', end, delta);
            res1.min(res2)
        },
        _ => unreachable!() 
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let x = scan.token::<isize>().unwrap();
        let y = scan.token::<isize>().unwrap();
        let s = scan.token::<String>().unwrap();
        let s:Vec<char> = s.chars().collect();
        let mut i = 0;

        let mut res = 0;
        while i < s.len() {
            let mut next = None;
            for j in i+1..s.len() {
                if s[j] != '?' { 
                    next = Some(j);
                    break;
                }
            }
            let inc = match next {
                None => {
                    rec(x, y, s[i], None, (s.len() - i - 1) as isize)
                },
                Some(j) => {
                    rec(x, y, s[i], Some(s[j]), (j - i - 1) as isize)
                }
            };
            // dbg!(&s, i, s[i], next, inc);
            res += inc;
            if next.is_none() { break; }
            i = next.unwrap();
        }
        writeln!(out, "Case #{}: {}", tt+1, res).unwrap();
    }
}
