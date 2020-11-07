use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

struct Party {
    vote_count: usize,
    choices: VecDeque<usize>
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let c = scan.token::<usize>().unwrap();
    let p = scan.token::<usize>().unwrap();
    let mut candidates = HashSet::new();
    let mut parties = HashMap::new();
    for i in 1..=c { candidates.insert(i); }
    let mut total_votes = 0;
    for i in 0..p {
        let vote_count = scan.token::<usize>().unwrap();
        total_votes += vote_count;
        
        let mut choices = VecDeque::new();
        for _ in 0..c {
            let choice = scan.token::<usize>().unwrap();
            choices.push_back(choice);
        }
        parties.insert(i, Party{
            vote_count,
            choices
        });
    }
    let quorum = total_votes / 2 + 1;

    loop {
        let mut votes = HashMap::new();
        for i in 0..p {
            let party = parties.get_mut(&i).unwrap();
            let mut choice = *party.choices.front().unwrap();
            while !candidates.contains(&choice) {
                party.choices.pop_front().unwrap();
                choice = *party.choices.front().unwrap();
            }
            let vote_count = party.vote_count;
            match votes.get_mut(&choice) {
                Some(c) => *c = *c + vote_count,
                None => { votes.insert(choice, vote_count); },
            };
        }
        let it = votes.iter();
        let max_votes = it.max_by_key(|v| v.1).unwrap();
        let min_votes = candidates
            .iter().map(|cand| (cand, votes.get(cand).unwrap_or(&0)))
            .min_by(|x, y| {
                x.1.cmp(&y.1).then_with(|| x.0.cmp(&y.0))
            })
            .unwrap();

        if *max_votes.1 >= quorum {
            println!("{}", max_votes.0);
            break;
        } 
        candidates.remove(&min_votes.0.clone());
    }
}
