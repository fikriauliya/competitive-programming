use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct Pos (i32,i32);

#[derive(Eq, PartialEq, Clone, Debug)]
struct Distance {
    pos: Pos,
    cost: i32,
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
            .then_with(|| { other.pos.1.cmp(&self.pos.1) })
            .then_with(|| { other.pos.0.cmp(&self.pos.0) })
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

fn populate_deltas(m: &Vec<Vec<char>>, home: Pos) -> BinaryHeap<Reverse<Distance>> {
    let mut res = BinaryHeap::new();
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if '!' == m[i][j] {
                let cost = (home.1 - j as i32).pow(2) + (home.0 - i as i32).pow(2);
                let distance = Distance { pos: Pos(i as i32, j as i32), cost };
                res.push(Reverse(distance));
            }
        }
    }
    res
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    loop {
        let n = scan.token::<i32>().unwrap();
        let h = scan.token::<usize>().unwrap();
        let w = scan.token::<usize>().unwrap();
        if (0, 0, 0) == (n, h, w) { break; }

        let mut homes = vec![None; 26];
        let mut m = vec![vec![' '; w]; h];
        let player_count = 26;
        let mut scores = vec![None; 26];

        for i in 0..h {
            let line: Vec<char> = scan.line().unwrap().chars().collect();
            for j in 0..w {
                m[i][j] = line[j];
                if (('A' as u8)..('Z' as u8 + 1)).contains(&(line[j] as u8)) {
                    let index = line[j] as usize - 'A' as usize;
                    homes[index] = Some(Pos(i as i32, j as i32));
                    scores[index] = Some(0f64);
                }
            }
        }

        let mut next_pos = Vec::new();
        for i in 0..player_count {
            if let Some(home) = homes[i] {
                next_pos.push(Some(populate_deltas(&m, home)));
            } else {
                next_pos.push(None);
            }
        }

        for i in 0..n {
            let mut to_take = vec![None; 26];
            for p in 0..player_count {
                if let Some(mut np) = next_pos[p].take() {
                    loop {
                        let np = np.pop();
                        if let Some(np) = np {
                            if m[np.0.pos.0 as usize][np.0.pos.1 as usize] == '!' {
                                to_take[p] = Some(np.0.pos);
                                break;
                            }
                        } else { break; }
                    }
                    next_pos[p] = Some(np);
                }
            }
            for p in 0..player_count {
                match (&to_take[p], scores[p]) {
                    (Some(tt), Some(sc)) => { 
                        let competitor_count = to_take.iter().filter(|t| 
                                                  t.map_or_else(|| false, |t| t == *tt)
                                                  ).count() as f64;
                        scores[p] = Some(sc + 1f64 / competitor_count);
                        m[tt.0 as usize][tt.1 as usize] = '.';
                    },
                    _ => (),
                }
            }
        }
        
        for i in 0..h {
            for j in 0..w { print!("{}", m[i][j]); }
            println!();
        }
        for i in 0..player_count {
            if let Some(sc) = scores[i] {
                println!("{} {:.6}", ('A' as u8 + i as u8) as char, sc);
            }
        }
        println!();
    }
}
