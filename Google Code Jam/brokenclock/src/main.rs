use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let a = scan.token::<f64>().unwrap();
        let b = scan.token::<f64>().unwrap();
        let c = scan.token::<f64>().unwrap();
        let ticks = vec![a,b,c];
        let mut res = None;
        'outer: for h in 0..3 {
            for m in 0..3 {
                for s in 0..3 {
                    for i in 0..60 {
                        if h != m && h != s && m != s {
                            let h = (ticks[h]+(i as f64)*720e9)%432e11;
                            let s = (ticks[s]+(i as f64)*720e9)%432e11;
                            let m = (ticks[m]+(i as f64)*720e9)%432e11;

                            let nano = h;
                            if m != (nano * 12.0) % 432e11 { break; }
                            if s != (nano * 720.0) % 432e11 { break; }

                            // dbg!(&nano);
                            let hour = (nano/1e9/60.0/60.0).floor();
                            let minute = ((nano-(hour*60.0*60.0*1e9))/1e9/60.0).floor();
                            let second = ((nano-(hour*60.0*60.0*1e9)
                                           -(minute*60.0*1e9))/1e9).floor();
                            let nanosecond = nano-(hour*60.0*60.0*1e9)
                                            - (minute*60.0*1e9)
                                            - (second*1e9);
                            res = Some((hour as usize, minute as usize, second as usize, nanosecond as usize));
                            break 'outer;
                        }
                    }
                }
            }
        }

        let (hour, minute, second, nanosecond) = res.unwrap();
        writeln!(out, "Case #{}: {} {} {} {}", tt+1, hour, minute, second, nanosecond).unwrap();
    }
}
