use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Debug)]
enum Direction {
    LEFT, RIGHT, UP, DOWN
}
fn rec(visited: &mut HashSet<isize>, need_move: (usize, usize), v: &mut Vec<Vec<isize>>) -> 
    Option<Vec<(usize, (usize, usize), (usize, usize), Direction)>> {
    if v[need_move.0][need_move.1] == -1 { return Some(vec![]) };
    if v[need_move.0][need_move.1] == -2 { return None };
    if visited.contains(&v[need_move.0][need_move.1]) { return None };

    let mut nexts: Vec<((usize, usize), (usize, usize), Direction)> = Vec::new();
    for i in vec![-1isize, 1] {
        if (0..v.len()).contains(&((need_move.0 as isize + i) as usize)) {
            if v[(need_move.0 as isize + i) as usize][need_move.1] == v[need_move.0][need_move.1] {
                if (0..v.len()).contains(&((need_move.0 as isize + i*2) as usize)) {
                    nexts.push((((need_move.0 as isize + i*2) as usize, need_move.1),
                                ((need_move.0 as isize + i) as usize, need_move.1),
                                if i*2>0 { Direction::DOWN} else { Direction::UP}));
                }
                if (0..v.len()).contains(&((need_move.0 as isize - i) as usize)) {
                    nexts.push((((need_move.0 as isize - i) as usize, need_move.1), 
                                ((need_move.0 as isize + i) as usize, need_move.1),
                                if -i>0 { Direction::DOWN} else {Direction::UP}));
                }
            }
        }
        if (0..v[0].len()).contains(&((need_move.1 as isize + i) as usize)) {
            if v[need_move.0][(need_move.1 as isize + i) as usize] == v[need_move.0][need_move.1] {
                if (0..v[0].len()).contains(&((need_move.1 as isize + i*2) as usize)) {
                    nexts.push(((need_move.0, (need_move.1 as isize + i*2) as usize), 
                                (need_move.0, (need_move.1 as isize + i) as usize),
                                if i*2>0 { Direction::RIGHT} else {Direction::LEFT}));
                }
                if (0..v[0].len()).contains(&((need_move.1 as isize - i) as usize)) {
                    nexts.push(((need_move.0, (need_move.1 as isize - i) as usize),
                                (need_move.0, (need_move.1 as isize + i) as usize),
                                if -i>0 { Direction::RIGHT } else {Direction::LEFT}));
                }
            }
        }
    }
    // dbg!(&nexts);
    let mut res = None;
    visited.insert(v[need_move.0][need_move.1]);
    let mut dir = Direction::LEFT;
    let mut another = (0, 0);

    for next in nexts {
        res = match (res, rec(visited, next.0, v)) {
            (None, Some(rec_res)) => { dir = next.2; another = next.1; Some(rec_res) }
            (Some(res), None) => Some(res),
            (Some(res), Some(rec_res)) => {
                if res.len() < rec_res.len() { Some(res) }
                else if res.len() > rec_res.len() { dir = next.2; another = next.1; Some(rec_res) }
                else {
                    // let a = res.iter().map(|s| s.0.to_string()).collect::<Vec<String>>().join(" ");
                    // let b = rec_res.iter().map(|s| s.0.to_string()).collect::<Vec<String>>().join(" ");
                    // if a > b { Some(rec_res) }
                    // else { Some(res) }
                    let mut winner = 0;
                    for i in 0..res.len() {
                        if res[i].0 == rec_res[i].0 { continue; }
                        // if res[i].0.to_string() > rec_res[i].0.to_string() { winner = 1; break; }
                        if res[i].0 > rec_res[i].0 { winner = 1; break; }
                        else { winner = 0; break; }
                    }
                    if winner == 0 { Some(res) }
                    else { dir = next.2; another = next.1; Some(rec_res) }
                }
            }
            (None, None) => None,
        };
    }
    visited.remove(&v[need_move.0][need_move.1]);
    if res.is_some() {
        res.as_mut().unwrap().push((v[need_move.0][need_move.1] as usize, need_move, another, dir));
    }
    res
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let m = scan.token::<usize>().unwrap();
    let n = scan.token::<usize>().unwrap();
    let mut v = Vec::with_capacity(m);

    for _ in 0..m {
        let mut l = Vec::with_capacity(n);
        for _ in 0..n {
            let x = scan.token::<isize>().unwrap();
            l.push(x);
        }
        v.push(l);
    }
    let r = scan.token::<usize>().unwrap() - 1;
    let c = scan.token::<usize>().unwrap() - 1;
    let res = rec(&mut HashSet::new(), (r, c), &mut v);
    match res {
        Some(res) => {
            // dbg!(&res);
            let last = res.last().unwrap();
            let a = last.1;
            let b = last.2;
            let impossible = match last.3 {
                Direction::UP => (a.0-1, a.1) == (r, c) || (b.0-1, b.1) == (r, c),
                Direction::DOWN => (a.0+1, a.1) == (r, c) || (b.0+1, b.1) == (r, c),
                Direction::LEFT => (a.0, a.1-1) == (r, c) || (b.0, b.1-1) == (r, c),
                Direction::RIGHT => (a.0, a.1+1) == (r, c) || (b.0, b.1+1) == (r, c) 
            };
            if impossible { writeln!(out, "impossible").unwrap(); }
            else { writeln!(out, "{}", res.iter().map(|s| format!("{}", s.0)).collect::<Vec<String>>().join(" ")); }
        },
        None => { writeln!(out, "impossible").unwrap(); }
    };
}
