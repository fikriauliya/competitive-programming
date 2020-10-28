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

fn clear(bits: &mut u32, knowns: &mut u32, bit: u32) {
    *bits = *bits & !(1u32 << bit);
    *knowns = *knowns | (1u32 << bit);
}

fn set(bits: &mut u32, knowns: &mut u32, bit: u32) {
    *bits = *bits | (1u32 << bit);
    *knowns = *knowns | (1u32 << bit);
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    loop {
        let n = scan.token::<usize>().unwrap();
        if n == 0 { break; }
        let mut bits = 0u32;
        let mut knowns = 0u32;

        for _ in 0..n {
            let lines = scan.line().unwrap();
            let mut lines = lines.split_ascii_whitespace();
            match lines.next().unwrap() {
                "CLEAR" => {
                    let i = lines.next().unwrap().parse::<u32>().unwrap();
                    clear(&mut bits, &mut knowns, i);
                },
                "SET" => {
                    let i = lines.next().unwrap().parse::<u32>().unwrap();
                    set(&mut bits, &mut knowns, i);
                },
                "OR" => {
                    let i = lines.next().unwrap().parse::<u32>().unwrap();
                    let j = lines.next().unwrap().parse::<u32>().unwrap();
                    let bit_i = if bits & (1u32 << i) > 0 { 1 } else { 0 };
                    let bit_j = if bits & (1u32 << j) > 0 { 1 } else { 0 };
                    let j_known = knowns & (1u32 << j) > 0;
                    match (bit_j, j_known) {
                        (1, true) => set(&mut bits, &mut knowns, i),
                        (_, false) if bit_i == 0 => knowns = knowns & !(1u32 << i),
                        _ => (),
                    }
                },
                "AND" => {
                    let i = lines.next().unwrap().parse::<u32>().unwrap();
                    let j = lines.next().unwrap().parse::<u32>().unwrap();
                    let bit_i = if bits & (1u32 << i) > 0 { 1 } else { 0 };
                    let bit_j = if bits & (1u32 << j) > 0 { 1 } else { 0 };
                    let j_known = knowns & (1u32 << j) > 0;
                    match (bit_j, j_known) {
                        (0, true) => clear(&mut bits, &mut knowns, i),
                        (_, false) if bit_i == 1 => knowns = knowns & !(1u32 << i),
                        _ => (),
                    }
                },
                _ => panic!()
            }
        }
        let mut i = 1 << 31;
        while i > 0 {
            if knowns & i > 0 {
                if bits & i > 0 { print!("1") }
                else { print!("0"); }
            } else {
                print!("?");
            }
            i = i >> 1;
        }
        println!();
    }
}
