use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashSet;
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
    let (n, t) = (scan.token::<usize>().unwrap(), scan.token::<i32>().unwrap());
    let mut a = vec![];
    for _ in 0..n {
        let num = scan.token::<usize>().unwrap();
        a.push(num);
    }

    match t {
        1 => { writeln!(out, "7"); ()},
        2 => {
            if a[0] > a[1] {
                writeln!(out, "Bigger");
            }
            else if a[0] == a[1] {
                writeln!(out, "Equal");
            }
            else {
                writeln!(out, "Smaller");
            }
        },
        3 => {
            let mut first_threes = vec![0; 3];
            first_threes.copy_from_slice(&a[0..3]);
            first_threes.sort();

            writeln!(out, "{}", &first_threes[1]);
        },
        4 => {
            writeln!(out, "{}", a.iter().sum::<usize>());
        },
        5 => {
            writeln!(out, "{}", a.iter().filter(|a| *a % 2 == 0).sum::<usize>());
        },
        6 => {
            let a: Vec<usize> = a.iter().map(|a| *a % 26).collect();
            let a: Vec<String> = a.iter().map(|a| std::char::from_u32(*a as u32 + 'a' as u32).unwrap().to_string()).collect();
            writeln!(out, "{}", a.join(""));
        },
        7 => {
            let mut i = 0usize;
            let mut visited = HashSet::new();
            loop {
                i = a[i];
                if !(0..n).contains(&i)  {
                    writeln!(out, "Out");
                    break;
                } else if i == n-1 {
                    writeln!(out, "Done");
                    break;
                }
                if visited.contains(&i) {
                    writeln!(out, "Cyclic");
                    break;
                } else {
                    visited.insert(i);
                }
            }
        },
        _ => ()
    }
}
