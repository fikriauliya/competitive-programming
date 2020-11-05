use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::BTreeMap;
use std::ops::Bound;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Debug)]
struct Node {
    height: u32,
    left: Option<u32>,
    right: Option<u32>,
}

enum Position { Left, Right}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut tree: BTreeMap<u32, Node>  = BTreeMap::new();
    let mut counter: u64 = 0;
    for _ in 0..n {
        let item = scan.token::<u32>().unwrap();
        if tree.is_empty() {
            tree.insert(item, Node {
                height: 0,
                left: None,
                right: None
            });
        } else {
            let lower = tree.range((Bound::Unbounded, Bound::Excluded(item))).next_back();
            let higher = tree.range((Bound::Excluded(item), Bound::Unbounded)).next();
            let (idx, pos) = match (lower, higher) {
                (Some(l), None) => (*l.0, Position::Left),
                (None, Some(h)) => (*h.0, Position::Right),
                (Some(l), Some(h)) => match (l.1.right.as_ref(), h.1.left.as_ref()) {
                    (Some(_), None) => (*h.0, Position::Right),
                    (None, Some(_)) => (*l.0, Position::Left),
                    _ => panic!("Never happens")
                },
                (None, None) => panic!("Never happens. Should go to empty branch"),
            };

            let parent = tree.get_mut(&idx).unwrap();
            match pos {
                Position::Left => parent.right = Some(item),
                Position::Right => parent.left = Some(item),
            };
            let height = parent.height + 1;
            tree.insert(item, Node {
                height,
                left: None,
                right: None
            });
            counter += height as u64;
        }
        writeln!(out, "{}", counter);
    }
}
