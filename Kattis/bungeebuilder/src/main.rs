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

#[derive(Debug, Copy, Clone)]
struct Building {
    prev_lowest: Option<u32>,
    prev_height: Option<u32>,
    height: u32,
    pos: usize
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut hs = Vec::new();
    for _ in 0..n {
        let h = scan.token::<u32>().unwrap();
        hs.push(h);
    }

    let mut prevs: Vec<Building> = Vec::new();
    prevs.push(Building{
        pos: 0,
        height: hs[0],
        prev_lowest: None,
        prev_height: None,
    });

    let mut res = 0;
    for i in 1..n {
        // dbg!(&i);
        // dbg!(&hs[i]);
        let mut prev = prevs.last().cloned();
        let mut start = prev;
        // if let Some(prev) = prev {
        //     for j in prev.pos+1..i {
        //         lowest_from_prev = lowest_from_prev
        //             .map(|l| l.min(hs[j]))
        //             .or(Some(hs[j]));
        //     }
        //     let height_from_prev = lowest_from_prev
        //         .map(|l| prev.height.min(hs[i]) - l);
        //     dbg!(&lowest_from_prev);
        //     dbg!(&height_from_prev);
        //     res = res.max(height_from_prev.unwrap_or(0));
        //     dbg!(&res);
        // }

        let mut lowest_from_start: Option<u32> = None;
        while let Some(p) = prev {
            if p.height > hs[i] { break; }
            start = prevs.pop();
            // lowest_from_prev = 
            //     match (prev.prev_lowest, lowest_from_prev) {
            //     (Some(pl), Some(lv)) => Some(pl.min(lv)),
            //     (a, b) => a.or(b),
            // };
            // let height_from_prev = lowest_from_prev
            //     .and_then(|l| prev.prev_height.map(|ph| ph.min(hs[i]) - l));
            // dbg!(&lowest_from_prev);
            // dbg!(&height_from_prev);
            // res = res.max(height_from_prev.unwrap_or(0));
            prev = prevs.last().cloned();
            // dbg!(&res);
        }

        start = prev.or(start);
        if let Some(start) = start {
            for j in start.pos+1..i {
                lowest_from_start = lowest_from_start
                    .map(|l| l.min(hs[j]))
                    .or(Some(hs[j]));
            }
            let height_from_start = lowest_from_start
                .map(|l| start.height.min(hs[i]) - l);
            res = res.max(height_from_start.unwrap_or(0));
        }

        prevs.push(Building{
            pos: i, 
            prev_height: start.map(|l| l.height),
            prev_lowest: lowest_from_start,
            height: hs[i]
        });
        // dbg!(&prevs);
    }
    println!("{}", res);
}
