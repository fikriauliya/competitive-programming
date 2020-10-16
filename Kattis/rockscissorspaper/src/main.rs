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
    let t = scan.token::<usize>().unwrap();
    let movements: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    for tt in 0..t {
        let r = scan.token::<usize>().unwrap();
        let c = scan.token::<usize>().unwrap();
        let d = scan.token::<usize>().unwrap();
        let mut m = vec![vec![' ';c];r];
        for rr in 0..r {
            let line: Vec<char> = scan.line().unwrap().chars().collect();
            for cc in 0..c {
                m[rr][cc] = line[cc];
            }
        }
        // dbg!(&m);

        for i in 0..d {
            let mut new_m = vec![vec![' ';c];r];
            for rr in 0..r {
                for cc in 0..c {
                    let sur_pos: Vec<(usize, usize)>= 
                        movements.iter()
                        .map(|m| (rr as i32 + m.0, cc as i32 + m.1))
                        .filter(|p| (0..r as i32).contains(&p.0) && (0..c as i32).contains(&p.1))
                        .map(|p| (p.0 as usize, p.1 as usize))
                        .collect();
                    let con_r = sur_pos.iter().any(|sp| m[sp.0][sp.1] == 'R');
                    let con_p = sur_pos.iter().any(|sp| m[sp.0][sp.1] == 'P');
                    let con_s = sur_pos.iter().any(|sp| m[sp.0][sp.1] == 'S');
                    new_m[rr][cc] = match m[rr][cc] {
                        'R' if con_p => 'P',
                        'P' if con_s => 'S',
                        'S' if con_r => 'R',
                        _ => m[rr][cc]
                    }
                }
            }
            m = new_m;
        }

        for rr in 0..r {
            for cc in 0..c {
                write!(out, "{}", m[rr][cc]);
            }
            writeln!(out, "");
        }
        if tt != t-1 {
            writeln!(out, "");
        }
    }
}
