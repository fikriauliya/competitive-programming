use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, VecDeque, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let prev = scan.line().unwrap();
    let prev_last = prev.chars().next_back().unwrap();
    let n = scan.token::<usize>().unwrap();
    let mut st_to_ends: HashMap<char, VecDeque<(String, char)>> = HashMap::new();
    for _ in 0..n {
        let unused = scan.line().unwrap().to_string();
        let chars: Vec<char> = unused.chars().collect();
        match st_to_ends.get_mut(&chars[0]) {
            Some(ends) => {
                ends.push_back((unused, chars[chars.len() - 1]));
            },
            None => { 
                let mut ends = VecDeque::new();
                ends.push_back((unused, chars[chars.len() - 1]));
                st_to_ends.insert(chars[0], ends); 
            }
        }
    }
    match st_to_ends.get(&prev_last) {
        Some(ends) => {
            for end in ends {
                if !st_to_ends.contains_key(&end.1) {
                    writeln!(out, "{}!", end.0);
                    return;
                }
                if end.1 == prev_last && ends.len() == 1 {
                    writeln!(out, "{}!", end.0);
                    return;
                }
            }
            writeln!(out, "{}", ends.iter().next().unwrap().0);
        },
        None => { writeln!(out, "?"); }
    }
}
