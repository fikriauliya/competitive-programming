use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 { return b; }
    return gcd(b % a, a);
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let n = scan.token::<usize>().unwrap();
        let q = scan.token::<usize>().unwrap();
        let mut roads = HashMap::new();
        let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n-1 {
            let x = scan.token::<usize>().unwrap();
            let y = scan.token::<usize>().unwrap();
            let l = scan.token::<usize>().unwrap();
            let a = scan.token::<usize>().unwrap();
            roads.insert((x,y), (l, a));
            roads.insert((y,x), (l, a));
            match edges.get_mut(&x) {
                Some(others) => {others.push(y)}
                None => {
                    edges.insert(x, vec![y]);
                }
            }
            match edges.get_mut(&y) {
                Some(others) => {others.push(x)}
                None => {
                    edges.insert(y, vec![x]);
                }
            }
        }
        let mut parents:HashMap<usize, usize> = HashMap::new();
        let mut queues: Vec<(usize, usize)> = Vec::new();
        for to in edges.get(&1).unwrap() {
            queues.push((*to, 1));
        }

        parents.insert(1, 0);
        loop {
            if queues.len() == 0 { break;}
            let cur = queues.pop().unwrap();
            parents.insert(cur.0, cur.1);

            for to in edges.get(&(cur.0)).unwrap() {
                if parents.get(&to).is_none() {
                    queues.push((*to, cur.0));
                }
            }
        }

        write!(out, "Case #{}: ", tt+1 ).unwrap();
        for i in 0..q {
            let mut from = scan.token::<usize>().unwrap();
            let w = scan.token::<usize>().unwrap();
            let mut costs = Vec::new();
            while parents.get(&from).unwrap() != &0 {
                let parent = parents.get(&from).unwrap();
                let (l, a) = roads.get(&(from, *parent)).unwrap();
                if w >= *l {
                    costs.push(a);
                }
                from = *parent;
            }
            let mut res = 0;
            if costs.len() > 0 {
                res = *costs[0];
                for i in 1..costs.len() {
                    res = gcd(*costs[i], res);
                }
            }
            write!(out, "{}", res).unwrap();
            if i < q - 1 {
                write!(out, " ").unwrap();
            }
        }
        writeln!(out).unwrap();
        // dbg!(&parents);
    }
}
