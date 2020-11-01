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
    let delta = vec![(0, 2018), (1680, 1118), (1118, 1680), (2018, 0)];
    let mut plot: HashMap<(i32, i32), HashSet<usize>> = HashMap::new();
    let mut paired = HashSet::new();

    for i in 0..n {
        let x = scan.token::<i32>().unwrap();
        let y = scan.token::<i32>().unwrap();
        if let Some(pairs) = plot.get(&(x, y)) {
            // dbg!(i, pairs);
            for pair in pairs {
                if i < *pair {
                    paired.insert((i, *pair));
                } else {
                    paired.insert((*pair, i));
                }
            }
        }
        for d in &delta {
            for a in vec![-1, 1] {
                for b in vec![-1, 1] {
                    let (pair_x, pair_y) = (x + a * d.0, y + b * d.1);
                    match plot.get_mut(&(pair_x, pair_y)) {
                        Some(pairs) => {
                            pairs.insert(i); ()
                        },
                        None => {
                            let mut s = HashSet::new();
                            s.insert(i);
                            plot.insert((pair_x, pair_y), s);
                            ()
                        }
                    };
                }
            }
        }
    }
    // dbg!(&paired);

    println!("{}", paired.len());
}
