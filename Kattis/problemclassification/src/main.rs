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
    let n = scan.token::<usize>().unwrap();
    let mut category_word = HashMap::new();
    for _ in 0..n {
        let category = scan.token::<String>().unwrap();
        let w = scan.token::<usize>().unwrap();
        let mut words = Vec::new();
        for _ in 0..w {
            let word = scan.token::<String>().unwrap();
            words.push(word);
        }
        category_word.insert(category, words);
    }
    let mut word_count = HashMap::new();
    loop {
        let line = scan.line();
        if line.is_none() { break; }
        let line = line.unwrap();
        let words = line.split_ascii_whitespace();
        for word in words {
            let word = word.to_string();
            match word_count.get_mut(&word) {
                Some(c) => {*c += 1;},
                None => { word_count.insert(word, 1); },
            };
        }
    }
    let mut res = Vec::new();
    let mut max_score = 0;
    for (category, words) in category_word {
        let score = words.iter().map(|w| word_count.get(w).unwrap_or(&0)).sum();
        if score > max_score {
            res.clear();
            max_score = score;
            res.push(category);
        } else if score == max_score {
            res.push(category);
        }
    }
    res.sort();
    writeln!(out, "{}", res.join("\n"));

}
