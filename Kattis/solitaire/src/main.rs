use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn rec(rems: &mut HashSet<(isize, isize)>, matrix: &Vec<Vec<char>>, moves: usize) -> (usize, usize) {
    let mut move_and_next = HashMap::new();
    move_and_next.insert((-1, 0), (-2, 0));
    move_and_next.insert((1, 0), (2, 0));
    move_and_next.insert((0, -1), (0, -2));
    move_and_next.insert((0, 1), (0, 2));

    let mut can_move = false;
    let mut res = (std::usize::MAX, std::usize::MAX);
    for (i, j) in rems.clone().iter() {
        for (m, n) in move_and_next.iter() {
            if rems.contains(&(*i+m.0, *j+m.1)) && 
                (0..matrix.len() as isize).contains(&(*i+n.0)) &&
                (0..matrix[0].len() as isize).contains(&(*j+n.1)) &&
                !rems.contains(&(*i+n.0, *j+n.1)) && 
                matrix[(*i+n.0) as usize][(*j+n.1) as usize] != '#' {
                    can_move = true;

                    rems.remove(&(*i, *j));
                    rems.remove(&(*i+m.0, *j+m.1));
                    rems.insert((*i+n.0, *j+n.1));
                    res = res.min(rec(rems, matrix, moves + 1));
                    rems.remove(&(*i+n.0, *j+n.1));
                    rems.insert((*i+m.0, *j+m.1));
                    rems.insert((*i, *j));
            }
        }
    }
    if !can_move { return (rems.len(), moves); }
    return res;
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for _ in 0..t {
        let mut m = Vec::new();
        let mut rems = HashSet::new();
        let mut i = 0;
        loop {
            let line = scan.line();
            if line.is_none() { break; }
            let line = line.unwrap();
            if line == "" { break; }

            let mut cur = Vec::new();
            for (j, c) in line.char_indices() {
                cur.push(c);
                if c == 'o' {
                    rems.insert((i as isize, j as isize));
                }
            }
            m.push(cur);
            i += 1;
        }
        let res = rec(&mut rems, &m, 0);
        writeln!(out, "{} {}", res.0, res.1);
    }
}
