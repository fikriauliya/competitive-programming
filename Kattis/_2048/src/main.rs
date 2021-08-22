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
    let mut m = vec![vec![0i32;4];4];
    for i in 0..4 {
        for j in 0..4 {
            m[i][j] = scan.token::<i32>().unwrap();
        }
    }
    let movement = scan.token::<i32>().unwrap();
    let mut mnew = vec![vec![0i32;4];4];
    match movement {
        0 => { for i in 0..4 { for j in 0..4 { mnew[i][j] = m[3-i][3-j] } } },  //left
        2 => { for i in 0..4 { for j in 0..4 { mnew[i][j] = m[i][j] } } },  //right
        1 => { for i in 0..4 { for j in 0..4 { mnew[i][j] = m[3-j][i] } } }, //up
        3 => { for i in 0..4 { for j in 0..4 { mnew[i][j] = m[j][3-i] } } }, //down
        _ => (),
    }
    let mut m = mnew;
    for i in 0..4 {
        for j in (0..4).rev() {
            if m[i][j] == 0 {
                let mut nj = j as i32 - 1;
                while (0..4).contains(&nj) {
                    if m[i][nj as usize] == 0 { 
                        nj -= 1;
                    } else {
                        m[i][j] = m[i][nj as usize];
                        m[i][nj as usize] = 0; 
                        break;
                    }
                }
            }
        }
    }
    for i in 0..4 {
        for j in (1..4).rev() {
            if m[i][j] == m[i][j-1] {
                m[i][j] = 2 * m[i][j];
                m[i][j-1] = 0;
            }
        }
    }
    for i in 0..4 {
        for j in (0..4).rev() {
            if m[i][j] == 0 {
                let mut nj = j as i32 - 1;
                while (0..4).contains(&nj) {
                    if m[i][nj as usize] == 0 { 
                        nj -= 1;
                    } else {
                        m[i][j] = m[i][nj as usize];
                        m[i][nj as usize] = 0; 
                        break;
                    }
                }
            }
        }
    }

    let mut mnew = vec![vec![0i32;4];4];
    match movement {
        0 => { for i in 0..4 { for j in 0..4 { mnew[i][j] = m[3-i][3-j] } } },  //left
        2 => { for i in 0..4 { for j in 0..4 { mnew[i][j] = m[i][j] } } },  //right
        1 => { for i in 0..4 { for j in 0..4 { mnew[i][j] = m[j][3-i] } } }, //up
        3 => { for i in 0..4 { for j in 0..4 { mnew[i][j] = m[3-j][i] } } }, //down
        _ => (),
    }
    let m = mnew;
    for i in 0..4 {
        let line: Vec<String> = m[i].iter().map(|item| item.to_string()).collect();
        println!("{}", line.join(" "));
    }
}
