use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

// fn rand(random: &mut u32) {
//     *random ^= *random << 13;
//     *random ^= *random >> 17;
//     *random ^= *random << 5;
// }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap(); 
    let p = scan.token::<usize>().unwrap();
    // let mut random = 34 as u32;
    // rand(&mut random);
    for tt in 0..t {
        let mut max = 0;
        let mut res = 0;
        for i in 0..100 {
            let line = scan.line().unwrap();
            let scores: Vec<u32> = line.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect();
            let correct = scores.into_iter().filter(|s| *s == 1).count();
            if max < correct {
                res = i;
                max = correct;
            }
            // dbg!(&i, &correct);
        }
        // dbg!(max);
        writeln!(out, "Case #{}: {}", tt+1, res + 1).unwrap();
        // for score in scores {
        //     rand(&mut random);
        //     let r = ((random % 1200000) as f32 - 600000f32)/100000f32;
        //     dbg!(r);
        //     // if score == 1 {
                
        //     // }
        // }
    }
}
