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

enum Winner {
    One,
    Two,
    Error
}

enum Player {
    One,
    Two
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let p1 = scan.token::<String>().unwrap();
    let p2 = scan.token::<String>().unwrap();
    let n = scan.token::<u32>().unwrap();
    'case: for _ in 0..n {
        let line = scan.line().unwrap();
        let sets: Vec<&str> = line.split(" ").collect();
        let mut wins = vec![0;2];
        let mut game_winner = None;
        for (i, set) in sets.iter().enumerate() {
            if let Some(_) = game_winner {
                writeln!(out, "ne"); continue 'case;
            }
            let set = set.split(":").map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let winner = match (i, set[0], set[1]) {
                (_, a, b) if a.max(b) < 6 =>  Winner::Error,
                (_, 6, b) if b <= 4 => Winner::One,
                (_, a, 6) if a <= 4 => Winner::Two,
                (0, 7, 6) | (1, 7, 6) => Winner::One,
                (0, 6, 7) | (1, 6, 7) => Winner::Two,
                (0, a, b) if a > 7 || b > 7 => Winner::Error,
                (1, a, b) if a > 7 || b > 7 => Winner::Error,
                (_, a, b) if (a >= 6 || b >= 6) && a-b == 2 => Winner::One,
                (_, a, b) if (a >= 6 || b >= 6) && b-a == 2 => Winner::Two,
                _ => Winner::Error,
            };
            match (&p1.clone()[..], &p2.clone()[..], winner) {
                ("federer", _, Winner::Two) => {
                    writeln!(out, "ne"); continue 'case;
                }
                (_, "federer", Winner::One) => {
                    writeln!(out, "ne"); continue 'case;
                }
                (_, _, Winner::One) => wins[0]+=1,
                (_, _, Winner::Two) => wins[1]+=1,
                _ => { 
                    writeln!(out, "ne"); continue 'case; () 
                }
            }
            game_winner = match (wins[0], wins[1]) {
                (_, 2) => Some(Player::Two),
                (2, _) => Some(Player::One),
                _ => None
            }
        }
        if let None = game_winner {
            writeln!(out, "ne"); continue 'case;
        }
        writeln!(out, "da");
    }


}
