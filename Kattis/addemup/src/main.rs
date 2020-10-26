use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let sum = scan.token::<usize>().unwrap();
    let mut nums: HashMap<usize, HashSet<usize>>= HashMap::new();
    let blacklist = vec!['3', '4', '7'];
    for i in 0..n {
        let t = scan.token::<usize>().unwrap();
        let st = t.to_string();
        let mut ch = st.chars();
        if !ch.any(|it| blacklist.contains(&it)) {
            let rev= st.chars().rev();
            let rev: String = rev.map(|ch| match ch {
                '1'|'2'|'5'|'8'|'0' => ch,
                '6' => '9',
                '9' => '6',
                _ => panic!()
            }).collect();
            let k = rev.parse::<usize>().unwrap();
            match nums.get_mut(&k) {
                Some(v) => { v.insert(i); () },
                None => {
                    let mut v = HashSet::new();
                    v.insert(i);
                    nums.insert(k, v);
                    ()
                }
            }
        }
        let k = t;
        match nums.get_mut(&k) {
            Some(v) => { v.insert(i); () },
            None => {
                let mut v = HashSet::new();
                v.insert(i);
                nums.insert(k, v);
                ()
            }
        }
    }
    for (k, v1) in &nums {
        let rem = sum - k;
        if let Some(v2) = nums.get(&rem) { 
            if v2.len() > 1 || v1.len() > 1 {
                println!("YES"); return;
            } else {
                let has_diff = v2.difference(v1).into_iter().next().is_some();
                if has_diff { println!("YES"); return; }
            }
        }
    }
    println!("NO");
}
