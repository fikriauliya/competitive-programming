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
    let m: HashMap<&str, &str> = [("@", "at"), ("&", "and"), ("1", "one"), ("1", "won"), ("2", "to"), ("2", "too"), ("2", "two"), ("4", "for"), ("4", "four"), ("b", "bea"), ("b", "be"), ("b", "bee"), ("c", "sea"), ("c", "see"), ("i", "eye"), ("o", "oh"), ("o", "owe"), ("r", "are"), ("u", "you"), ("y", "why")]
        .iter().map(|v| (v.1, v.0)).collect();
    for _ in 0..n {
        let mut line = scan.line().unwrap();
        let mut res = "".to_string();
        let mut lowered = line.to_lowercase();

        while !line.is_empty() {
            let mut min_index = None;
            let mut key: Option<&str> = None;
            for k in m.keys() {
                let i = lowered.find(k);
                match (i, min_index) {
                    (Some(i), Some(mi)) => {
                        if i == mi { 
                            if k.len() > key.unwrap().len() { 
                                key = Some(k);
                            } 
                        }
                        else if i < mi {
                            min_index = Some(i);
                            key = Some(k);
                        }
                    },
                    (Some(_), None) => {
                        min_index = i;
                        key = Some(k);
                    },
                    _ => (),
                }
            }
            if let Some(key) = key{
                let mut sub = m[key].to_string();
                let index = min_index.unwrap();

                let chars:Vec<char> = line.chars().collect();
                if chars[index].is_uppercase() {
                    sub = sub.to_uppercase();
                }
                let mut prefix = (&line[..index]).to_string();
                prefix.push_str(&sub);
                res.push_str(&prefix);

                line = (&line[index+key.len()..]).to_string();
                lowered = line.to_lowercase();
            } else {
                res.push_str(&line);
                break;
            }
        }
        writeln!(out, "{}", res).ok();
    }
}
