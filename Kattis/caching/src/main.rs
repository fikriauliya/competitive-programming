use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::{HashSet, HashMap, VecDeque, BTreeSet};
use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct CacheEntry {
    value: usize,
    next_access: Option<usize>,
}

impl PartialOrd for CacheEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for CacheEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.next_access, other.next_access) {
            (Some(s), Some(o)) => o.cmp(&s),
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            _ => Ordering::Equal,
        }.then_with(|| self.value.cmp(&other.value))
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let c = scan.token::<usize>().unwrap();
    let _ = scan.token::<usize>().unwrap();
    let a = scan.token::<usize>().unwrap();
    let mut access = Vec::new();
    let mut next_access: HashMap<usize, VecDeque<usize>> = HashMap::new();
    for i in 0..a {
        let el = scan.token::<usize>().unwrap();
        access.push(el);
        match next_access.get_mut(&el) {
            Some(na) => na.push_back(i),
            None => {
                let na = VecDeque::new();
                next_access.insert(el, na);
            }
        }
    }

    let mut cache_priority: BTreeSet<CacheEntry> = BTreeSet::new();
    let mut cache_content: HashMap<usize, CacheEntry> = HashMap::new();
    let mut res = 0;
    for i in 0..a {
        let na = next_access.get_mut(&access[i]).unwrap().pop_front();
        if let Some(entry) = cache_content.get_mut(&access[i]) {
            cache_priority.remove(&entry);
            entry.next_access = na;
            cache_priority.insert(*entry);
        } else {
            if cache_content.len() == c {
                let rem = cache_priority.iter().next().unwrap().clone();
                cache_content.remove(&rem.value);
                cache_priority.remove(&rem);
            }
            res += 1;
            let entry = CacheEntry{
                value: access[i],
                next_access: na,
            };
            cache_content.insert(access[i], entry);
            cache_priority.insert(entry);
        }
    }
    writeln!(out, "{}", res);
}
