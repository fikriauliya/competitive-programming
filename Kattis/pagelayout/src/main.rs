use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Paper {
    w: u32,
    h: u32,
    x: u32,
    y: u32,
}

impl Paper {
    fn intersect(&self, other: Paper) -> bool {
        let (x1, y1, x2, y2) = (self.x, self.y, self.x + self.w, self.y + self.h);
        let (ox1, oy1, ox2, oy2) = (other.x, other.y, other.x + other.w, other.y + other.h);
        !((y1 <= oy1 && y2 <= oy1) || (y1 >= oy2 && y2 >= oy2)) && 
            !((x1 <= ox1 && x2 <= ox1) || (x1 >= ox2 && x2 >= ox2))
    }
    fn area(&self) -> u32 { self.w * self.h }
}

fn ls_one(x: u32) -> u32 {
    ((x as i32) & -(x as i32)) as u32
}

fn rec(rems: u32, papers: &Vec<Paper>, no_intersects: &HashMap<u32, u32>, area: u32) -> u32 {
    if rems == 0 { return area; }

    let mut res = 0;
    let mut next_rems = rems;
    loop {
        let bit = ls_one(next_rems);
        if bit == 0 { break; }
        let i = (bit as f32).log2() as u32;

        let rems = no_intersects[&i] & next_rems;
        res = res.max(rec(rems, papers, no_intersects, area + papers[i as usize].area()));
        next_rems = next_rems & !bit;
        if rems == next_rems { break; } //with or without this has the same reminders, prefer with
    }
    res
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    loop {
        let n = scan.token::<u32>().unwrap();
        if n == 0 { return; }
        let mut papers = Vec::new();
        for _ in 0..n {
            let w = scan.token::<u32>().unwrap();
            let h = scan.token::<u32>().unwrap();
            let x = scan.token::<u32>().unwrap();
            let y = scan.token::<u32>().unwrap();
            papers.push(Paper{w,h,x,y});
        }
        let mut no_intersects: HashMap<u32, u32> = HashMap::new();
        for i in 0..n {
            no_intersects.insert(i, 0);
            for j in 0..n {
                if !papers[i as usize].intersect(papers[j as usize]) {
                    match no_intersects.get_mut(&i) {
                        Some(v) => {*v |= 1 << j; },
                        None => unreachable!()
                    }
                }
            }
        }
        writeln!(out, "{}", rec((1 << n) - 1, &papers, &no_intersects, 0));
    }
}
