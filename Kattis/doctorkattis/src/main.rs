use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cmp::Ordering;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Debug, Eq, PartialEq, Clone)]
struct Cat {
    level: u8,
    order: usize,
}

impl PartialOrd for Cat {
    fn partial_cmp(&self, other: &Cat) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Cat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level
            .cmp(&other.level)
            .then_with(|| other.order.cmp(&self.order))
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut cat_to_detall: HashMap<String, Cat> = HashMap::new();
    let mut priority: BTreeMap<Cat, String> = BTreeMap::new();

    for i in 0..n {
        let cmd = scan.token::<u8>().unwrap();
        match cmd {
            0 => {
                let name = scan.token::<String>().unwrap();
                let level = scan.token::<u8>().unwrap();
                let cat = Cat{ order: i, level };
                cat_to_detall.insert(name.clone(), cat.clone());
                priority.insert(cat, name.clone());
            },
            1 => {
                let name = scan.token::<String>().unwrap();
                let update_level = scan.token::<u8>().unwrap();
                match cat_to_detall.get_mut(&name[..]) {
                    Some(c) => {
                        priority.remove(c);
                        c.level += update_level;
                        priority.insert(c.clone(), name.clone());
                    },
                    None => panic!(),
                }
            },
            2 => {
                let name = scan.token::<String>().unwrap();
                let cat = cat_to_detall.remove(&name[..]).unwrap();
                priority.remove(&cat);
            },
            3 => {
                let last = priority.iter().next_back();
                if let Some(c) = last {
                    writeln!(out, "{}", c.1);
                } else {
                    writeln!(out, "The clinic is empty");
                }
            },
            _ => panic!()
        }
    }
}
