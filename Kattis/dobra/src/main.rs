use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(word: &mut Vec<char>, prev_vocal: usize, prev_consonant: usize, needs_l: bool) -> usize {
    if word.len() == 0 { 
        if needs_l { return 0; }
        return 1; 
    }

    let last = word.pop().unwrap();
    let res = match last {
        '_' => {
            let v = if prev_vocal < 2 { 5 * rec(word, prev_vocal + 1, 0, needs_l) }
            else { 0 };
            let c = if prev_consonant < 2 { 
                20 * rec(word, 0, prev_consonant + 1, needs_l) + 
                    rec(word, 0, prev_consonant + 1, false)
            }
            else { 0 };
            v + c
        },
        'A'|'E'|'I'|'O'|'U' => { 
            if prev_vocal == 2 { 0 }
            else { rec(word, prev_vocal + 1, 0, needs_l) }
        },
        _ => {
            if prev_consonant == 2 { 0 }
            else { rec(word, 0, prev_consonant + 1, (last != 'L') && needs_l) }
        }
    };
    word.push(last);
    res
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let word = scan.line().unwrap();
    let mut c:Vec<char> = word.chars().collect();
    let res = rec(&mut c, 0, 0, true);
    writeln!(out, "{}", res);
}
