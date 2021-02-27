use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let l:Vec<char> = scan.line().unwrap().chars().collect();
    let mut pairs: HashMap<usize, usize> = HashMap::new();
    let mut opens = Vec::new();
    let mut bracket_count = 0;
    for i in 0..l.len() {
        if l[i] == '(' {
            opens.push(i);
            bracket_count += 1;
        }
        if l[i] == ')' {
            let open = opens.pop().unwrap();
            pairs.insert(open, i);
        }
    }
    let mut res: HashSet<String> = HashSet::new();
    for i in 0..2usize.pow(bracket_count as u32) - 1 {
        let mut pos_open = 0;
        let mut cur_res = Vec::new();
        let mut to_remove_closes = Vec::new();
        for j in 0..l.len() {
            if l[j] == '(' && (i & (1 << pos_open) == 0) {
                to_remove_closes.push(pairs.get(&j).unwrap());
            } else if l[j] == ')' && !to_remove_closes.is_empty() 
                && **to_remove_closes.last().unwrap() == j {
                to_remove_closes.pop();
            } else {
                cur_res.push(l[j]);
            }
            if l[j] == '(' {
                pos_open += 1;
            }
        }
        res.insert(cur_res.iter().collect());
    }
    let mut res: Vec<String> = res.into_iter().collect();
    res.sort();
    writeln!(out, "{}", res.join("\n")).unwrap();
}
