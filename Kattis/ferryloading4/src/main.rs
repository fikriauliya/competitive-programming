use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::VecDeque;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(PartialEq, Eq)]
enum Location {
    Left, Right 
}

fn opposite(location: Location) -> Location {
    match location {
        Location::Left => Location::Right,
        Location::Right => Location::Left,
    }
}
fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for _ in 0..t {
        let l = scan.token::<usize>().unwrap() * 100;
        let m = scan.token::<usize>().unwrap();
        let mut res = 0;
        let mut ferry = Location::Left;
        let mut left = VecDeque::new();
        let mut right = VecDeque::new();
        for _ in 0..m {
            let len = scan.token::<usize>().unwrap();
            let car = match &scan.token::<String>().unwrap()[..] {
                "left" => Location::Left,
                "right" => Location::Right,
                _ => panic!()
            };
            match car {
                Location::Left => left.push_back((len, car)),
                Location::Right => right.push_back((len, car)),
            };
        }
        let mut cap = 0;
        while !left.is_empty() || !right.is_empty() {
            let car = match ferry {
                Location::Left => left.front(),
                Location::Right => right.front(),
            };
            if let Some(car) = car {
                if cap + car.0 <= l {
                    cap += car.0;
                    let data = match ferry {
                        Location::Left => &mut left,
                        Location::Right => &mut right,
                    };
                    data.pop_front();
                } else {
                    ferry = opposite(ferry);
                    cap = 0;
                    res += 1;
                }
            } else {
                ferry = opposite(ferry);
                cap = 0;
                res += 1;
            }
        }
        if cap != 0 { res += 1 };
        println!("{}", res)
    }
}
