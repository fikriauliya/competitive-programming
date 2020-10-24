use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashMap;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let m = scan.token::<usize>().unwrap();
    let s = scan.token::<usize>().unwrap();
    let mut v = vec![Vec::new(); m];
    for i in 0..m {
        for _ in 0..s {
            let val = scan.token::<usize>().unwrap();
            v[i].push(val);
        }
    }
    let mut counts: HashMap<usize, usize> = HashMap::new();
    let mut len = s;
    for i in 0..s {
        for j in 0..m {
            let k = v[j][i];
            match counts.get_mut(&k) {
                Some(v) => { *v = *v + 1; () },
                None => { counts.insert(k, 1); () }
            }
            if counts[&v[j][i]] == m {
                counts.remove(&k);
            }
        }
        if counts.is_empty() {
            len = i + 1;
            break;
        }
    }
    println!("{}", len);
    let res = &v[0][0..len];
    let mut res: Vec<usize> = res.iter().map(|v| *v).clone().collect();
    res.sort();
    let res: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("{}", res.join(" "));
}
