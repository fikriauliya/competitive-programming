use std::io;
use std::str;
use std::convert::TryFrom;

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
    let mut words = Vec::new();
    let mut pos = 0;
    for i in 0..n {
        let word = scan.line().unwrap();
        if word == "$" {
            pos = i;
        } else {
            words.push(word);
        }
    }
    let len: usize = words.iter().map(|w| w.len()).sum();
    let mut numbers: Vec<String> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"
    ].iter().map(|n| n.to_string()).collect();
    let tens = vec![".", ".", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    for i in 20..100 {
        let mut number = tens[i/10].to_string();
        if i % 10 != 0 {
            number.push_str(&numbers[i%10]);
        }
        numbers.push(number);
    }
    let hundreds = vec![".", "onehundred", "twohundred", "threehundred", "fourhundred", "fivehundred", "sixhundred", "sevenhundred", "eighthundred", "ninehundred"];
    for i in 100..1000 {
        let mut number = hundreds[i/100].to_string();
        if i % 100 != 0 {
            number.push_str(&numbers[i%100]);
        }
        numbers.push(number);
    }
    for i in len..numbers.len() {
        let number = &numbers[i];
        if len + number.len() == i {
            words.insert(pos, number.to_string());
            println!("{}", words.join(" "));
            break;
        }
    }
}
