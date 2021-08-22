use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn p(cand: &Vec<usize>) {
    let v:Vec<String> = cand.into_iter().map(|s| s.to_string()).collect();
    println!("{}", v.join(" "));
}

fn find<R:io::BufRead>(v: &mut Vec<usize>, low: isize, high: isize, num: usize, scan: &mut UnsafeScanner<R>) {
    // dbg!(&v, low, high, num);
    if high + 1 == low { 
        v.insert(low as usize, num); 
        return;
    }
    if low + 1 == high || low == high {
        let mut high = high as usize;
        let mut low = low as usize;
        if low == high {
            if low == 0 {
                high = low + 1;
            } else {
                low = low - 1;
            }
        }
        p(&vec![v[low],v[high],num]); 
        let median = scan.token::<usize>().unwrap();
        if median == num {
            v.insert(high, num);
        } else if median == v[low] {
            v.insert(low, num);
        } else {
            v.insert(high + 1, num);
        }
        return;
    }
    if low > high { return; }

    let delta = (high - low) / 3;
    let t1 = low + delta;
    let t2 = high - delta;
    // dbg!(delta, t1, t2);

    p(&vec![v[t1 as usize],v[t2 as usize],num]); 
    let median = scan.token::<usize>().unwrap();
    if median == num {
        find(v, t1+1, t2-1, num, scan);
    } else if median == v[t1 as usize] {
        find(v, low, t1-1, num, scan);
    } else {
        find(v, t2+1, high, num, scan);
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    let n = scan.token::<usize>().unwrap();
    let q = scan.token::<usize>().unwrap();
    for _ in 0..t {
        let mut v = vec![0;3];
        let cand = vec![1,2,3];
        p(&cand);
        let median = scan.token::<usize>().unwrap();
        v[1] = median;
        let mut cand: Vec<usize> = cand.into_iter().filter(|c| *c!=median).collect();
        v[0] = cand.pop().unwrap();
        v[2] = cand.pop().unwrap();

        let mut num = 4;
        while num <= n {
            let high = v.len() - 1;
            find(&mut v, 0, high as isize, num, scan);
            num += 1;
        }
        p(&v);
        let judge = scan.token::<isize>().unwrap();
        if judge == -1 { return; }
    }
}
