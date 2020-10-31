use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum DataStructure { Queue, PriorityQueue, Stack }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    loop {
        let n = scan.token::<usize>();
        if n.is_none() { break; }
        let n = n.unwrap();
        let mut queue = VecDeque::new();
        let mut stack = Vec::new();
        let mut pqueue = BinaryHeap::new();

        let mut candidates = HashSet::new();
        candidates.insert(DataStructure::Stack);
        candidates.insert(DataStructure::Queue);
        candidates.insert(DataStructure::PriorityQueue);

        let mut res = None;
        for _ in 0..n {
            let command = scan.token::<usize>().unwrap();
            if command == 1 {
                let param = scan.token::<usize>().unwrap();
                queue.push_back(param);
                stack.push(param);
                pqueue.push(param);
            } else {
                let output = scan.token::<usize>().unwrap();
                let q = queue.pop_front();
                let s = stack.pop();
                let pq = pqueue.pop();

                let mut cur_candidates = HashSet::new();
                match (q, s, pq) {
                    (Some(q), Some(s), Some(pq)) => {
                        if q == output { cur_candidates.insert(DataStructure::Queue); }
                        if s == output { cur_candidates.insert(DataStructure::Stack); }
                        if pq == output { cur_candidates.insert(DataStructure::PriorityQueue); }
                        ()
                    },
                    _ => {
                        res = Some("impossible");
                        ()
                    }
                }

                candidates = candidates.intersection(&cur_candidates).cloned().collect();
            }
        }

        res = if let None = res {
            if candidates.len() > 1 { Some("not sure") }
            else if candidates.len() == 0 { Some("impossible") }
            else {
                let el = candidates.iter().next().unwrap();
                if el == &DataStructure::Queue { Some("queue") }
                else if el == &DataStructure::Stack { Some("stack") }
                else { Some("priority queue") }
            }
        } else { res };

        println!("{}", res.unwrap());
    }
}
