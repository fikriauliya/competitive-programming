#[allow(unused_imports)]
use std::cmp::{Ordering, Reverse};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList};
#[allow(unused_imports)]
use std::convert::TryFrom;
use std::io;
use std::str;

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }
    pub fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok().expect("Failed parse"));
            }
            self.buf_str.clear();
            let len = self
                .reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            if len == 0 {
                return None;
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
    pub fn line(&mut self) -> Option<String> {
        let mut input = String::new();
        let len = self.reader.read_line(&mut input).expect("Failed read");
        match len {
            0 => None,
            _ => {
                Self::trim_newline(&mut input);
                Some(input)
            }
        }
    }
}
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token().unwrap();
    'case: for tt in 1..=t {
        let origin = scan.line().unwrap();
        let origin = origin.chars().collect::<Vec<char>>();
        let mut res = origin.clone();
        if res.len() == 1 {
            writeln!(out, "Case #{}: IMPOSSIBLE", tt).unwrap();
            continue 'case;
        }

        // dbg!(&res);
        'outer: for i in 0..origin.len() {
            // dbg!(i, &res);
            for j in 0..origin.len() {
                if j == i {
                    continue;
                }
                if res[i] != origin[i] {
                    continue 'outer;
                }
                if res[i] != res[j] && res[i] != origin[j] && origin[i] != res[j] {
                    let tmp = res[i];
                    res[i] = res[j];
                    res[j] = tmp;
                    continue 'outer;
                }
            }
            writeln!(out, "Case #{}: IMPOSSIBLE", tt).unwrap();
            continue 'case;
        }
        writeln!(out, "Case #{}: {}", tt, res.iter().collect::<String>());
    }
}
