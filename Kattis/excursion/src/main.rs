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

fn mergesort(arr: &mut Vec<u8>, left: usize, right: usize) -> usize {
    if left >= right { return 0 };
    let half = (left + right) / 2;
    let mut res = 0;
    res += mergesort(arr, left, half);
    res += mergesort(arr, half+1, right);
    let mut l = left;
    let mut r = half+1;

    let mut merged = Vec::new();
    while l <= half && r <= right {
        if arr[l] <= arr[r] { 
            merged.push(arr[l]);
            l += 1; 
        } else {
            res += half - l + 1;
            merged.push(arr[r]);
            r += 1;
        }
    };
    while l <= half { merged.push(arr[l]); l += 1; }
    while r <= right { merged.push(arr[r]); r += 1; }
    let sliced = &mut arr[left..right+1];
    sliced.copy_from_slice(&merged);
    res
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let line = scan.line().unwrap();
    let chars = line.chars();
    let mut nums: Vec<u8> = chars.into_iter().map(|it| it as u8 - '0' as u8).collect();
    let len = nums.len();
    let res = mergesort(&mut nums, 0, len-1);
    println!("{}", res);
}
