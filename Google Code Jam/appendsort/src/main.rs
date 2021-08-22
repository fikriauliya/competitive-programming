use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let n = scan.token::<usize>().unwrap();
        let mut m = Vec::new();
        for i in 0..n {
            let num = scan.token::<usize>().unwrap();
            m.push(num);
        }
        let mut res = 0;
        for i in 1..n {
            let prev_dig = (m[i-1] as f64).log10() as usize + 1;
            let cur_dig = (m[i] as f64).log10() as usize + 1;
            if cur_dig == prev_dig {
                if m[i] <= m[i-1] {
                    m[i] *= 10;
                    res += 1;
                }
            } else if cur_dig < prev_dig {
                let diff_dig = prev_dig as u32 - cur_dig as u32;
                let prev_prefix = m[i-1] / 
                    (10usize.pow(diff_dig));
                if prev_prefix == m[i] {
                    let cand = m[i-1] + 1;
                    if cand/(10usize.pow(diff_dig)) == m[i] {
                        m[i] = cand;
                        res += diff_dig;
                    } else {
                        m[i] = m[i] * 10usize.pow(diff_dig) * 10;
                        res += diff_dig + 1;
                    }
                } else if prev_prefix > m[i] {
                    m[i] = m[i] * 10usize.pow(diff_dig) * 10;
                    res += diff_dig + 1;
                } else {
                    m[i] = m[i] * 10usize.pow(diff_dig);
                    res += diff_dig;
                }
            }
        }
        dbg!(m);
        writeln!(out, "Case #{}: {}", tt+1, res).unwrap();
    }
}
