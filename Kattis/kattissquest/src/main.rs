use std::io;
use std::str;
use std::convert::TryFrom;
use std::cmp::Ordering;
use std::collections::{HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Debug, Eq, PartialEq, Clone)]
struct Quest {
    energy: i64,
    gold: i64,
    id: i64,
}

impl Ord for Quest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.energy.cmp(&other.energy)
            .then_with(|| self.gold.cmp(&other.gold))
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Quest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<i64>().unwrap();
    let mut q = BTreeMap::new();
    for id in 0..n {
        let command = scan.token::<String>().unwrap();
        match &command[..] {
            "add" => {
                let energy = scan.token::<i64>().unwrap();
                let gold = scan.token::<i64>().unwrap();
                let quest = Quest { energy, gold, id };
                q.insert(quest, ());
            },
            "query" => {
                let mut x = scan.token::<i64>().unwrap();
                let mut gold_received = 0;
                let search_quest = Quest {
                    energy: x,
                    gold: std::i64::MAX,
                    id
                };
                let mut to_remove = Vec::new();
                let less = q.range(..=&search_quest);
                let mut it = less.into_iter();
                while let Some(quest) = it.next_back() {
                    if quest.0.energy > x { break; }

                    gold_received += quest.0.gold; 
                    x -= quest.0.energy;

                    to_remove.push((*quest.0).clone());

                    let search_quest = if x < quest.0.energy {
                        Quest {
                            energy: x,
                            gold: std::i64::MAX,
                            id
                        }
                    } else {
                        Quest {
                            energy: quest.0.energy,
                            gold: quest.0.gold,
                            id: quest.0.id 
                        }
                    };
                    let less = q.range(..&search_quest);
                    it = less.into_iter();
                }
                for rem in to_remove {
                    q.remove(&rem);
                }
                writeln!(out, "{}", gold_received);
            },
            _ => panic!()
        }
    }
}
